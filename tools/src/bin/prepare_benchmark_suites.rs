use std::{
    collections::BTreeSet,
    env, fs, io,
    path::{Path, PathBuf},
};

use zstd_rs_tools::{
    cross_block_repetition, ensure_clean_dir, json_logs, parse_value, repeated_chunks,
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
        .filter(|path| path.is_file() && path.extension().is_none())
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
        let index = ((slot as f64 / (count - 1) as f64) * (length - 1) as f64).round() as usize;
        if indexes.last().copied() != Some(index) {
            indexes.push(index);
        }
    }
    indexes
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
    repeated_chunks(
        &[
            "\"@babel/code-frame@^7.24.0\":\n  version \"7.24.2\"\n  resolved \"https://registry.yarnpkg.com/@babel/code-frame/-/code-frame-7.24.2.tgz\"\n  integrity sha512-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n  dependencies:\n    \"@babel/highlight\" \"^7.24.0\"\n    picocolors \"^1.0.0\"\n\n",
            "\"chalk@^5.3.0\", \"chalk@^5.4.1\":\n  version \"5.4.1\"\n  resolved \"https://registry.yarnpkg.com/chalk/-/chalk-5.4.1.tgz\"\n  integrity sha512-BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB\n  dependencies:\n    ansi-styles \"^6.2.1\"\n\n",
            "\"esbuild@^0.24.2\":\n  version \"0.24.2\"\n  resolved \"https://registry.yarnpkg.com/esbuild/-/esbuild-0.24.2.tgz\"\n  integrity sha512-CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\n  optionalDependencies:\n    \"@esbuild/linux-x64\" \"0.24.2\"\n    \"@esbuild/darwin-arm64\" \"0.24.2\"\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_poetry_lock(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "[[package]]\nname = \"attrs\"\nversion = \"24.3.0\"\ndescription = \"Classes Without Boilerplate\"\noptional = false\npython-versions = \">=3.8\"\nfiles = [\n    {file = \"attrs-24.3.0-py3-none-any.whl\", hash = \"sha256:1111111111111111111111111111111111111111111111111111111111111111\"},\n]\n\n",
            "[[package]]\nname = \"cffi\"\nversion = \"1.17.1\"\ndescription = \"Foreign Function Interface for Python calling C code.\"\noptional = false\npython-versions = \">=3.8\"\nfiles = [\n    {file = \"cffi-1.17.1.tar.gz\", hash = \"sha256:4444444444444444444444444444444444444444444444444444444444444444\"},\n]\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_go_sum(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "cloud.google.com/go v0.118.0 h1:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa=\n",
            "cloud.google.com/go v0.118.0/go.mod h1:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb=\n",
            "github.com/google/go-cmp v0.6.0 h1:ccccccccccccccccccccccccccccccccccccccccccc=\n",
            "github.com/google/go-cmp v0.6.0/go.mod h1:ddddddddddddddddddddddddddddddddddddddddddd=\n",
            "golang.org/x/sys v0.29.0 h1:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee=\n",
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
        ],
        target_bytes,
    )
}

fn synthetic_package_lock_json(target_bytes: usize) -> Vec<u8> {
    synthetic_json_packages("package-lock", target_bytes)
}

fn synthetic_composer_lock(target_bytes: usize) -> Vec<u8> {
    synthetic_json_packages("composer", target_bytes)
}

fn synthetic_pipfile_lock(target_bytes: usize) -> Vec<u8> {
    synthetic_json_packages("pipfile", target_bytes)
}

fn synthetic_json_packages(kind: &str, target_bytes: usize) -> Vec<u8> {
    let mut text = format!("{{\n  \"name\": \"generated-{kind}\",\n  \"packages\": {{\n");
    let mut idx = 0usize;
    while text.len() < target_bytes {
        text.push_str(&format!(
            "    \"pkg-{idx:04}\": {{ \"version\": \"{}.{}.\", \"resolved\": \"https://example.com/pkg-{idx:04}.tgz\", \"integrity\": \"sha512-{idx:064}\" }},\n",
            1 + (idx % 5),
            idx % 10,
        ));
        idx += 1;
    }
    text.push_str("    \"tail\": {}\n  }\n}\n");
    text.into_bytes()
}

fn synthetic_gemfile(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "source \"https://rubygems.org\"\nruby \"3.3.0\"\n\ngem \"rails\", \"~> 8.0.1\"\ngem \"puma\", \"~> 6.6\"\ngroup :development, :test do\n  gem \"rspec-rails\", \"~> 7.1\"\nend\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_gemfile_lock(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "GEM\n  remote: https://rubygems.org/\n  specs:\n    actionpack (8.0.1)\n      activesupport (= 8.0.1)\n    puma (6.6.0)\n\n",
            "DEPENDENCIES\n  puma (~> 6.6)\n  rails (~> 8.0.1)\n\nBUNDLED WITH\n   2.6.2\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_go_mod(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &["module example.com/generated/service\n\ngo 1.23.4\n\nrequire (\n    github.com/gorilla/mux v1.8.1\n    github.com/redis/go-redis/v9 v9.7.0\n)\n\n"],
        target_bytes,
    )
}

fn synthetic_package_json(target_bytes: usize) -> Vec<u8> {
    synthetic_json_packages("package", target_bytes)
}

fn synthetic_tsconfig_json(target_bytes: usize) -> Vec<u8> {
    let mut text =
        "{\"compilerOptions\":{\"target\":\"ES2022\",\"module\":\"NodeNext\",\"paths\":{"
            .to_string();
    let mut idx = 0usize;
    while text.len() < target_bytes {
        text.push_str(&format!(
            "\"@feature/{idx:04}/*\":[\"./src/feature_{idx:04}/*\"],"
        ));
        idx += 1;
    }
    text.push_str("\"tail\":[]}},\"include\":[\"src/**/*.ts\"]}\n");
    text.into_bytes()
}

fn synthetic_pyproject_toml(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &[
            "[build-system]\nrequires = [\"setuptools>=70\", \"wheel\"]\nbuild-backend = \"setuptools.build_meta\"\n\n",
            "[project]\nname = \"generated-project\"\nversion = \"1.0.0\"\nrequires-python = \">=3.11\"\n\n",
            "[tool.ruff]\nline-length = 100\ntarget-version = \"py311\"\n\n",
        ],
        target_bytes,
    )
}

fn synthetic_pom_xml(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &["<project>\n  <modelVersion>4.0.0</modelVersion>\n  <groupId>com.example.generated</groupId>\n  <artifactId>service</artifactId>\n  <version>1.0.0</version>\n</project>\n\n"],
        target_bytes,
    )
}

fn synthetic_dockerfile(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &["FROM rust:1.88-bookworm AS build\nWORKDIR /app\nCOPY Cargo.toml Cargo.lock ./\nRUN cargo build --release -p ruzstd-cli\n\nFROM debian:bookworm-slim\nCOPY --from=build /app/target/release/ruzstd-cli /usr/local/bin/ruzstd-cli\n"],
        target_bytes,
    )
}

fn synthetic_pubspec_yaml(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &["name: generated_app\ndescription: \"Generated Flutter application fixture.\"\npublish_to: none\nenvironment:\n  sdk: \">=3.6.0 <4.0.0\"\ndependencies:\n  flutter:\n    sdk: flutter\n  riverpod: ^2.6.1\n\n"],
        target_bytes,
    )
}

fn synthetic_pubspec_lock(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &["packages:\n  async:\n    dependency: transitive\n    description:\n      name: async\n      sha256: \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\n      url: \"https://pub.dev\"\n    source: hosted\n    version: \"2.13.0\"\n\nsdks:\n  dart: \">=3.6.0 <4.0.0\"\n"],
        target_bytes,
    )
}

fn synthetic_build_bazel(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &["load(\"@rules_rust//rust:defs.bzl\", \"rust_binary\", \"rust_library\")\n\npackage(default_visibility = [\"//visibility:public\"])\n\nrust_library(\n    name = \"core_lib\",\n    srcs = glob([\"src/**/*.rs\"]),\n)\n\n"],
        target_bytes,
    )
}

fn synthetic_workspace(target_bytes: usize) -> Vec<u8> {
    repeated_chunks(
        &["workspace(name = \"generated_workspace\")\n\nload(\"@bazel_tools//tools/build_defs/repo:http.bzl\", \"http_archive\")\n\nhttp_archive(\n    name = \"rules_rust\",\n    urls = [\"https://example.com/rules_rust.tar.gz\"],\n)\n\n"],
        target_bytes,
    )
}
