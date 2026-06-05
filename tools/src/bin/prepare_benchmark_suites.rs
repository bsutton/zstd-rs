use std::{
    collections::BTreeSet,
    env, fs, io,
    path::{Path, PathBuf},
};

use serde_json::{json, Value};
use zstd_rs_tools::{
    cross_block_repetition, ensure_clean_dir, has_flag, json_logs, parse_value, repeated_chunks,
    repeated_text, repo_root, write_fixture, xorshift,
};

struct ManifestEntry {
    fixture: String,
    bytes: u64,
    source: String,
    kind: String,
}

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if has_flag(&args, "--help") || has_flag(&args, "-h") {
        print_help();
        return Ok(());
    }
    let repo = repo_root();
    let benchmark_root = repo.join("benchmarks");
    let fixture_root = benchmark_root.join("fixtures");
    let manifest_root = benchmark_root.join("manifests");

    let suite = parse_value(&args, "--suite", "all");
    let output_root = PathBuf::from(parse_value(
        &args,
        "--output-root",
        fixture_root.display().to_string(),
    ));
    let manifest_root = PathBuf::from(parse_value(
        &args,
        "--manifest-root",
        manifest_root.display().to_string(),
    ));
    let zstd_repo = args
        .windows(2)
        .find_map(|window| (window[0] == "--zstd-repo").then(|| PathBuf::from(&window[1])));

    fs::create_dir_all(&output_root)?;
    fs::create_dir_all(&manifest_root)?;

    match suite.as_str() {
        "broad-local" => prepare_broad_local(&repo, &output_root, &manifest_root)?,
        "broad-c-zstd" => {
            let Some(zstd_repo) = zstd_repo else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "--zstd-repo is required for the broad-c-zstd suite",
                ));
            };
            prepare_broad_c_zstd(&output_root, &manifest_root, &zstd_repo)?;
        }
        "all" => {
            prepare_broad_local(&repo, &output_root, &manifest_root)?;
            if let Some(zstd_repo) = zstd_repo {
                prepare_broad_c_zstd(&output_root, &manifest_root, &zstd_repo)?;
            } else {
                println!("skipping broad-c-zstd: no --zstd-repo provided");
            }
        }
        other => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unsupported --suite {other}"),
            ));
        }
    }

    Ok(())
}

fn print_help() {
    println!(
        "Usage: prepare_benchmark_suites [--suite broad-local|broad-c-zstd|all] \\\n    [--output-root DIR] [--manifest-root DIR] [--zstd-repo DIR]\n\nOptions:\n  --suite NAME        Which suite to prepare.\n  --output-root DIR   Directory to write prepared fixture suites into.\n  --manifest-root DIR Directory to write suite manifests into.\n  --zstd-repo DIR     Local C zstd checkout for broad-c-zstd.\n  -h, --help          Show this help message."
    );
}

fn prepare_broad_local(repo: &Path, output_root: &Path, manifest_root: &Path) -> io::Result<()> {
    let suite_name = "broad-local";
    let suite_dir = output_root.join(suite_name);
    ensure_clean_dir(&suite_dir)?;
    fs::create_dir_all(manifest_root)?;
    let mut manifest = Vec::new();
    let mut used_names = BTreeSet::new();

    write_generated(
        repeated_text(1024 * 1024),
        &suite_dir,
        "generated_repeated_text_001m.txt",
        &mut manifest,
        "generated_text",
    )?;
    write_generated(
        json_logs(1024 * 1024),
        &suite_dir,
        "generated_json_logs_001m.jsonl",
        &mut manifest,
        "generated_json",
    )?;
    write_generated(
        cross_block_repetition(1024 * 1024),
        &suite_dir,
        "generated_cross_block_001m.bin",
        &mut manifest,
        "generated_binary",
    )?;
    write_generated(
        xorshift(0x00c0_ffee, 1024 * 1024),
        &suite_dir,
        "generated_xorshift_001m.bin",
        &mut manifest,
        "generated_incompressible",
    )?;

    for (name, data, kind) in generated_suite_fixtures() {
        write_generated(data, &suite_dir, name, &mut manifest, kind)?;
    }

    for source in local_source_fixtures(repo) {
        if source.is_file() {
            let name = unique_repo_fixture_name(repo, &source, &mut used_names);
            copy_fixture(&source, &suite_dir, &name, &mut manifest, "repo_source")?;
        }
    }

    for source in local_build_fixtures(repo) {
        if source.is_file() {
            let name = format!(
                "build_{}",
                source
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("artifact")
            );
            copy_fixture(&source, &suite_dir, &name, &mut manifest, "build_artifact")?;
        }
    }

    let dictionary = repo.join("ruzstd").join("dict_tests").join("dictionary");
    if dictionary.is_file() {
        copy_fixture(
            &dictionary,
            &suite_dir,
            "dict_dictionary.bin",
            &mut manifest,
            "dictionary",
        )?;
    }

    let dict_files_dir = repo.join("ruzstd").join("dict_tests").join("files");
    let dict_files = read_files_with_extension(&dict_files_dir, "service")?;
    for source in select_size_spread(dict_files, 14) {
        let name = format!(
            "dict_{}",
            source
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("file")
        );
        copy_fixture(&source, &suite_dir, &name, &mut manifest, "dictionary_text")?;
    }

    let decodecorpus_dir = repo.join("ruzstd").join("decodecorpus_files");
    let decodecorpus_files = fs::read_dir(&decodecorpus_dir)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path.extension().and_then(|extension| extension.to_str()) != Some("zst")
        })
        .collect::<Vec<_>>();
    for source in select_size_spread(decodecorpus_files, 12) {
        let name = format!(
            "decodecorpus_{}",
            source
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("file")
        );
        copy_fixture(&source, &suite_dir, &name, &mut manifest, "decodecorpus")?;
    }

    let manifest_path = manifest_root.join(format!("{suite_name}.json"));
    write_manifest(&manifest_path, &manifest)?;
    println!("{}\t{} fixtures", suite_dir.display(), manifest.len());
    println!("{}\tmanifest", manifest_path.display());
    Ok(())
}

fn prepare_broad_c_zstd(
    output_root: &Path,
    manifest_root: &Path,
    zstd_repo: &Path,
) -> io::Result<()> {
    let suite_name = "broad-c-zstd";
    let suite_dir = output_root.join(suite_name);
    ensure_clean_dir(&suite_dir)?;
    fs::create_dir_all(manifest_root)?;
    let mut manifest = Vec::new();
    let mut seen = BTreeSet::new();

    for pattern in [
        "README.md",
        "doc/*.md",
        "lib/common/*.[ch]",
        "lib/compress/*.[ch]",
        "lib/decompress/*.[ch]",
        "programs/*.[ch]",
        "tests/*.[ch]",
    ] {
        let matches = zstd_matches(zstd_repo, pattern)?;
        for source in select_size_spread(matches, 6) {
            let rel = source.strip_prefix(zstd_repo).unwrap_or(&source);
            let name = rel
                .components()
                .map(|component| component.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("_");
            if seen.insert(name.clone()) {
                copy_fixture(&source, &suite_dir, &name, &mut manifest, "zstd_repo")?;
            }
        }
    }

    let manifest_path = manifest_root.join(format!("{suite_name}.json"));
    write_manifest(&manifest_path, &manifest)?;
    println!("{}\t{} fixtures", suite_dir.display(), manifest.len());
    println!("{}\tmanifest", manifest_path.display());
    Ok(())
}

fn local_source_fixtures(repo: &Path) -> Vec<PathBuf> {
    [
        ".gitignore",
        "Cargo.toml",
        "Cargo.lock",
        ".github/workflows/ci.yml",
        "cli/Cargo.toml",
        "cli/src/main.rs",
        "cli/src/progress.rs",
        "ruzstd/Cargo.toml",
        "ruzstd/fuzz/.gitignore",
        "ruzstd/fuzz/Cargo.toml",
        "ruzstd/src/encoding/blocks/compressed.rs",
        "ruzstd/src/encoding/match_generator.rs",
        "tools/src/bin/benchmark_zstd.rs",
        "tools/src/bin/prepare_benchmark_suites.rs",
    ]
    .into_iter()
    .map(|path| repo.join(path))
    .collect()
}

fn local_build_fixtures(repo: &Path) -> Vec<PathBuf> {
    ["target/release/ruzstd-cli", "target/release/libruzstd.rlib"]
        .into_iter()
        .map(|path| repo.join(path))
        .collect()
}

fn generated_suite_fixtures() -> Vec<(&'static str, Vec<u8>, &'static str)> {
    vec![
        (
            "generated_yarn.lock",
            synthetic_yarn_lock(128 * 1024),
            "generated_lockfile",
        ),
        (
            "generated_poetry.lock",
            synthetic_poetry_lock(128 * 1024),
            "generated_lockfile",
        ),
        (
            "generated_go.sum",
            synthetic_go_sum(128 * 1024),
            "generated_lockfile",
        ),
        (
            "generated_requirements.txt",
            synthetic_requirements_txt(64 * 1024),
            "generated_config",
        ),
        (
            "generated_package-lock.json",
            synthetic_package_lock_json(128 * 1024),
            "generated_json_lockfile",
        ),
        (
            "generated_composer.lock",
            synthetic_composer_lock(128 * 1024),
            "generated_lockfile",
        ),
        (
            "generated_pipfile.lock",
            synthetic_pipfile_lock(128 * 1024),
            "generated_lockfile",
        ),
        (
            "generated_Gemfile",
            synthetic_gemfile(64 * 1024),
            "generated_config",
        ),
        (
            "generated_Gemfile.lock",
            synthetic_gemfile_lock(128 * 1024),
            "generated_lockfile",
        ),
        (
            "generated_go.mod",
            synthetic_go_mod(64 * 1024),
            "generated_config",
        ),
        (
            "generated_package.json",
            synthetic_package_json(64 * 1024),
            "generated_config",
        ),
        (
            "generated_turbo.json",
            synthetic_package_json(64 * 1024),
            "generated_config",
        ),
        (
            "generated_tsconfig.json",
            synthetic_tsconfig_json(64 * 1024),
            "generated_config",
        ),
        (
            "generated_deno.json",
            synthetic_tsconfig_json(64 * 1024),
            "generated_config",
        ),
        (
            "generated_nx.json",
            synthetic_tsconfig_json(64 * 1024),
            "generated_config",
        ),
        (
            "generated_pyproject.toml",
            synthetic_pyproject_toml(64 * 1024),
            "generated_config",
        ),
        (
            "generated_wrangler.toml",
            synthetic_pyproject_toml(64 * 1024),
            "generated_config",
        ),
        (
            "generated_pom.xml",
            synthetic_pom_xml(64 * 1024),
            "generated_config",
        ),
        (
            "generated_Dockerfile",
            synthetic_dockerfile(64 * 1024),
            "generated_config",
        ),
        (
            "generated_pubspec.yaml",
            synthetic_pubspec_yaml(64 * 1024),
            "generated_config",
        ),
        (
            "generated_buf.yaml",
            synthetic_pubspec_yaml(64 * 1024),
            "generated_config",
        ),
        (
            "generated_pubspec.lock",
            synthetic_pubspec_lock(128 * 1024),
            "generated_lockfile",
        ),
        (
            "generated_BUILD.bazel",
            synthetic_build_bazel(64 * 1024),
            "generated_code",
        ),
        (
            "generated_WORKSPACE",
            synthetic_workspace(64 * 1024),
            "generated_code",
        ),
    ]
}

fn write_generated(
    data: Vec<u8>,
    destination_dir: &Path,
    name: &str,
    manifest: &mut Vec<ManifestEntry>,
    kind: &str,
) -> io::Result<()> {
    let path = write_fixture(destination_dir, name, &data)?;
    manifest.push(ManifestEntry {
        fixture: name.to_string(),
        bytes: fs::metadata(path)?.len(),
        source: "generated".to_string(),
        kind: kind.to_string(),
    });
    Ok(())
}

fn copy_fixture(
    source: &Path,
    destination_dir: &Path,
    name: &str,
    manifest: &mut Vec<ManifestEntry>,
    kind: &str,
) -> io::Result<()> {
    fs::create_dir_all(destination_dir)?;
    let destination = destination_dir.join(name);
    fs::copy(source, &destination)?;
    manifest.push(ManifestEntry {
        fixture: name.to_string(),
        bytes: fs::metadata(&destination)?.len(),
        source: source.display().to_string(),
        kind: kind.to_string(),
    });
    Ok(())
}

fn unique_repo_fixture_name(
    repo: &Path,
    source: &Path,
    used_names: &mut BTreeSet<String>,
) -> String {
    let base = format!(
        "repo_{}",
        source
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file")
    );
    if used_names.insert(base.clone()) {
        return base;
    }

    let rel = source.strip_prefix(repo).unwrap_or(source);
    let mut candidate = format!(
        "repo_{}",
        rel.components()
            .map(|component| component.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("_")
    );
    if used_names.insert(candidate.clone()) {
        return candidate;
    }

    let stem = candidate.clone();
    let mut suffix = 2usize;
    loop {
        candidate = format!("{stem}_{suffix}");
        if used_names.insert(candidate.clone()) {
            return candidate;
        }
        suffix += 1;
    }
}

fn read_files_with_extension(dir: &Path, extension: &str) -> io::Result<Vec<PathBuf>> {
    Ok(fs::read_dir(dir)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext == extension)
        })
        .collect())
}

fn select_size_spread(mut files: Vec<PathBuf>, count: usize) -> Vec<PathBuf> {
    if files.len() <= count {
        files.sort();
        return files;
    }
    files.sort_by_key(|path| {
        (
            path.metadata().map(|metadata| metadata.len()).unwrap_or(0),
            path.file_name().map(|name| name.to_os_string()),
        )
    });
    quantile_indexes(files.len(), count)
        .into_iter()
        .map(|index| files[index].clone())
        .collect()
}

fn quantile_indexes(length: usize, count: usize) -> Vec<usize> {
    if length <= count {
        return (0..length).collect();
    }
    let mut indexes = Vec::new();
    for slot in 0..count {
        let value = (slot as f64 / (count - 1) as f64) * (length - 1) as f64;
        let index = round_half_even(value);
        if indexes.last().copied() != Some(index) {
            indexes.push(index);
        }
    }
    indexes
}

fn round_half_even(value: f64) -> usize {
    let floor = value.floor();
    let fraction = value - floor;
    if fraction < 0.5 {
        floor as usize
    } else if fraction > 0.5 {
        floor as usize + 1
    } else {
        let floor = floor as usize;
        if floor & 1 == 0 {
            floor
        } else {
            floor + 1
        }
    }
}

fn zstd_matches(zstd_repo: &Path, pattern: &str) -> io::Result<Vec<PathBuf>> {
    if !pattern.contains('*') {
        let path = zstd_repo.join(pattern);
        return Ok(path.is_file().then_some(path).into_iter().collect());
    }

    let (dir, suffixes): (&str, &[&str]) = if let Some(dir) = pattern.strip_suffix("/*.[ch]") {
        (dir, &["c", "h"])
    } else if let Some(dir) = pattern.strip_suffix("/*.md") {
        (dir, &["md"])
    } else {
        return Ok(Vec::new());
    };
    Ok(fs::read_dir(zstd_repo.join(dir))
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| suffixes.contains(&ext))
        })
        .collect())
}

fn write_manifest(path: &Path, entries: &[ManifestEntry]) -> io::Result<()> {
    let mut text = String::from("[\n");
    for (idx, entry) in entries.iter().enumerate() {
        let comma = if idx + 1 == entries.len() { "" } else { "," };
        text.push_str(&format!(
            "  {{\n    \"fixture\": \"{}\",\n    \"bytes\": {},\n    \"source\": \"{}\",\n    \"kind\": \"{}\"\n  }}{comma}\n",
            json_escape(&entry.fixture),
            entry.bytes,
            json_escape(&entry.source),
            json_escape(&entry.kind),
        ));
    }
    text.push_str("]\n");
    zstd_rs_tools::write_all(path, &text)
}

fn json_escape(value: &str) -> String {
    value
        .chars()
        .flat_map(|ch| match ch {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            other => vec![other],
        })
        .collect()
}

fn synthetic_yarn_lock(target_bytes: usize) -> Vec<u8> {
    let mut data = b"# THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.\n\n".to_vec();
    data.extend(repeated_chunks(
        &[
            "\"@babel/code-frame@^7.24.0\":\n  version \"7.24.2\"\n  resolved \"https://registry.yarnpkg.com/@babel/code-frame/-/code-frame-7.24.2.tgz\"\n  integrity sha512-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n  dependencies:\n    \"@babel/highlight\" \"^7.24.0\"\n    picocolors \"^1.0.0\"\n\n",
            "\"chalk@^5.3.0\", \"chalk@^5.4.1\":\n  version \"5.4.1\"\n  resolved \"https://registry.yarnpkg.com/chalk/-/chalk-5.4.1.tgz\"\n  integrity sha512-BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB\n  dependencies:\n    ansi-styles \"^6.2.1\"\n\n",
            "\"esbuild@^0.24.2\":\n  version \"0.24.2\"\n  resolved \"https://registry.yarnpkg.com/esbuild/-/esbuild-0.24.2.tgz\"\n  integrity sha512-CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\n  optionalDependencies:\n    \"@esbuild/linux-x64\" \"0.24.2\"\n    \"@esbuild/darwin-arm64\" \"0.24.2\"\n\n",
        ],
        target_bytes,
    ));
    data
}

fn synthetic_poetry_lock(target_bytes: usize) -> Vec<u8> {
    let mut data = b"[metadata]\nlock-version = \"2.0\"\npython-versions = \">=3.11,<4.0\"\ncontent-hash = \"5555555555555555555555555555555555555555555555555555555555555555\"\n\n".to_vec();
    data.extend(repeated_chunks(
        &[
            "[[package]]\nname = \"attrs\"\nversion = \"24.3.0\"\ndescription = \"Classes Without Boilerplate\"\noptional = false\npython-versions = \">=3.8\"\nfiles = [\n    {file = \"attrs-24.3.0-py3-none-any.whl\", hash = \"sha256:1111111111111111111111111111111111111111111111111111111111111111\"},\n    {file = \"attrs-24.3.0.tar.gz\", hash = \"sha256:2222222222222222222222222222222222222222222222222222222222222222\"},\n]\n\n",
            "[[package]]\nname = \"cffi\"\nversion = \"1.17.1\"\ndescription = \"Foreign Function Interface for Python calling C code.\"\noptional = false\npython-versions = \">=3.8\"\nfiles = [\n    {file = \"cffi-1.17.1-cp311-cp311-manylinux2014_x86_64.whl\", hash = \"sha256:3333333333333333333333333333333333333333333333333333333333333333\"},\n    {file = \"cffi-1.17.1.tar.gz\", hash = \"sha256:4444444444444444444444444444444444444444444444444444444444444444\"},\n]\n\n",
        ],
        target_bytes,
    ));
    data
}

fn synthetic_go_sum(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "cloud.google.com/go v0.118.0 h1:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa=\n",
            "cloud.google.com/go v0.118.0/go.mod h1:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb=\n",
            "github.com/google/go-cmp v0.6.0 h1:ccccccccccccccccccccccccccccccccccccccccccc=\n",
            "github.com/google/go-cmp v0.6.0/go.mod h1:ddddddddddddddddddddddddddddddddddddddddddd=\n",
            "golang.org/x/sys v0.29.0 h1:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee=\n",
            "golang.org/x/sys v0.29.0/go.mod h1:fffffffffffffffffffffffffffffffffffffffffff=\n",
        ],
        target_bytes,
    )
}

fn synthetic_requirements_txt(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "aiohttp==3.11.11 ; python_version >= \"3.10\"\n",
            "attrs==24.3.0 ; python_version >= \"3.8\"\n",
            "cryptography==44.0.0 ; platform_python_implementation != \"PyPy\"\n",
            "pydantic==2.10.4 ; python_version >= \"3.9\"\n",
            "uvicorn[standard]==0.34.0 ; python_version >= \"3.10\"\n",
            "watchfiles==1.0.4 ; sys_platform != \"win32\"\n",
        ],
        target_bytes,
    )
}

fn synthetic_package_lock_json(target_bytes: usize) -> Vec<u8> {
    let mut packages = serde_json::Map::new();
    packages.insert(
        String::new(),
        json!({
            "name": "generated-lockfile",
            "version": "1.0.0",
            "dependencies": {}
        }),
    );
    let mut document = json!({
        "name": "generated-lockfile",
        "version": "1.0.0",
        "lockfileVersion": 3,
        "requires": true,
        "packages": packages
    });
    let mut package_index = 0usize;
    while compact_json_len(&document) < target_bytes {
        let name = format!("node_modules/pkg-{package_index:04}");
        let dep_a = format!("pkg-{:04}", (package_index + 1) % 97);
        let dep_b = format!("pkg-{:04}", (package_index + 7) % 97);
        object_mut(&mut document, "packages").insert(
            name,
            json!({
                "version": format!("{}.{}.{}", 1 + (package_index % 3), package_index % 10, (package_index * 7) % 10),
                "resolved": format!("https://registry.npmjs.org/pkg-{package_index:04}/-/{package_index:04}.tgz"),
                "integrity": format!("sha512-{package_index:064}"),
                "license": "MIT",
                "dependencies": {
                    dep_a: "^1.0.0",
                    dep_b: "^2.0.0"
                }
            }),
        );
        package_index += 1;
    }
    pretty_json(&document)
}

fn synthetic_composer_lock(target_bytes: usize) -> Vec<u8> {
    let mut document = json!({
        "_readme": ["This is a generated composer.lock fixture."],
        "content-hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "packages": [],
        "packages-dev": []
    });
    let mut package_index = 0usize;
    while compact_json_len(&document) < target_bytes {
        let entry = json!({
            "name": format!("vendor/package-{package_index:04}"),
            "version": format!("{}.{}.{}", 1 + (package_index % 4), package_index % 10, (package_index * 3) % 10),
            "source": {
                "type": "git",
                "url": format!("https://example.com/vendor/package-{package_index:04}.git"),
                "reference": format!("{package_index:040}")
            },
            "require": {
                "php": ">=8.2",
                format!("vendor/dependency-{:04}", (package_index + 1) % 53): "^2.0"
            }
        });
        let key = if package_index.checked_rem(5) == Some(0) {
            "packages-dev"
        } else {
            "packages"
        };
        document
            .as_object_mut()
            .and_then(|root| root.get_mut(key))
            .and_then(Value::as_array_mut)
            .expect("composer package array")
            .push(entry);
        package_index += 1;
    }
    pretty_json(&document)
}

fn synthetic_pipfile_lock(target_bytes: usize) -> Vec<u8> {
    let mut document = json!({
        "_meta": {
            "hash": {"sha256": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"},
            "pipfile-spec": 6,
            "requires": {"python_version": "3.12"},
            "sources": [{"name": "pypi", "url": "https://pypi.org/simple", "verify_ssl": true}]
        },
        "default": {},
        "develop": {}
    });
    let mut package_index = 0usize;
    while compact_json_len(&document) < target_bytes {
        let target = if package_index.checked_rem(6) == Some(0) {
            "develop"
        } else {
            "default"
        };
        object_mut(&mut document, target).insert(
            format!("package-{package_index:04}"),
            json!({
                "version": format!("=={}.{}.{}", 1 + (package_index % 5), package_index % 10, (package_index * 9) % 10),
                "hashes": [
                    format!("sha256:{package_index:064}"),
                    format!("sha256:{:064}", package_index + 1)
                ],
                "markers": "python_version >= \"3.10\""
            }),
        );
        package_index += 1;
    }
    pretty_json(&document)
}

fn synthetic_gemfile(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "source \"https://rubygems.org\"\n",
            "ruby \"3.3.0\"\n\n",
            "gem \"rails\", \"~> 8.0.1\"\n",
            "gem \"puma\", \"~> 6.6\"\n",
            "gem \"redis\", \"~> 5.4\"\n",
            "group :development, :test do\n",
            "  gem \"rspec-rails\", \"~> 7.1\"\n",
            "  gem \"rubocop\", \"~> 1.72\"\n",
            "end\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_gemfile_lock(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "GEM\n  remote: https://rubygems.org/\n  specs:\n    actionpack (8.0.1)\n      activesupport (= 8.0.1)\n      nokogiri (>= 1.8.5)\n    activesupport (8.0.1)\n      benchmark (>= 0.3)\n      concurrent-ruby (~> 1.0, >= 1.3.1)\n    puma (6.6.0)\n\n",
            "DEPENDENCIES\n  puma (~> 6.6)\n  rails (~> 8.0.1)\n  redis (~> 5.4)\n\nBUNDLED WITH\n   2.6.2\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_go_mod(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "module example.com/generated/service\n\n",
            "go 1.23.4\n\n",
            "require (\n",
            "    github.com/gorilla/mux v1.8.1\n",
            "    github.com/redis/go-redis/v9 v9.7.0\n",
            "    golang.org/x/sync v0.10.0\n",
            ")\n\n",
            "replace github.com/example/internal => ../internal\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_package_json(target_bytes: usize) -> Vec<u8> {
    let mut document = json!({
        "name": "generated-package",
        "version": "1.0.0",
        "private": true,
        "scripts": {
            "build": "vite build",
            "dev": "vite",
            "test": "vitest run"
        },
        "dependencies": {},
        "devDependencies": {}
    });
    let mut package_index = 0usize;
    while compact_json_len(&document) < target_bytes {
        object_mut(&mut document, "dependencies").insert(
            format!("dep-{package_index:04}"),
            Value::String(format!("^{}.0.0", 1 + package_index % 5)),
        );
        object_mut(&mut document, "devDependencies").insert(
            format!("dev-dep-{package_index:04}"),
            Value::String(format!(
                "~{}.{package_index_mod}.0",
                1 + package_index % 3,
                package_index_mod = package_index % 10
            )),
        );
        package_index += 1;
    }
    pretty_json(&document)
}

fn synthetic_tsconfig_json(target_bytes: usize) -> Vec<u8> {
    let mut document = json!({
        "compilerOptions": {
            "target": "ES2022",
            "module": "NodeNext",
            "moduleResolution": "NodeNext",
            "strict": true,
            "noUncheckedIndexedAccess": true,
            "paths": {}
        },
        "include": ["src/**/*.ts", "src/**/*.tsx"],
        "exclude": ["dist", "node_modules"]
    });
    let mut path_index = 0usize;
    while compact_json_len(&document) < target_bytes {
        document
            .as_object_mut()
            .and_then(|root| root.get_mut("compilerOptions"))
            .and_then(Value::as_object_mut)
            .and_then(|compiler| compiler.get_mut("paths"))
            .and_then(Value::as_object_mut)
            .expect("tsconfig paths object")
            .insert(
                format!("@feature/{path_index:04}/*"),
                json!([format!("./src/feature_{path_index:04}/*")]),
            );
        path_index += 1;
    }
    pretty_json(&document)
}

fn synthetic_pyproject_toml(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "[build-system]\nrequires = [\"setuptools>=70\", \"wheel\"]\nbuild-backend = \"setuptools.build_meta\"\n\n",
            "[project]\nname = \"generated-project\"\nversion = \"1.0.0\"\nrequires-python = \">=3.11\"\ndependencies = [\n  \"httpx>=0.28.1\",\n  \"pydantic>=2.10.4\",\n]\n\n",
            "[tool.ruff]\nline-length = 100\ntarget-version = \"py311\"\n\n",
            "[tool.pytest.ini_options]\naddopts = \"-q\"\ntestpaths = [\"tests\"]\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_pom_xml(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &["<project>\n  <modelVersion>4.0.0</modelVersion>\n  <groupId>com.example.generated</groupId>\n  <artifactId>service</artifactId>\n  <version>1.0.0</version>\n  <dependencies>\n    <dependency>\n      <groupId>org.slf4j</groupId>\n      <artifactId>slf4j-api</artifactId>\n      <version>2.0.16</version>\n    </dependency>\n  </dependencies>\n</project>\n\n"],
        target_bytes,
    )
}

fn synthetic_dockerfile(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "FROM rust:1.88-bookworm AS build\n",
            "WORKDIR /app\n",
            "COPY Cargo.toml Cargo.lock ./\n",
            "COPY cli ./cli\n",
            "COPY ruzstd ./ruzstd\n",
            "RUN cargo build --release -p ruzstd-cli\n\n",
            "FROM debian:bookworm-slim\n",
            "COPY --from=build /app/target/release/ruzstd-cli /usr/local/bin/ruzstd-cli\n",
            "ENTRYPOINT [\"/usr/local/bin/ruzstd-cli\"]\n",
        ],
        target_bytes,
    )
}

fn synthetic_pubspec_yaml(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "name: generated_app\n",
            "description: \"Generated Flutter application fixture.\"\n",
            "publish_to: none\n",
            "environment:\n",
            "  sdk: \">=3.6.0 <4.0.0\"\n",
            "dependencies:\n",
            "  flutter:\n",
            "    sdk: flutter\n",
            "  riverpod: ^2.6.1\n",
            "  go_router: ^14.8.1\n",
            "dev_dependencies:\n",
            "  flutter_test:\n",
            "    sdk: flutter\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_pubspec_lock(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "packages:\n  async:\n    dependency: transitive\n    description:\n      name: async\n      sha256: \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\n      url: \"https://pub.dev\"\n    source: hosted\n    version: \"2.13.0\"\n  collection:\n    dependency: transitive\n    description:\n      name: collection\n      sha256: \"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\"\n      url: \"https://pub.dev\"\n    source: hosted\n    version: \"1.19.1\"\n\n",
            "sdks:\n  dart: \">=3.6.0 <4.0.0\"\n  flutter: \">=3.27.0\"\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_build_bazel(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "load(\"@rules_rust//rust:defs.bzl\", \"rust_binary\", \"rust_library\")\n\n",
            "package(default_visibility = [\"//visibility:public\"])\n\n",
            "rust_library(\n    name = \"core_lib\",\n    srcs = glob([\"src/**/*.rs\"]),\n    crate_root = \"src/lib.rs\",\n    deps = [\"@crate_index//:serde\"],\n)\n\n",
            "rust_binary(\n    name = \"tool\",\n    srcs = [\"src/main.rs\"],\n    deps = [\":core_lib\"],\n)\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_workspace(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "workspace(name = \"generated_workspace\")\n\n",
            "load(\"@bazel_tools//tools/build_defs/repo:http.bzl\", \"http_archive\")\n\n",
            "http_archive(\n    name = \"rules_rust\",\n    sha256 = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",\n    urls = [\"https://example.com/rules_rust.tar.gz\"],\n)\n\n",
            "register_toolchains(\"@rules_rust//rust:toolchain\")\n\n",
        ],
        target_bytes,
    )
}

fn compact_json_len(value: &Value) -> usize {
    serde_json::to_vec(value)
        .expect("synthetic benchmark JSON should serialize")
        .len()
}

fn pretty_json(value: &Value) -> Vec<u8> {
    serde_json::to_vec_pretty(value).expect("synthetic benchmark JSON should serialize")
}

fn object_mut<'a>(value: &'a mut Value, key: &str) -> &'a mut serde_json::Map<String, Value> {
    value
        .as_object_mut()
        .and_then(|root| root.get_mut(key))
        .and_then(Value::as_object_mut)
        .expect("synthetic JSON object")
}
