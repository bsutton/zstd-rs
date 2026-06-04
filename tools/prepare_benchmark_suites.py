#!/usr/bin/env python3
"""Prepare reproducible benchmark fixture suites inside the repo."""

import argparse
import json
import math
import shutil
from pathlib import Path

from generate_zstd_fixtures import (
    cross_block_repetition,
    json_logs,
    repeated_text,
    xorshift,
)


REPO_ROOT = Path(__file__).resolve().parent.parent
BENCHMARK_ROOT = REPO_ROOT / "benchmarks"
FIXTURE_ROOT = BENCHMARK_ROOT / "fixtures"
MANIFEST_ROOT = BENCHMARK_ROOT / "manifests"

LOCAL_SOURCE_FIXTURES = [
    REPO_ROOT / ".gitignore",
    REPO_ROOT / "Cargo.toml",
    REPO_ROOT / "Cargo.lock",
    REPO_ROOT / ".github" / "workflows" / "ci.yml",
    REPO_ROOT / "cli" / "Cargo.toml",
    REPO_ROOT / "cli" / "src" / "main.rs",
    REPO_ROOT / "cli" / "src" / "progress.rs",
    REPO_ROOT / "ruzstd" / "Cargo.toml",
    REPO_ROOT / "ruzstd" / "fuzz" / ".gitignore",
    REPO_ROOT / "ruzstd" / "fuzz" / "Cargo.toml",
    REPO_ROOT / "ruzstd" / "src" / "encoding" / "blocks" / "compressed.rs",
    REPO_ROOT / "ruzstd" / "src" / "encoding" / "match_generator.rs",
    REPO_ROOT / "tools" / "benchmark_zstd.py",
    REPO_ROOT / "tools" / "prepare_benchmark_suites.py",
]

LOCAL_BUILD_FIXTURES = [
    REPO_ROOT / "target" / "release" / "ruzstd-cli",
    REPO_ROOT / "target" / "release" / "libruzstd.rlib",
]

ZSTD_REPO_PATTERNS = [
    "README.md",
    "doc/*.md",
    "lib/common/*.[ch]",
    "lib/compress/*.[ch]",
    "lib/decompress/*.[ch]",
    "programs/*.[ch]",
    "tests/*.[ch]",
]


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--suite",
        choices=["broad-local", "broad-c-zstd", "all"],
        default="all",
        help="Which suite to prepare.",
    )
    parser.add_argument(
        "--output-root",
        type=Path,
        default=FIXTURE_ROOT,
        help="Directory to write prepared fixture suites into.",
    )
    parser.add_argument(
        "--manifest-root",
        type=Path,
        default=MANIFEST_ROOT,
        help="Directory to write suite manifests into.",
    )
    parser.add_argument(
        "--zstd-repo",
        type=Path,
        default=None,
        help="Optional path to a local C zstd checkout for the broad-c-zstd suite.",
    )
    return parser.parse_args()


def ensure_clean_dir(path: Path):
    path.mkdir(parents=True, exist_ok=True)
    for child in path.iterdir():
        if child.is_file() or child.is_symlink():
            child.unlink()
        else:
            shutil.rmtree(child)


def quantile_indexes(length: int, count: int) -> list[int]:
    if length <= count:
        return list(range(length))

    indexes = []
    for slot in range(count):
        fraction = slot / (count - 1)
        index = round(fraction * (length - 1))
        if not indexes or indexes[-1] != index:
            indexes.append(index)
    return indexes


def select_size_spread(files: list[Path], count: int) -> list[Path]:
    sized = sorted(files, key=lambda path: (path.stat().st_size, path.name))
    return [sized[index] for index in quantile_indexes(len(sized), count)]


def copy_fixture(source: Path, destination_dir: Path, name: str, manifest: list[dict], kind: str):
    destination = destination_dir / name
    shutil.copy2(source, destination)
    manifest.append(
        {
            "fixture": name,
            "bytes": destination.stat().st_size,
            "source": str(source),
            "kind": kind,
        }
    )


def unique_repo_fixture_name(source: Path, used_names: set[str]) -> str:
    base_name = f"repo_{source.name}"
    if base_name not in used_names:
        used_names.add(base_name)
        return base_name

    relative = source.relative_to(REPO_ROOT)
    candidate = "repo_" + "_".join(relative.parts)
    if candidate not in used_names:
        used_names.add(candidate)
        return candidate

    stem = candidate
    suffix = 2
    while True:
        candidate = f"{stem}_{suffix}"
        if candidate not in used_names:
            used_names.add(candidate)
            return candidate
        suffix += 1


def write_generated_fixture(data: bytes, destination_dir: Path, name: str, manifest: list[dict], kind: str):
    destination = destination_dir / name
    destination.write_bytes(data)
    manifest.append(
        {
            "fixture": name,
            "bytes": len(data),
            "source": "generated",
            "kind": kind,
        }
    )


def repeated_chunks(chunks: list[str], target_bytes: int) -> bytes:
    text = ""
    index = 0
    while len(text.encode("utf-8")) < target_bytes:
        text += chunks[index % len(chunks)]
        index += 1
    return text.encode("utf-8")


def synthetic_yarn_lock(target_bytes: int) -> bytes:
    stanzas = [
        '"@babel/code-frame@^7.24.0":\n'
        '  version "7.24.2"\n'
        '  resolved "https://registry.yarnpkg.com/@babel/code-frame/-/code-frame-7.24.2.tgz"\n'
        '  integrity sha512-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n'
        '  dependencies:\n'
        '    "@babel/highlight" "^7.24.0"\n'
        '    picocolors "^1.0.0"\n\n',
        '"chalk@^5.3.0", "chalk@^5.4.1":\n'
        '  version "5.4.1"\n'
        '  resolved "https://registry.yarnpkg.com/chalk/-/chalk-5.4.1.tgz"\n'
        '  integrity sha512-BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB\n'
        '  dependencies:\n'
        '    ansi-styles "^6.2.1"\n\n',
        '"esbuild@^0.24.2":\n'
        '  version "0.24.2"\n'
        '  resolved "https://registry.yarnpkg.com/esbuild/-/esbuild-0.24.2.tgz"\n'
        '  integrity sha512-CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\n'
        '  optionalDependencies:\n'
        '    "@esbuild/linux-x64" "0.24.2"\n'
        '    "@esbuild/darwin-arm64" "0.24.2"\n\n',
    ]
    header = "# THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.\n\n"
    return (header.encode("utf-8") + repeated_chunks(stanzas, target_bytes))


def synthetic_poetry_lock(target_bytes: int) -> bytes:
    stanzas = [
        "[[package]]\n"
        'name = "attrs"\n'
        'version = "24.3.0"\n'
        'description = "Classes Without Boilerplate"\n'
        'optional = false\n'
        'python-versions = ">=3.8"\n'
        'files = [\n'
        '    {file = "attrs-24.3.0-py3-none-any.whl", hash = "sha256:1111111111111111111111111111111111111111111111111111111111111111"},\n'
        '    {file = "attrs-24.3.0.tar.gz", hash = "sha256:2222222222222222222222222222222222222222222222222222222222222222"},\n'
        "]\n\n",
        "[[package]]\n"
        'name = "cffi"\n'
        'version = "1.17.1"\n'
        'description = "Foreign Function Interface for Python calling C code."\n'
        'optional = false\n'
        'python-versions = ">=3.8"\n'
        'files = [\n'
        '    {file = "cffi-1.17.1-cp311-cp311-manylinux2014_x86_64.whl", hash = "sha256:3333333333333333333333333333333333333333333333333333333333333333"},\n'
        '    {file = "cffi-1.17.1.tar.gz", hash = "sha256:4444444444444444444444444444444444444444444444444444444444444444"},\n'
        "]\n\n",
    ]
    metadata = (
        "[metadata]\n"
        'lock-version = "2.0"\n'
        'python-versions = ">=3.11,<4.0"\n'
        'content-hash = "5555555555555555555555555555555555555555555555555555555555555555"\n\n'
    )
    return (metadata.encode("utf-8") + repeated_chunks(stanzas, target_bytes))


def synthetic_go_sum(target_bytes: int) -> bytes:
    lines = [
        "cloud.google.com/go v0.118.0 h1:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa=\n",
        "cloud.google.com/go v0.118.0/go.mod h1:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb=\n",
        "github.com/google/go-cmp v0.6.0 h1:ccccccccccccccccccccccccccccccccccccccccccc=\n",
        "github.com/google/go-cmp v0.6.0/go.mod h1:ddddddddddddddddddddddddddddddddddddddddddd=\n",
        "golang.org/x/sys v0.29.0 h1:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee=\n",
        "golang.org/x/sys v0.29.0/go.mod h1:fffffffffffffffffffffffffffffffffffffffffff=\n",
    ]
    return repeated_chunks(lines, target_bytes)


def synthetic_requirements_txt(target_bytes: int) -> bytes:
    lines = [
        'aiohttp==3.11.11 ; python_version >= "3.10"\n',
        'attrs==24.3.0 ; python_version >= "3.8"\n',
        'cryptography==44.0.0 ; platform_python_implementation != "PyPy"\n',
        'pydantic==2.10.4 ; python_version >= "3.9"\n',
        'uvicorn[standard]==0.34.0 ; python_version >= "3.10"\n',
        'watchfiles==1.0.4 ; sys_platform != "win32"\n',
    ]
    return repeated_chunks(lines, target_bytes)


def synthetic_package_lock_json(target_bytes: int) -> bytes:
    document = {
        "name": "generated-lockfile",
        "version": "1.0.0",
        "lockfileVersion": 3,
        "requires": True,
        "packages": {
            "": {
                "name": "generated-lockfile",
                "version": "1.0.0",
                "dependencies": {},
            }
        },
    }
    package_index = 0
    while len(json.dumps(document, separators=(",", ":")).encode("utf-8")) < target_bytes:
        name = f"node_modules/pkg-{package_index:04d}"
        dep_a = f"pkg-{(package_index + 1) % 97:04d}"
        dep_b = f"pkg-{(package_index + 7) % 97:04d}"
        document["packages"][name] = {
            "version": f"{1 + (package_index % 3)}.{package_index % 10}.{(package_index * 7) % 10}",
            "resolved": f"https://registry.npmjs.org/pkg-{package_index:04d}/-/{package_index:04d}.tgz",
            "integrity": f"sha512-{package_index:064d}",
            "license": "MIT",
            "dependencies": {dep_a: "^1.0.0", dep_b: "^2.0.0"},
        }
        package_index += 1
    return json.dumps(document, indent=2, sort_keys=True).encode("utf-8")


def synthetic_composer_lock(target_bytes: int) -> bytes:
    document = {
        "_readme": ["This is a generated composer.lock fixture."],
        "content-hash": "a" * 64,
        "packages": [],
        "packages-dev": [],
    }
    package_index = 0
    while len(json.dumps(document, separators=(",", ":")).encode("utf-8")) < target_bytes:
        entry = {
            "name": f"vendor/package-{package_index:04d}",
            "version": f"{1 + (package_index % 4)}.{package_index % 10}.{(package_index * 3) % 10}",
            "source": {
                "type": "git",
                "url": f"https://example.com/vendor/package-{package_index:04d}.git",
                "reference": f"{package_index:040d}",
            },
            "require": {
                "php": ">=8.2",
                f"vendor/dependency-{(package_index + 1) % 53:04d}": "^2.0",
            },
        }
        if package_index % 5 == 0:
            document["packages-dev"].append(entry)
        else:
            document["packages"].append(entry)
        package_index += 1
    return json.dumps(document, indent=2, sort_keys=True).encode("utf-8")


def synthetic_pipfile_lock(target_bytes: int) -> bytes:
    document = {
        "_meta": {
            "hash": {"sha256": "b" * 64},
            "pipfile-spec": 6,
            "requires": {"python_version": "3.12"},
            "sources": [{"name": "pypi", "url": "https://pypi.org/simple", "verify_ssl": True}],
        },
        "default": {},
        "develop": {},
    }
    package_index = 0
    while len(json.dumps(document, separators=(",", ":")).encode("utf-8")) < target_bytes:
        target = document["develop" if package_index % 6 == 0 else "default"]
        target[f"package-{package_index:04d}"] = {
            "version": f"=={1 + (package_index % 5)}.{package_index % 10}.{(package_index * 9) % 10}",
            "hashes": [
                f"sha256:{package_index:064d}",
                f"sha256:{package_index + 1:064d}",
            ],
            "markers": 'python_version >= "3.10"',
        }
        package_index += 1
    return json.dumps(document, indent=2, sort_keys=True).encode("utf-8")


def synthetic_gemfile(target_bytes: int) -> bytes:
    lines = [
        'source "https://rubygems.org"\n',
        'ruby "3.3.0"\n\n',
        'gem "rails", "~> 8.0.1"\n',
        'gem "puma", "~> 6.6"\n',
        'gem "redis", "~> 5.4"\n',
        'group :development, :test do\n',
        '  gem "rspec-rails", "~> 7.1"\n',
        '  gem "rubocop", "~> 1.72"\n',
        "end\n\n",
    ]
    return repeated_chunks(lines, target_bytes)


def synthetic_gemfile_lock(target_bytes: int) -> bytes:
    sections = [
        "GEM\n"
        "  remote: https://rubygems.org/\n"
        "  specs:\n"
        "    actionpack (8.0.1)\n"
        "      activesupport (= 8.0.1)\n"
        "      nokogiri (>= 1.8.5)\n"
        "    activesupport (8.0.1)\n"
        "      benchmark (>= 0.3)\n"
        "      concurrent-ruby (~> 1.0, >= 1.3.1)\n"
        "    puma (6.6.0)\n\n",
        "DEPENDENCIES\n"
        "  puma (~> 6.6)\n"
        "  rails (~> 8.0.1)\n"
        "  redis (~> 5.4)\n\n"
        "BUNDLED WITH\n"
        "   2.6.2\n\n",
    ]
    return repeated_chunks(sections, target_bytes)


def synthetic_go_mod(target_bytes: int) -> bytes:
    lines = [
        "module example.com/generated/service\n\n",
        "go 1.23.4\n\n",
        "require (\n",
        "    github.com/gorilla/mux v1.8.1\n",
        "    github.com/redis/go-redis/v9 v9.7.0\n",
        "    golang.org/x/sync v0.10.0\n",
        ")\n\n",
        "replace github.com/example/internal => ../internal\n\n",
    ]
    return repeated_chunks(lines, target_bytes)


def synthetic_package_json(target_bytes: int) -> bytes:
    document = {
        "name": "generated-package",
        "version": "1.0.0",
        "private": True,
        "scripts": {
            "build": "vite build",
            "dev": "vite",
            "test": "vitest run",
        },
        "dependencies": {},
        "devDependencies": {},
    }
    package_index = 0
    while len(json.dumps(document, separators=(",", ":")).encode("utf-8")) < target_bytes:
        document["dependencies"][f"dep-{package_index:04d}"] = f"^{1 + package_index % 5}.0.0"
        document["devDependencies"][f"dev-dep-{package_index:04d}"] = (
            f"~{1 + package_index % 3}.{package_index % 10}.0"
        )
        package_index += 1
    return json.dumps(document, indent=2, sort_keys=True).encode("utf-8")


def synthetic_tsconfig_json(target_bytes: int) -> bytes:
    document = {
        "compilerOptions": {
            "target": "ES2022",
            "module": "NodeNext",
            "moduleResolution": "NodeNext",
            "strict": True,
            "noUncheckedIndexedAccess": True,
            "paths": {},
        },
        "include": ["src/**/*.ts", "src/**/*.tsx"],
        "exclude": ["dist", "node_modules"],
    }
    path_index = 0
    while len(json.dumps(document, separators=(",", ":")).encode("utf-8")) < target_bytes:
        document["compilerOptions"]["paths"][f"@feature/{path_index:04d}/*"] = [
            f"./src/feature_{path_index:04d}/*"
        ]
        path_index += 1
    return json.dumps(document, indent=2, sort_keys=True).encode("utf-8")


def synthetic_pyproject_toml(target_bytes: int) -> bytes:
    sections = [
        "[build-system]\n"
        'requires = ["setuptools>=70", "wheel"]\n'
        'build-backend = "setuptools.build_meta"\n\n',
        "[project]\n"
        'name = "generated-project"\n'
        'version = "1.0.0"\n'
        'requires-python = ">=3.11"\n'
        'dependencies = [\n'
        '  "httpx>=0.28.1",\n'
        '  "pydantic>=2.10.4",\n'
        "]\n\n",
        "[tool.ruff]\n"
        'line-length = 100\n'
        'target-version = "py311"\n\n',
        "[tool.pytest.ini_options]\n"
        'addopts = "-q"\n'
        'testpaths = ["tests"]\n\n',
    ]
    return repeated_chunks(sections, target_bytes)


def synthetic_pom_xml(target_bytes: int) -> bytes:
    blocks = [
        "<project>\n"
        "  <modelVersion>4.0.0</modelVersion>\n"
        "  <groupId>com.example.generated</groupId>\n"
        "  <artifactId>service</artifactId>\n"
        "  <version>1.0.0</version>\n"
        "  <dependencies>\n"
        "    <dependency>\n"
        "      <groupId>org.slf4j</groupId>\n"
        "      <artifactId>slf4j-api</artifactId>\n"
        "      <version>2.0.16</version>\n"
        "    </dependency>\n"
        "  </dependencies>\n"
        "</project>\n\n"
    ]
    return repeated_chunks(blocks, target_bytes)


def synthetic_dockerfile(target_bytes: int) -> bytes:
    lines = [
        "FROM rust:1.88-bookworm AS build\n",
        "WORKDIR /app\n",
        "COPY Cargo.toml Cargo.lock ./\n",
        "COPY cli ./cli\n",
        "COPY ruzstd ./ruzstd\n",
        "RUN cargo build --release -p ruzstd-cli\n\n",
        "FROM debian:bookworm-slim\n",
        "COPY --from=build /app/target/release/ruzstd-cli /usr/local/bin/ruzstd-cli\n",
        'ENTRYPOINT ["/usr/local/bin/ruzstd-cli"]\n',
    ]
    return repeated_chunks(lines, target_bytes)


def synthetic_pubspec_yaml(target_bytes: int) -> bytes:
    lines = [
        "name: generated_app\n",
        'description: "Generated Flutter application fixture."\n',
        "publish_to: none\n",
        "environment:\n",
        '  sdk: ">=3.6.0 <4.0.0"\n',
        "dependencies:\n",
        "  flutter:\n",
        "    sdk: flutter\n",
        "  riverpod: ^2.6.1\n",
        "  go_router: ^14.8.1\n",
        "dev_dependencies:\n",
        "  flutter_test:\n",
        "    sdk: flutter\n\n",
    ]
    return repeated_chunks(lines, target_bytes)


def synthetic_pubspec_lock(target_bytes: int) -> bytes:
    sections = [
        "packages:\n"
        "  async:\n"
        "    dependency: transitive\n"
        '    description:\n      name: async\n      sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"\n      url: "https://pub.dev"\n'
        '    source: hosted\n'
        '    version: "2.13.0"\n'
        "  collection:\n"
        "    dependency: transitive\n"
        '    description:\n      name: collection\n      sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"\n      url: "https://pub.dev"\n'
        '    source: hosted\n'
        '    version: "1.19.1"\n\n',
        "sdks:\n"
        '  dart: ">=3.6.0 <4.0.0"\n'
        '  flutter: ">=3.27.0"\n\n',
    ]
    return repeated_chunks(sections, target_bytes)


def synthetic_build_bazel(target_bytes: int) -> bytes:
    blocks = [
        'load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")\n\n',
        'package(default_visibility = ["//visibility:public"])\n\n',
        "rust_library(\n"
        '    name = "core_lib",\n'
        '    srcs = glob(["src/**/*.rs"]),\n'
        '    crate_root = "src/lib.rs",\n'
        '    deps = ["@crate_index//:serde"],\n'
        ")\n\n",
        "rust_binary(\n"
        '    name = "tool",\n'
        '    srcs = ["src/main.rs"],\n'
        '    deps = [":core_lib"],\n'
        ")\n\n",
    ]
    return repeated_chunks(blocks, target_bytes)


def synthetic_workspace(target_bytes: int) -> bytes:
    blocks = [
        'workspace(name = "generated_workspace")\n\n',
        'load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")\n\n',
        "http_archive(\n"
        '    name = "rules_rust",\n'
        '    sha256 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",\n'
        '    urls = ["https://example.com/rules_rust.tar.gz"],\n'
        ")\n\n",
        "register_toolchains(\"@rules_rust//rust:toolchain\")\n\n",
    ]
    return repeated_chunks(blocks, target_bytes)


def prepare_broad_local(output_root: Path, manifest_root: Path):
    suite_name = "broad-local"
    suite_dir = output_root / suite_name
    ensure_clean_dir(suite_dir)
    manifest_root.mkdir(parents=True, exist_ok=True)
    manifest = []
    used_names = set()

    write_generated_fixture(
        repeated_text(1024 * 1024),
        suite_dir,
        "generated_repeated_text_001m.txt",
        manifest,
        "generated_text",
    )
    write_generated_fixture(
        json_logs(1024 * 1024),
        suite_dir,
        "generated_json_logs_001m.jsonl",
        manifest,
        "generated_json",
    )
    write_generated_fixture(
        cross_block_repetition(1024 * 1024),
        suite_dir,
        "generated_cross_block_001m.bin",
        manifest,
        "generated_binary",
    )
    write_generated_fixture(
        xorshift(0xC0FFEE, 1024 * 1024),
        suite_dir,
        "generated_xorshift_001m.bin",
        manifest,
        "generated_incompressible",
    )
    write_generated_fixture(
        synthetic_yarn_lock(128 * 1024),
        suite_dir,
        "generated_yarn.lock",
        manifest,
        "generated_lockfile",
    )
    write_generated_fixture(
        synthetic_poetry_lock(128 * 1024),
        suite_dir,
        "generated_poetry.lock",
        manifest,
        "generated_lockfile",
    )
    write_generated_fixture(
        synthetic_go_sum(128 * 1024),
        suite_dir,
        "generated_go.sum",
        manifest,
        "generated_lockfile",
    )
    write_generated_fixture(
        synthetic_requirements_txt(64 * 1024),
        suite_dir,
        "generated_requirements.txt",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_package_lock_json(128 * 1024),
        suite_dir,
        "generated_package-lock.json",
        manifest,
        "generated_json_lockfile",
    )
    write_generated_fixture(
        synthetic_composer_lock(128 * 1024),
        suite_dir,
        "generated_composer.lock",
        manifest,
        "generated_lockfile",
    )
    write_generated_fixture(
        synthetic_pipfile_lock(128 * 1024),
        suite_dir,
        "generated_pipfile.lock",
        manifest,
        "generated_lockfile",
    )
    write_generated_fixture(
        synthetic_gemfile(64 * 1024),
        suite_dir,
        "generated_Gemfile",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_gemfile_lock(128 * 1024),
        suite_dir,
        "generated_Gemfile.lock",
        manifest,
        "generated_lockfile",
    )
    write_generated_fixture(
        synthetic_go_mod(64 * 1024),
        suite_dir,
        "generated_go.mod",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_package_json(64 * 1024),
        suite_dir,
        "generated_package.json",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_package_json(64 * 1024),
        suite_dir,
        "generated_turbo.json",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_tsconfig_json(64 * 1024),
        suite_dir,
        "generated_tsconfig.json",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_tsconfig_json(64 * 1024),
        suite_dir,
        "generated_deno.json",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_tsconfig_json(64 * 1024),
        suite_dir,
        "generated_nx.json",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_pyproject_toml(64 * 1024),
        suite_dir,
        "generated_pyproject.toml",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_pyproject_toml(64 * 1024),
        suite_dir,
        "generated_wrangler.toml",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_pom_xml(64 * 1024),
        suite_dir,
        "generated_pom.xml",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_dockerfile(64 * 1024),
        suite_dir,
        "generated_Dockerfile",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_pubspec_yaml(64 * 1024),
        suite_dir,
        "generated_pubspec.yaml",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_pubspec_yaml(64 * 1024),
        suite_dir,
        "generated_buf.yaml",
        manifest,
        "generated_config",
    )
    write_generated_fixture(
        synthetic_pubspec_lock(128 * 1024),
        suite_dir,
        "generated_pubspec.lock",
        manifest,
        "generated_lockfile",
    )
    write_generated_fixture(
        synthetic_build_bazel(64 * 1024),
        suite_dir,
        "generated_BUILD.bazel",
        manifest,
        "generated_code",
    )
    write_generated_fixture(
        synthetic_workspace(64 * 1024),
        suite_dir,
        "generated_WORKSPACE",
        manifest,
        "generated_code",
    )

    for source in LOCAL_SOURCE_FIXTURES:
        if source.is_file():
            copy_fixture(
                source,
                suite_dir,
                unique_repo_fixture_name(source, used_names),
                manifest,
                "repo_source",
            )

    for source in LOCAL_BUILD_FIXTURES:
        if source.is_file():
            copy_fixture(source, suite_dir, f"build_{source.name}", manifest, "build_artifact")

    dictionary = REPO_ROOT / "ruzstd" / "dict_tests" / "dictionary"
    if dictionary.is_file():
        copy_fixture(dictionary, suite_dir, "dict_dictionary.bin", manifest, "dictionary")

    dict_files_dir = REPO_ROOT / "ruzstd" / "dict_tests" / "files"
    dict_files = sorted(path for path in dict_files_dir.glob("*.service") if path.is_file())
    for source in select_size_spread(dict_files, 14):
        copy_fixture(source, suite_dir, f"dict_{source.name}", manifest, "dictionary_text")

    decodecorpus_dir = REPO_ROOT / "ruzstd" / "decodecorpus_files"
    decodecorpus_files = sorted(
        path
        for path in decodecorpus_dir.iterdir()
        if path.is_file() and path.suffix != ".zst"
    )
    for source in select_size_spread(decodecorpus_files, 12):
        copy_fixture(source, suite_dir, f"decodecorpus_{source.name}", manifest, "decodecorpus")

    manifest_path = manifest_root / f"{suite_name}.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"{suite_dir}\t{len(manifest)} fixtures")
    print(f"{manifest_path}\tmanifest")


def prepare_broad_c_zstd(output_root: Path, manifest_root: Path, zstd_repo: Path | None):
    if zstd_repo is None:
        raise SystemExit("--zstd-repo is required for the broad-c-zstd suite")

    suite_name = "broad-c-zstd"
    suite_dir = output_root / suite_name
    ensure_clean_dir(suite_dir)
    manifest_root.mkdir(parents=True, exist_ok=True)
    manifest = []

    seen = set()
    for pattern in ZSTD_REPO_PATTERNS:
        matches = sorted(path for path in zstd_repo.glob(pattern) if path.is_file())
        for source in select_size_spread(matches, min(6, len(matches))):
            rel = source.relative_to(zstd_repo)
            name = "_".join(rel.parts)
            if name in seen:
                continue
            seen.add(name)
            copy_fixture(source, suite_dir, name, manifest, "zstd_repo")

    manifest_path = manifest_root / f"{suite_name}.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"{suite_dir}\t{len(manifest)} fixtures")
    print(f"{manifest_path}\tmanifest")


def main():
    args = parse_args()
    args.output_root.mkdir(parents=True, exist_ok=True)
    args.manifest_root.mkdir(parents=True, exist_ok=True)

    if args.suite in ("broad-local", "all"):
        prepare_broad_local(args.output_root, args.manifest_root)
    if args.suite in ("broad-c-zstd", "all"):
        if args.zstd_repo is None:
            print("skipping broad-c-zstd: no --zstd-repo provided")
        else:
            prepare_broad_c_zstd(args.output_root, args.manifest_root, args.zstd_repo)


if __name__ == "__main__":
    main()
