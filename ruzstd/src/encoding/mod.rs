//! Structures and utilities used for compressing/encoding data into the Zstd format.

pub(crate) mod block_header;
pub(crate) mod blocks;
pub(crate) mod frame_header;
pub(crate) mod match_generator;
pub(crate) mod util;

mod frame_compressor;
mod levels;
pub use frame_compressor::FrameCompressor;
pub use match_generator::MatchGeneratorDriver;

use crate::io::{Read, Write};
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::path::Path;

use self::util::{likely_composer_lockfile_text, likely_incompressible, likely_lockfile_text};

const FILE_TYPE_SAMPLE_BYTES: usize = 32 * 1024;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum CompressionFileProfile {
    None,
    CargoLock,
    ComposerLock,
    SmallTextLockfile,
    DependencyJsonLockfile,
}

impl CompressionFileProfile {
    pub(crate) const fn internal_hint_code(self) -> u8 {
        match self {
            Self::None => 0,
            Self::CargoLock => 1,
            Self::ComposerLock => 2,
            Self::SmallTextLockfile => 3,
            Self::DependencyJsonLockfile => 4,
        }
    }

    pub(crate) const fn from_internal_hint_code(code: u8) -> Self {
        match code {
            1 => Self::CargoLock,
            2 => Self::ComposerLock,
            3 => Self::SmallTextLockfile,
            4 => Self::DependencyJsonLockfile,
            _ => Self::None,
        }
    }
}

fn classify_extension_exact(lower_ext: &str) -> Option<CompressionFileType> {
    let family = match lower_ext.as_bytes().first().copied()? {
        b'7' => match lower_ext {
            "7z" => CompressionFileType::ArchiveLike,
            _ => return None,
        },
        b'a' => match lower_ext {
            "a" | "adoc" | "aff" | "apk" | "app" | "ar" | "asm" | "astro" | "avsc" | "awk" => {
                if matches!(lower_ext, "a" | "apk" | "ar") {
                    CompressionFileType::ArchiveLike
                } else if lower_ext == "aff" {
                    CompressionFileType::DictionaryText
                } else if matches!(lower_ext, "adoc" | "avsc") {
                    CompressionFileType::ConfigText
                } else {
                    CompressionFileType::CodeText
                }
            }
            _ => return None,
        },
        b'b' => match lower_ext {
            "bash" | "bat" | "bazel" | "bicep" | "bzl" => CompressionFileType::CodeText,
            "bib" | "bin" | "bz2" => {
                if lower_ext == "bin" {
                    CompressionFileType::BinaryLike
                } else if lower_ext == "bz2" {
                    CompressionFileType::ArchiveLike
                } else {
                    CompressionFileType::ConfigText
                }
            }
            _ => return None,
        },
        b'c' => match lower_ext {
            "c" | "cab" | "cc" | "cfg" | "cjs" | "class" | "clj" | "cljs" | "cljc" | "cmd"
            | "conf" | "config" | "cpp" | "crate" | "cs" | "csproj" | "css" | "csv" | "csh"
            | "cxx" => match lower_ext {
                "cab" | "crate" => CompressionFileType::ArchiveLike,
                "cfg" | "conf" | "config" | "csproj" | "csv" => CompressionFileType::ConfigText,
                "class" => CompressionFileType::BinaryLike,
                _ => CompressionFileType::CodeText,
            },
            _ => return None,
        },
        b'd' => match lower_ext {
            "d" | "dart" | "dat" | "deb" | "desktop" | "dic" | "dict" | "dll" | "dmg"
            | "dockerfile" => match lower_ext {
                "dat" | "dll" => CompressionFileType::BinaryLike,
                "deb" | "dmg" => CompressionFileType::ArchiveLike,
                "desktop" => CompressionFileType::ConfigText,
                "dic" | "dict" => CompressionFileType::DictionaryText,
                _ => CompressionFileType::CodeText,
            },
            _ => return None,
        },
        b'e' => match lower_ext {
            "ear" | "edn" | "elm" | "env" | "eot" | "erl" | "exe" | "ex" | "exs" => match lower_ext
            {
                "ear" => CompressionFileType::ArchiveLike,
                "env" => CompressionFileType::ConfigText,
                "eot" | "exe" => CompressionFileType::BinaryLike,
                _ => CompressionFileType::CodeText,
            },
            _ => return None,
        },
        b'f' => match lower_ext {
            "fish" | "fs" | "fsi" | "fsproj" | "fsx" => {
                if lower_ext == "fsproj" {
                    CompressionFileType::ConfigText
                } else {
                    CompressionFileType::CodeText
                }
            }
            _ => return None,
        },
        b'g' => match lower_ext {
            "gem" | "geojson" | "go" | "gql" | "gradle" | "graphql" | "groovy" | "gvy" | "gz" => {
                match lower_ext {
                    "gem" | "gz" => CompressionFileType::ArchiveLike,
                    "geojson" => CompressionFileType::JsonText,
                    "go" | "gql" | "gradle" | "graphql" | "groovy" | "gvy" => {
                        CompressionFileType::CodeText
                    }
                    _ => return None,
                }
            }
            _ => return None,
        },
        b'h' => match lower_ext {
            "h" | "har" | "hbs" | "hcl" | "hh" | "hjson" | "hpp" | "hrl" | "hs" | "htm"
            | "html" | "hxx" => match lower_ext {
                "har" => CompressionFileType::JsonText,
                "hcl" => CompressionFileType::ConfigText,
                "hjson" => CompressionFileType::JsonText,
                _ => CompressionFileType::CodeText,
            },
            _ => return None,
        },
        b'i' => match lower_ext {
            "img" | "ini" | "ipa" | "ipynb" | "iso" => match lower_ext {
                "ini" => CompressionFileType::ConfigText,
                "ipynb" => CompressionFileType::JsonText,
                "img" => CompressionFileType::BinaryLike,
                _ => CompressionFileType::ArchiveLike,
            },
            _ => return None,
        },
        b'j' => match lower_ext {
            "jar" | "java" | "jl" | "js" | "json" | "json5" | "jsonc" | "jsonl" | "jsx" => {
                match lower_ext {
                    "jar" => CompressionFileType::ArchiveLike,
                    "json" | "json5" | "jsonc" | "jsonl" => CompressionFileType::JsonText,
                    _ => CompressionFileType::CodeText,
                }
            }
            _ => return None,
        },
        b'k' => match lower_ext {
            "kdl" | "kt" | "kts" => {
                if lower_ext == "kdl" {
                    CompressionFileType::ConfigText
                } else {
                    CompressionFileType::CodeText
                }
            }
            _ => return None,
        },
        b'l' => match lower_ext {
            "less" | "lhs" | "lib" | "liquid" | "list" | "log" | "lua" | "lz4" | "lzma" => {
                match lower_ext {
                    "lib" => CompressionFileType::BinaryLike,
                    "list" | "log" => CompressionFileType::ConfigText,
                    "lz4" | "lzma" => CompressionFileType::ArchiveLike,
                    _ => CompressionFileType::CodeText,
                }
            }
            _ => return None,
        },
        b'm' => match lower_ext {
            "m" | "markdown" | "mcmeta" | "md" | "mdx" | "meson" | "mjs" | "ml" | "mli" | "mm"
            | "mount" | "msi" | "msix" => match lower_ext {
                "markdown" | "md" => CompressionFileType::ConfigText,
                "mcmeta" => CompressionFileType::JsonText,
                "mount" => CompressionFileType::ConfigText,
                "msi" | "msix" => CompressionFileType::ArchiveLike,
                _ => CompressionFileType::CodeText,
            },
            _ => return None,
        },
        b'n' => match lower_ext {
            "ndjson" | "nim" | "nix" | "node" | "nupkg" => match lower_ext {
                "ndjson" => CompressionFileType::JsonText,
                "nim" | "nix" => CompressionFileType::CodeText,
                "node" => CompressionFileType::BinaryLike,
                "nupkg" => CompressionFileType::ArchiveLike,
                _ => return None,
            },
            _ => return None,
        },
        b'o' => match lower_ext {
            "o" | "obj" | "objcopy" | "otf" => CompressionFileType::BinaryLike,
            _ => return None,
        },
        b'p' => match lower_ext {
            "pak" | "path" | "pbxproj" | "pdf" | "php" | "phpt" | "pkg" | "pl" | "plist" | "pm"
            | "policy" | "properties" | "props" | "proto" | "ps1" | "psv" | "pyd" | "py"
            | "pyc" | "pyi" | "pyw" => match lower_ext {
                "pak" | "pkg" => CompressionFileType::ArchiveLike,
                "path" | "pbxproj" | "plist" | "policy" | "properties" | "props" | "psv" => {
                    CompressionFileType::ConfigText
                }
                "pdf" | "pyd" | "pyc" => CompressionFileType::BinaryLike,
                _ => CompressionFileType::CodeText,
            },
            _ => return None,
        },
        b'r' => match lower_ext {
            "r" | "rar" | "rb" | "repo" | "resx" | "rlib" | "ron" | "rpm" | "rs" | "rst"
            | "rules" | "rkt" => match lower_ext {
                "rar" | "rpm" => CompressionFileType::ArchiveLike,
                "repo" | "resx" | "ron" | "rst" | "rules" => CompressionFileType::ConfigText,
                "rlib" => CompressionFileType::ArchiveLike,
                _ => CompressionFileType::CodeText,
            },
            _ => return None,
        },
        b's' => match lower_ext {
            "s" | "sass" | "scala" | "scss" | "sed" | "service" | "sh" | "sln" | "so"
            | "socket" | "sql" | "svg" | "svelte" | "swift" => match lower_ext {
                "service" | "sln" | "socket" | "svg" => CompressionFileType::ConfigText,
                "so" => CompressionFileType::BinaryLike,
                _ => CompressionFileType::CodeText,
            },
            _ => return None,
        },
        b't' => match lower_ext {
            "tar" | "target" | "tcl" | "tex" | "tf" | "tfvars" | "tgz" | "thrift" | "timer"
            | "toml" | "topojson" | "ts" | "tsv" | "tsx" | "ttf" | "twig" | "txz" | "tzst" => {
                match lower_ext {
                    "tar" | "tgz" | "txz" | "tzst" => CompressionFileType::ArchiveLike,
                    "target" | "tex" | "tf" | "tfvars" | "timer" | "toml" | "tsv" => {
                        CompressionFileType::ConfigText
                    }
                    "topojson" => CompressionFileType::JsonText,
                    "ttf" => CompressionFileType::BinaryLike,
                    _ => CompressionFileType::CodeText,
                }
            }
            _ => return None,
        },
        b'v' => match lower_ext {
            "vbproj" | "vcxproj" | "vue" => {
                if lower_ext == "vue" {
                    CompressionFileType::CodeText
                } else {
                    CompressionFileType::ConfigText
                }
            }
            _ => return None,
        },
        b'w' => match lower_ext {
            "war" | "wasm" | "webmanifest" | "whl" | "woff" | "woff2" => match lower_ext {
                "war" | "whl" => CompressionFileType::ArchiveLike,
                "webmanifest" => CompressionFileType::JsonText,
                _ => CompressionFileType::BinaryLike,
            },
            _ => return None,
        },
        b'x' => match lower_ext {
            "xaml" | "xapk" | "xconfig" | "xhtml" | "xcconfig" | "xcstrings" | "xml" | "xpi"
            | "xsd" | "xsl" | "xslt" | "xz" => match lower_ext {
                "xapk" | "xpi" | "xz" => CompressionFileType::ArchiveLike,
                "xaml" | "xconfig" | "xcconfig" | "xml" | "xsd" | "xsl" | "xslt" => {
                    CompressionFileType::ConfigText
                }
                "xcstrings" => CompressionFileType::JsonText,
                "xhtml" => CompressionFileType::CodeText,
                _ => return None,
            },
            _ => return None,
        },
        b'y' => match lower_ext {
            "yaml" | "yml" => CompressionFileType::ConfigText,
            _ => return None,
        },
        b'z' => match lower_ext {
            "zig" | "zip" | "zsh" | "zst" => match lower_ext {
                "zip" | "zst" => CompressionFileType::ArchiveLike,
                _ => CompressionFileType::CodeText,
            },
            _ => return None,
        },
        _ => return None,
    };
    Some(family)
}

fn classify_named_file_exact(lower_name: &str) -> Option<CompressionFileType> {
    let family = match lower_name.as_bytes().first().copied()? {
        b'.' => match lower_name {
            ".babelrc" | ".clang-format" | ".clang-tidy" | ".coveragerc" | ".dockerignore"
            | ".editorconfig" | ".env" | ".env.development" | ".env.local" | ".env.production"
            | ".env.test" | ".eslintignore" | ".eslintrc" | ".flake8" | ".gitattributes"
            | ".gitignore" | ".gitmodules" | ".mailmap" | ".npmignore" | ".npmrc"
            | ".prettierignore" | ".prettierrc" | ".pylintrc" | ".shellcheckrc"
            | ".stylelintrc" | ".yamllint" | ".yarnrc" | ".yarnrc.yml" => {
                CompressionFileType::ConfigText
            }
            _ => return None,
        },
        b'a' => match lower_name {
            "api-extractor.json" | "appveyor.yml" | "azure-pipelines.yml" => {
                CompressionFileType::ConfigText
            }
            _ => return None,
        },
        b'b' => match lower_name {
            "brewfile" => CompressionFileType::ConfigText,
            "bsdmakefile" | "build.bazel" | "buildfile" => CompressionFileType::CodeText,
            "build.gradle" | "build.gradle.kts" => CompressionFileType::ConfigText,
            "buf.gen.yaml" | "buf.work.yaml" | "buf.yaml" => CompressionFileType::ConfigText,
            "bun.lock" => CompressionFileType::DictionaryText,
            _ => return None,
        },
        b'c' => match lower_name {
            "cargo.lock" => CompressionFileType::DictionaryText,
            "cmakelists.txt" => CompressionFileType::CodeText,
            "composer.json" => CompressionFileType::ConfigText,
            "composer.lock" | "containerfile" => {
                if lower_name == "containerfile" {
                    CompressionFileType::ConfigText
                } else {
                    CompressionFileType::DictionaryText
                }
            }
            _ => return None,
        },
        b'd' => match lower_name {
            "deno.json" | "deno.jsonc" | "devcontainer.json" => CompressionFileType::ConfigText,
            "docker-compose.yaml" | "docker-compose.yml" | "dockerfile" => {
                CompressionFileType::ConfigText
            }
            _ => return None,
        },
        b'g' => match lower_name {
            "gemfile" => CompressionFileType::ConfigText,
            "gemfile.lock" | "go.sum" => CompressionFileType::DictionaryText,
            "gnumakefile" | "go.mod" => {
                if lower_name == "go.mod" {
                    CompressionFileType::ConfigText
                } else {
                    CompressionFileType::CodeText
                }
            }
            "gradle.properties" => CompressionFileType::ConfigText,
            _ => return None,
        },
        b'j' => match lower_name {
            "jenkinsfile" | "justfile" => CompressionFileType::CodeText,
            "jsconfig.json" => CompressionFileType::ConfigText,
            _ => return None,
        },
        b'l' => match lower_name {
            "lerna.json" => CompressionFileType::ConfigText,
            _ => return None,
        },
        b'm' => match lower_name {
            "makefile" | "melos.yaml" | "meson.build" | "meson_options.txt" | "module.bazel" => {
                if lower_name == "melos.yaml" {
                    CompressionFileType::ConfigText
                } else {
                    CompressionFileType::CodeText
                }
            }
            "mix.lock" => CompressionFileType::DictionaryText,
            _ => return None,
        },
        b'n' => match lower_name {
            "netlify.toml" | "nx.json" => CompressionFileType::ConfigText,
            _ => return None,
        },
        b'p' => match lower_name {
            "package.json" => CompressionFileType::ConfigText,
            "pipfile"
            | "pnpm-workspace.yaml"
            | "podfile"
            | "pom.xml"
            | "procfile"
            | "pubspec.yaml"
            | "pyproject.toml" => CompressionFileType::ConfigText,
            "pipfile.lock" | "podfile.lock" | "pnpm-lock.yaml" | "pubspec.lock" => {
                CompressionFileType::DictionaryText
            }
            "poetry.lock" => CompressionFileType::ConfigText,
            _ => return None,
        },
        b'r' => match lower_name {
            "rakefile" => CompressionFileType::CodeText,
            "release-please-config.json" | "renovate.json" => CompressionFileType::ConfigText,
            "requirements.txt" => CompressionFileType::ConfigText,
            _ => return None,
        },
        b's' => match lower_name {
            "settings.gradle" | "settings.gradle.kts" => CompressionFileType::ConfigText,
            _ => return None,
        },
        b't' => match lower_name {
            "taskfile.yml" => CompressionFileType::ConfigText,
            "tiltfile" => CompressionFileType::CodeText,
            "tsconfig.json" | "turbo.json" | "typedoc.json" => CompressionFileType::ConfigText,
            _ => return None,
        },
        b'v' => match lower_name {
            "vagrantfile" => CompressionFileType::ConfigText,
            _ => return None,
        },
        b'w' => match lower_name {
            "wrangler.toml" => CompressionFileType::ConfigText,
            "workspace" => CompressionFileType::CodeText,
            _ => return None,
        },
        b'y' => match lower_name {
            "yarn.lock" => CompressionFileType::ConfigText,
            _ => return None,
        },
        _ => return None,
    };
    Some(family)
}

fn classify_file_name(lower_name: &str) -> CompressionFileType {
    if lower_name.contains("dictionary") {
        return CompressionFileType::DictionaryText;
    }

    if let Some(family) = classify_named_file_exact(lower_name) {
        return family;
    }

    for suffix in file_name_prefixed_suffixes(lower_name) {
        if let Some(family) = classify_named_file_exact(suffix) {
            return family;
        }
    }

    for (idx, byte) in lower_name.bytes().enumerate() {
        if byte == b'.' && idx + 1 < lower_name.len() {
            if let Some(family) = classify_extension_exact(&lower_name[idx + 1..]) {
                return family;
            }
        }
    }

    CompressionFileType::Unknown
}

fn file_name_prefixed_suffixes(lower_name: &str) -> impl Iterator<Item = &str> {
    lower_name
        .bytes()
        .enumerate()
        .filter_map(move |(idx, byte)| {
            if matches!(byte, b'_' | b'-' | b'.') && idx + 1 < lower_name.len() {
                Some(&lower_name[idx + 1..])
            } else {
                None
            }
        })
}

fn file_name_matches_exact_or_prefixed(lower_name: &str, needle: &str) -> bool {
    lower_name == needle || file_name_prefixed_suffixes(lower_name).any(|suffix| suffix == needle)
}

fn likely_json_text_sample(data: &[u8]) -> bool {
    if !sample_looks_text(data) {
        return false;
    }

    let trimmed = data
        .iter()
        .skip_while(|&&byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
        .copied()
        .collect::<Vec<u8>>();
    let trimmed = trimmed.as_slice();
    if !matches!(trimmed.first(), Some(b'{') | Some(b'[')) {
        return false;
    }

    let window = &trimmed[..trimmed.len().min(2048)];
    window.contains(&b':') && window.iter().filter(|&&byte| byte == b'"').count() >= 4
}

fn sample_looks_text(data: &[u8]) -> bool {
    if data.len() < 16 {
        return false;
    }

    let step = (data.len() / 128).max(1);
    let mut printable = 0usize;
    let mut total = 0usize;
    for idx in (0..data.len()).step_by(step).take(128) {
        total += 1;
        let byte = data[idx];
        if byte == b'\n'
            || byte == b'\r'
            || byte == b'\t'
            || byte == b' '
            || byte.is_ascii_graphic()
        {
            printable += 1;
        }
    }

    printable * 100 >= total * 85
}

fn likely_dictionary_text_sample(data: &[u8]) -> bool {
    let mut package_markers = 0usize;
    let mut name_markers = 0usize;
    let mut version_markers = 0usize;
    let mut checksum_markers = 0usize;
    let mut composer_packages = false;
    let mut composer_require = 0usize;
    let mut composer_reference = 0usize;

    for line in data.split(|&byte| byte == b'\n').take(512) {
        let trimmed = line
            .iter()
            .skip_while(|&&byte| matches!(byte, b' ' | b'\t'))
            .copied()
            .collect::<Vec<u8>>();
        let trimmed = trimmed.as_slice();

        if trimmed.starts_with(b"[[package]]") {
            package_markers += 1;
        } else if trimmed.starts_with(b"name = \"") || trimmed.starts_with(br#""name": ""#) {
            name_markers += 1;
        } else if trimmed.starts_with(b"version = \"") || trimmed.starts_with(br#""version": ""#) {
            version_markers += 1;
        } else if trimmed.starts_with(b"checksum = \"") {
            checksum_markers += 1;
        } else if trimmed.starts_with(br#""packages": ["#) {
            composer_packages = true;
        } else if trimmed.starts_with(br#""require": {"#) {
            composer_require += 1;
        } else if trimmed.starts_with(br#""reference": ""#) {
            composer_reference += 1;
        }

        if package_markers >= 2
            && name_markers >= 2
            && version_markers >= 2
            && checksum_markers >= 2
        {
            return true;
        }
        if composer_packages
            && name_markers >= 2
            && version_markers >= 2
            && composer_require >= 2
            && composer_reference >= 2
        {
            return true;
        }
    }

    false
}

fn likely_config_text_sample(data: &[u8]) -> bool {
    if !sample_looks_text(data) {
        return false;
    }

    let mut score = 0usize;
    for line in data.split(|&byte| byte == b'\n').take(256) {
        let line = line
            .iter()
            .skip_while(|&&byte| matches!(byte, b' ' | b'\t'))
            .copied()
            .collect::<Vec<u8>>();
        let line = line.as_slice();
        if line.is_empty() {
            continue;
        }

        if (line.starts_with(b"[") && line.ends_with(b"]"))
            || (line.starts_with(b"<") && line.contains(&b'>'))
        {
            score += 2;
        } else if line.starts_with(b"- ") || line.starts_with(b"* ") {
            score += 1;
        } else if line.starts_with(b"FROM ")
            || line.starts_with(b"RUN ")
            || line.starts_with(b"ENV ")
            || line.starts_with(b"COPY ")
            || line.starts_with(b"WORKDIR ")
            || line.starts_with(b"ENTRYPOINT ")
            || line.starts_with(b"CMD ")
        {
            score += 2;
        } else if (line.windows(2).any(|pair| pair == b": ")
            && !line.windows(3).any(|pair| pair == b"://"))
            || line.contains(&b'=')
        {
            score += 1;
        }

        if score >= 4 {
            return true;
        }
    }

    false
}

fn likely_code_text_sample(data: &[u8]) -> bool {
    if !sample_looks_text(data) {
        return false;
    }

    let mut score = 0usize;
    for line in data.split(|&byte| byte == b'\n').take(256) {
        let line = line
            .iter()
            .skip_while(|&&byte| matches!(byte, b' ' | b'\t'))
            .copied()
            .collect::<Vec<u8>>();
        let line = line.as_slice();
        if line.is_empty() {
            continue;
        }

        if line.ends_with(b";") || line.ends_with(b"{") || line.ends_with(b"}") {
            score += 1;
        }
        if line
            .windows(2)
            .any(|pair| matches!(pair, b"::" | b"->" | b"=>" | b"//"))
        {
            score += 1;
        }
        if line.starts_with(b"fn ")
            || line.starts_with(b"pub ")
            || line.starts_with(b"let ")
            || line.starts_with(b"const ")
            || line.starts_with(b"var ")
            || line.starts_with(b"def ")
            || line.starts_with(b"class ")
            || line.starts_with(b"import ")
            || line.starts_with(b"package ")
            || line.starts_with(b"use ")
            || line.starts_with(b"#include")
            || line.starts_with(b"<?php")
        {
            score += 2;
        }

        if score >= 4 {
            return true;
        }
    }

    false
}

fn has_archive_signature(data: &[u8]) -> bool {
    data.starts_with(b"PK\x03\x04")
        || data.starts_with(b"PK\x05\x06")
        || data.starts_with(b"PK\x07\x08")
        || data.starts_with(&[0x1F, 0x8B])
        || data.starts_with(b"BZh")
        || data.starts_with(&[0xFD, b'7', b'z', b'X', b'Z', 0x00])
        || data.starts_with(&[0x28, 0xB5, 0x2F, 0xFD])
        || data.starts_with(&[b'7', b'z', 0xBC, 0xAF, 0x27, 0x1C])
        || data.starts_with(b"Rar!\x1A\x07\x00")
        || data.starts_with(b"Rar!\x1A\x07\x01\x00")
        || (data.len() > 262 && &data[257..262] == b"ustar")
}

fn has_binary_signature(data: &[u8]) -> bool {
    data.starts_with(b"\x7FELF")
        || data.starts_with(b"MZ")
        || data.starts_with(b"\0asm")
        || data.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE])
        || data.starts_with(&[0xFE, 0xED, 0xFA, 0xCE])
        || data.starts_with(&[0xFE, 0xED, 0xFA, 0xCF])
        || data.starts_with(&[0xCE, 0xFA, 0xED, 0xFE])
        || data.starts_with(&[0xCF, 0xFA, 0xED, 0xFE])
        || data.starts_with(b"SQLite format 3\0")
        || data.starts_with(b"%PDF-")
}

fn compression_file_type_for_sample(sample: &[u8]) -> CompressionFileType {
    if sample.is_empty() {
        return CompressionFileType::Unknown;
    }
    if has_archive_signature(sample) {
        return CompressionFileType::ArchiveLike;
    }
    if has_binary_signature(sample) {
        return CompressionFileType::BinaryLike;
    }
    if likely_lockfile_text(sample)
        || likely_composer_lockfile_text(sample)
        || likely_dictionary_text_sample(sample)
    {
        return CompressionFileType::DictionaryText;
    }
    if likely_json_text_sample(sample) {
        return CompressionFileType::JsonText;
    }
    if likely_config_text_sample(sample) {
        return CompressionFileType::ConfigText;
    }
    if likely_code_text_sample(sample) {
        return CompressionFileType::CodeText;
    }
    if likely_incompressible(sample) {
        return CompressionFileType::BinaryLike;
    }
    CompressionFileType::Unknown
}

#[cfg(feature = "std")]
fn read_file_type_sample<R: Read>(source: &mut R) -> Vec<u8> {
    let mut sample = Vec::with_capacity(FILE_TYPE_SAMPLE_BYTES);
    let mut buffer = [0u8; 4096];
    while sample.len() < FILE_TYPE_SAMPLE_BYTES {
        let wanted = (FILE_TYPE_SAMPLE_BYTES - sample.len()).min(buffer.len());
        let read = source.read(&mut buffer[..wanted]).unwrap();
        if read == 0 {
            break;
        }
        sample.extend_from_slice(&buffer[..read]);
    }
    sample
}

/// Convenience function to compress some source into a target without reusing any resources of the compressor
/// ```rust
/// use ruzstd::encoding::{compress, CompressionLevel};
/// let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0];
/// let mut target = Vec::new();
/// compress(data, &mut target, CompressionLevel::Fastest);
/// ```
pub fn compress<R: Read, W: Write>(source: R, target: W, level: CompressionLevel) {
    let mut frame_enc = FrameCompressor::new(level);
    frame_enc.set_source(source);
    frame_enc.set_drain(target);
    frame_enc.compress();
}

/// Convenience function to compress some source into a target using a coarse file-type hint.
///
/// The public API stays narrow: callers provide only the requested compression level and
/// the file family. The encoder decides the internal starting point from there.
pub fn compress_with_file_type<R: Read, W: Write>(
    source: R,
    target: W,
    file_type: CompressionFileType,
    level: CompressionLevel,
) {
    let mut frame_enc =
        FrameCompressor::new_with_hints(level, file_type, CompressionFileProfile::None);
    frame_enc.set_source(source);
    frame_enc.set_drain(target);
    frame_enc.compress();
}

/// Convenience function to compress some source into a target using a path-based file-type hint.
#[cfg(feature = "std")]
pub fn compress_with_path<R: Read, W: Write>(
    mut source: R,
    target: W,
    path: &Path,
    level: CompressionLevel,
) {
    let sample = read_file_type_sample(&mut source);
    let file_type = compression_file_type_for_path_and_data(path, &sample);
    let file_profile = compression_file_profile_for_path_and_data(path, &sample);
    let mut frame_enc = FrameCompressor::new_with_hints(level, file_type, file_profile);
    frame_enc.set_source(sample.as_slice().chain(source));
    frame_enc.set_drain(target);
    frame_enc.compress();
}

/// Convenience function to compress some source into a Vec without reusing any resources of the compressor
/// ```rust
/// use ruzstd::encoding::{compress_to_vec, CompressionLevel};
/// let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0];
/// let compressed = compress_to_vec(data, CompressionLevel::Fastest);
/// ```
pub fn compress_to_vec<R: Read>(source: R, level: CompressionLevel) -> Vec<u8> {
    let mut vec = Vec::new();
    compress(source, &mut vec, level);
    vec
}

/// Convenience function to compress some source into a Vec using a coarse file-type hint.
pub fn compress_to_vec_with_file_type<R: Read>(
    source: R,
    file_type: CompressionFileType,
    level: CompressionLevel,
) -> Vec<u8> {
    let mut vec = Vec::new();
    compress_with_file_type(source, &mut vec, file_type, level);
    vec
}

/// Convenience function to compress some source into a Vec using a path-based file-type hint.
#[cfg(feature = "std")]
pub fn compress_to_vec_with_path<R: Read>(
    source: R,
    path: &Path,
    level: CompressionLevel,
) -> Vec<u8> {
    let mut vec = Vec::new();
    compress_with_path(source, &mut vec, path, level);
    vec
}

/// The compression mode used impacts the speed of compression,
/// and resulting compression ratios. Faster compression will result
/// in worse compression ratios, and vice versa.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CompressionLevel {
    /// This level does not compress the data at all, and simply wraps
    /// it in a Zstandard frame.
    Uncompressed,
    /// This level is roughly equivalent to Zstd compression level 1
    Fastest,
    /// This level is roughly equivalent to Zstd level 3,
    /// or the one used by the official compressor when no level
    /// is specified.
    ///
    /// UNIMPLEMENTED
    Default,
    /// This level is roughly equivalent to Zstd level 7.
    ///
    /// UNIMPLEMENTED
    Better,
    /// This level is roughly equivalent to Zstd level 11.
    ///
    /// UNIMPLEMENTED
    Best,
}

/// Coarse file families used to choose the encoder's internal starting point.
///
/// This is intentionally small and stable. It is not an exposed bag of tuning knobs.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CompressionFileType {
    Unknown,
    ArchiveLike,
    BinaryLike,
    CodeText,
    ConfigText,
    JsonText,
    DictionaryText,
}

#[cfg(feature = "std")]
pub fn compression_file_type_for_path(path: &Path) -> CompressionFileType {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    classify_file_name(&file_name.to_ascii_lowercase())
}

/// Classify using path first, then sample bytes if the path does not resolve to a known family.
#[cfg(feature = "std")]
pub fn compression_file_type_for_path_and_data(path: &Path, sample: &[u8]) -> CompressionFileType {
    let by_path = compression_file_type_for_path(path);
    if by_path != CompressionFileType::Unknown {
        return by_path;
    }
    compression_file_type_for_sample(sample)
}

#[cfg(feature = "std")]
pub(crate) fn compression_file_profile_for_path_and_data(
    path: &Path,
    sample: &[u8],
) -> CompressionFileProfile {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if file_name_matches_exact_or_prefixed(&file_name, "cargo.lock") {
        CompressionFileProfile::CargoLock
    } else if file_name_matches_exact_or_prefixed(&file_name, "composer.lock") {
        CompressionFileProfile::ComposerLock
    } else if file_name_matches_exact_or_prefixed(&file_name, "poetry.lock")
        || file_name_matches_exact_or_prefixed(&file_name, "pubspec.lock")
    {
        CompressionFileProfile::SmallTextLockfile
    } else if file_name_matches_exact_or_prefixed(&file_name, "package-lock.json")
        || file_name_matches_exact_or_prefixed(&file_name, "pipfile.lock")
        || util::likely_dependency_json_lockfile_text(sample)
    {
        CompressionFileProfile::DependencyJsonLockfile
    } else {
        CompressionFileProfile::None
    }
}

/// Trait used by the encoder that users can use to extend the matching facilities with their own algorithm
/// making their own tradeoffs between runtime, memory usage and compression ratio
///
/// This trait operates on buffers that represent the chunks of data the matching algorithm wants to work on.
/// Each one of these buffers is referred to as a *space*. One or more of these buffers represent the window
/// the decoder will need to decode the data again.
///
/// This library asks the Matcher for a new buffer using `get_next_space` to allow reusing of allocated buffers when they are no longer part of the
/// window of data that is being used for matching.
///
/// The library fills the buffer with data that is to be compressed and commits them back to the matcher using `commit_space`.
///
/// Then it will either call `start_matching` or, if the space is deemed not worth compressing, `skip_matching` is called.
///
/// This is repeated until no more data is left to be compressed.
pub trait Matcher {
    /// Get a space where we can put data to be matched on. Will be encoded as one block. The maximum allowed size is 128 kB.
    fn get_next_space(&mut self) -> alloc::vec::Vec<u8>;
    /// Get a reference to the last commited space
    fn get_last_space(&self) -> &[u8];
    /// Commit a space to the matcher so it can be matched against
    fn commit_space(&mut self, space: alloc::vec::Vec<u8>);
    /// Just process the data in the last commited space for future matching
    fn skip_matching(&mut self);
    /// Process the data in the last commited space for future matching AND generate matches for the data
    fn start_matching(&mut self, handle_sequence: impl for<'a> FnMut(Sequence<'a>));
    /// Reset this matcher so it can be used for the next new frame
    fn reset(&mut self, level: CompressionLevel);
    /// Provide a coarse file-type hint so the matcher can choose an internal starting point.
    ///
    /// Implementations that do not care about path/extension hints can ignore this hook.
    fn set_file_type_hint(&mut self, _file_type: CompressionFileType) {}
    /// Provide a narrower internal file profile when one is known.
    ///
    /// This is encoded as a small integer so the public matcher trait does not expose the
    /// encoder's private profile enum.
    fn set_internal_file_profile_hint(&mut self, _file_profile_code: u8) {}
    /// Synchronize the matcher with the encoder's current repeat-offset history.
    ///
    /// Matchers that do not use repeat-offset history can ignore this hook.
    fn set_repeat_offsets(&mut self, _newest: u32, _second: u32, _third: u32) {}
    /// Mark the last committed space as processed without indexing it for future matches.
    ///
    /// This is intended for data that has already been classified as very unlikely to
    /// be useful match history, such as incompressible raw blocks.
    fn skip_matching_for_incompressible(&mut self) {
        self.skip_matching();
    }
    /// Mark the last committed space as processed after it was emitted as an RLE block.
    ///
    /// The default behavior preserves the existing matcher contract by indexing the block
    /// normally. Matchers can specialize this because every minimum-match suffix in an RLE
    /// block has the same key.
    fn skip_matching_for_rle(&mut self) {
        self.skip_matching();
    }
    /// The size of the window the decoder will need to execute all sequences produced by this matcher
    ///
    /// May change after a call to reset with a different compression level
    fn window_size(&self) -> u64;
}

#[derive(PartialEq, Eq, Debug)]
/// Sequences that a [`Matcher`] can produce
pub enum Sequence<'data> {
    /// Is encoded as a sequence for the decoder sequence execution.
    ///
    /// First the literals will be copied to the decoded data,
    /// then `match_len` bytes are copied from `offset` bytes back in the decoded data
    Triple {
        literals: &'data [u8],
        offset: usize,
        match_len: usize,
    },
    /// This is returned as the last sequence in a block
    ///
    /// These literals will just be copied at the end of the sequence execution by the decoder
    Literals { literals: &'data [u8] },
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::{
        compression_file_profile_for_path_and_data, compression_file_type_for_path,
        compression_file_type_for_path_and_data, CompressionFileProfile, CompressionFileType,
        CompressionLevel,
    };
    use alloc::format;
    use alloc::vec::Vec;
    use std::path::Path;

    #[test]
    fn classifies_json_logs_as_json_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("logs/events.jsonl")),
            CompressionFileType::JsonText
        );
    }

    #[test]
    fn classifies_rust_sources_as_code_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("src/main.rs")),
            CompressionFileType::CodeText
        );
    }

    #[test]
    fn classifies_dictionary_named_bin_as_dictionary_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("dict_dictionary.bin")),
            CompressionFileType::DictionaryText
        );
    }

    #[test]
    fn classifies_service_file_as_config_text_even_with_dict_prefix() {
        assert_eq!(
            compression_file_type_for_path(Path::new("dict_systemd-logind.service")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_dockerfile_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("Dockerfile")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_makefile_as_code_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("Makefile")),
            CompressionFileType::CodeText
        );
    }

    #[test]
    fn classifies_prefixed_gitignore_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("repo_.gitignore")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_prefixed_cargo_lock_as_dictionary_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("repo_Cargo.lock")),
            CompressionFileType::DictionaryText
        );
    }

    #[test]
    fn classifies_cargo_lock_as_dictionary_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("Cargo.lock")),
            CompressionFileType::DictionaryText
        );
    }

    #[test]
    fn classifies_prefixed_yarn_lock_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("repo_yarn.lock")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_go_sum_as_dictionary_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("go.sum")),
            CompressionFileType::DictionaryText
        );
    }

    #[test]
    fn classifies_gemfile_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("Gemfile")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_gemfile_lock_as_dictionary_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("Gemfile.lock")),
            CompressionFileType::DictionaryText
        );
    }

    #[test]
    fn classifies_pipfile_lock_as_dictionary_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("Pipfile.lock")),
            CompressionFileType::DictionaryText
        );
    }

    #[test]
    fn classifies_composer_lock_as_dictionary_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("composer.lock")),
            CompressionFileType::DictionaryText
        );
    }

    #[test]
    fn profiles_prefixed_package_lock_as_dependency_json_lockfile() {
        let sample = br#"{
  "name": "app",
  "lockfileVersion": 3,
  "packages": {
    "": { "name": "app" }
  }
}"#;
        assert_eq!(
            compression_file_profile_for_path_and_data(
                Path::new("generated_package-lock.json"),
                sample
            ),
            CompressionFileProfile::DependencyJsonLockfile
        );
    }

    #[test]
    fn profiles_prefixed_cargo_lock_as_cargo_lock() {
        let sample = br#"[[package]]
name = "ruzstd"
version = "0.1.0"
"#;
        assert_eq!(
            compression_file_profile_for_path_and_data(Path::new("repo_Cargo.lock"), sample),
            CompressionFileProfile::CargoLock
        );
    }

    #[test]
    fn profiles_prefixed_composer_lock_as_composer_lock() {
        let sample = br#"{
  "packages": [
    {
      "name": "vendor/package",
      "require": {
        "php": ">=8.2"
      },
      "source": {
        "reference": "0000000000000000000000000000000000000001"
      },
      "version": "1.0.0"
    }
  ]
}"#;
        assert_eq!(
            compression_file_profile_for_path_and_data(
                Path::new("generated_composer.lock"),
                sample
            ),
            CompressionFileProfile::ComposerLock
        );
    }

    #[test]
    fn profiles_prefixed_poetry_lock_as_small_text_lockfile() {
        let sample = br#"[[package]]
name = "dep"
version = "1.0.0"
"#;
        assert_eq!(
            compression_file_profile_for_path_and_data(Path::new("generated_poetry.lock"), sample),
            CompressionFileProfile::SmallTextLockfile
        );
    }

    #[test]
    fn profiles_prefixed_pubspec_lock_as_small_text_lockfile() {
        let sample = br#"sdks:
  dart: ">=3.0.0 <4.0.0"
packages:
  args:
    dependency: transitive
"#;
        assert_eq!(
            compression_file_profile_for_path_and_data(Path::new("generated_pubspec.lock"), sample),
            CompressionFileProfile::SmallTextLockfile
        );
    }

    #[test]
    fn profiles_unknown_path_from_dependency_json_sample() {
        let mut sample = Vec::new();
        sample.extend_from_slice(b"{\n  \"_meta\": {},\n  \"default\": {\n");
        for idx in 0..256 {
            sample.extend_from_slice(format!("    \"dep-{idx:04}\": {{\n").as_bytes());
            sample.extend_from_slice(b"      \"hashes\": [\n");
            sample.extend_from_slice(
                b"        \"sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\n",
            );
            sample.extend_from_slice(b"      ],\n");
            sample.extend_from_slice(b"      \"version\": \"==1.0.0\"\n");
            sample.extend_from_slice(b"    },\n");
        }
        sample.extend_from_slice(b"  },\n  \"develop\": {\n");
        for idx in 0..256 {
            sample.extend_from_slice(format!("    \"devdep-{idx:04}\": {{\n").as_bytes());
            sample.extend_from_slice(b"      \"hashes\": [\n");
            sample.extend_from_slice(
                b"        \"sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\"\n",
            );
            sample.extend_from_slice(b"      ],\n");
            sample.extend_from_slice(b"      \"version\": \"==2.0.0\"\n");
            sample.extend_from_slice(b"    },\n");
        }
        sample.extend_from_slice(b"  }\n}\n");
        assert_eq!(
            compression_file_profile_for_path_and_data(Path::new("fixture.data"), &sample),
            CompressionFileProfile::DependencyJsonLockfile
        );
    }

    #[test]
    fn classifies_go_mod_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("go.mod")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_poetry_lock_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("poetry.lock")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_requirements_txt_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("requirements.txt")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_prefixed_npmrc_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("repo_.npmrc")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_proto_as_code_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("schema/service.proto")),
            CompressionFileType::CodeText
        );
    }

    #[test]
    fn classifies_dic_as_dictionary_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("lexicon/en_US.dic")),
            CompressionFileType::DictionaryText
        );
    }

    #[test]
    fn classifies_crate_as_archive_like() {
        assert_eq!(
            compression_file_type_for_path(Path::new("target/package/demo.crate")),
            CompressionFileType::ArchiveLike
        );
    }

    #[test]
    fn classifies_wasm_as_binary_like() {
        assert_eq!(
            compression_file_type_for_path(Path::new("pkg/module.wasm")),
            CompressionFileType::BinaryLike
        );
    }

    #[test]
    fn classifies_compound_archive_extensions() {
        assert_eq!(
            compression_file_type_for_path(Path::new("archives/release.tar.gz")),
            CompressionFileType::ArchiveLike
        );
    }

    #[test]
    fn classifies_prefixed_package_json_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("repo_package.json")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_tsconfig_json_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("tsconfig.json")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_markdown_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("docs/notes.md")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_pyproject_toml_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("pyproject.toml")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_html_as_code_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("web/index.html")),
            CompressionFileType::CodeText
        );
    }

    #[test]
    fn classifies_build_bazel_as_code_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("BUILD.bazel")),
            CompressionFileType::CodeText
        );
    }

    #[test]
    fn classifies_workspace_as_code_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("WORKSPACE")),
            CompressionFileType::CodeText
        );
    }

    #[test]
    fn classifies_pubspec_lock_as_dictionary_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("pubspec.lock")),
            CompressionFileType::DictionaryText
        );
    }

    #[test]
    fn classifies_pubspec_yaml_as_config_text() {
        assert_eq!(
            compression_file_type_for_path(Path::new("pubspec.yaml")),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn classifies_additional_named_configs() {
        for path in [
            "deno.json",
            "devcontainer.json",
            "buf.yaml",
            "turbo.json",
            "nx.json",
            "taskfile.yml",
            "wrangler.toml",
        ] {
            assert_eq!(
                compression_file_type_for_path(Path::new(path)),
                CompressionFileType::ConfigText,
                "{path}"
            );
        }
    }

    #[test]
    fn classifies_additional_extensions() {
        let config_paths = [
            "terraform/main.tf",
            "terraform/dev.tfvars",
            "ios/Project.xcconfig",
            "ios/project.pbxproj",
            "resources/strings.resx",
        ];
        for path in config_paths {
            assert_eq!(
                compression_file_type_for_path(Path::new(path)),
                CompressionFileType::ConfigText,
                "{path}"
            );
        }

        let json_paths = ["policy/config.hjson", "resources/App.xcstrings"];
        for path in json_paths {
            assert_eq!(
                compression_file_type_for_path(Path::new(path)),
                CompressionFileType::JsonText,
                "{path}"
            );
        }

        let code_paths = ["infra/main.bicep", "nix/shell.nix"];
        for path in code_paths {
            assert_eq!(
                compression_file_type_for_path(Path::new(path)),
                CompressionFileType::CodeText,
                "{path}"
            );
        }
    }

    #[test]
    fn samples_json_before_falling_back_to_unknown() {
        let sample = br#"{"packages":[{"name":"alpha","version":"1.0.0"}]}"#;
        assert_eq!(
            compression_file_type_for_path_and_data(Path::new("payload"), sample),
            CompressionFileType::JsonText
        );
    }

    #[test]
    fn samples_code_before_falling_back_to_unknown() {
        let sample = b"fn main() {\n    let value = call();\n    println!(\"{value}\");\n}\n";
        assert_eq!(
            compression_file_type_for_path_and_data(Path::new("payload"), sample),
            CompressionFileType::CodeText
        );
    }

    #[test]
    fn samples_config_before_falling_back_to_unknown() {
        let sample =
            b"[package]\nname = \"demo\"\nversion = \"0.1.0\"\n[dependencies]\nserde = \"1\"\n";
        assert_eq!(
            compression_file_type_for_path_and_data(Path::new("payload"), sample),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn samples_cargo_lock_before_falling_back_to_unknown() {
        let mut sample = Vec::new();
        for idx in 0..16 {
            sample.extend_from_slice(b"[[package]]\n");
            sample.extend_from_slice(format!("name = \"crate-{idx}\"\n").as_bytes());
            sample.extend_from_slice(b"version = \"1.0.0\"\n");
            sample.extend_from_slice(
                b"checksum = \"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\"\n",
            );
        }
        assert_eq!(
            compression_file_type_for_path_and_data(Path::new("payload"), &sample),
            CompressionFileType::DictionaryText
        );
    }

    #[test]
    fn samples_archive_signature_before_falling_back_to_unknown() {
        let sample = b"PK\x03\x04\x14\x00\x00\x00";
        assert_eq!(
            compression_file_type_for_path_and_data(Path::new("payload"), sample),
            CompressionFileType::ArchiveLike
        );
    }

    #[test]
    fn samples_binary_signature_before_falling_back_to_unknown() {
        let sample = b"\x7FELF\x02\x01\x01";
        assert_eq!(
            compression_file_type_for_path_and_data(Path::new("payload"), sample),
            CompressionFileType::BinaryLike
        );
    }

    #[test]
    fn plain_text_sample_can_stay_unknown() {
        let sample = b"this is a plain text blob without strong config or code markers\njust words and spaces repeated for a sample\n";
        assert_eq!(
            compression_file_type_for_path_and_data(Path::new("payload"), sample),
            CompressionFileType::Unknown
        );
    }

    #[test]
    fn known_path_wins_over_sample_fallback() {
        let sample = b"\x7FELF\x02\x01\x01";
        assert_eq!(
            compression_file_type_for_path_and_data(Path::new("config.yaml"), sample),
            CompressionFileType::ConfigText
        );
    }

    #[test]
    fn compression_level_equality_is_available_for_api_comparisons() {
        assert_eq!(CompressionLevel::Fastest, CompressionLevel::Fastest);
    }
}
