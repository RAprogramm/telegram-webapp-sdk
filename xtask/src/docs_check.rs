// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Documentation conformance check: every SDK API path referenced in prose
//! must exist in the crate, and the EN/RU wiki must stay symmetric.
//!
//! Prose drifts faster than code: a renamed or removed item keeps being
//! documented until someone notices. This task builds the public API index
//! from `src/` and fails when a Rust code block in `README.md`, `llms.txt`,
//! `wiki/` or `skills/` mentions a `module::path` that the crate does not
//! export. It also verifies that `wiki/_Sidebar.md` links every wiki file and
//! keeps the English and Russian sections balanced.

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf}
};

use regex::Regex;

use crate::{TaskError, io_error, repo_root};

/// Roots that may appear un-prefixed in documentation code blocks.
/// `core`, `yew` and `leptos` are deliberately absent: they collide with
/// `core::` from the standard library and with the real framework crates.
const CHECKED_ROOTS: &[&str] = &[
    "api", "dom", "logger", "mock", "pages", "router", "utils", "webapp"
];

/// Crate names accepted as the first segment of a fully qualified path.
const CRATE_ROOTS: &[&str] = &["telegram_webapp_sdk"];

/// Public modules and items of the crate, keyed by `::`-separated paths
/// relative to the crate root.
#[derive(Default)]
struct ApiIndex {
    modules: HashSet<String>,
    items:   HashSet<String>
}

impl ApiIndex {
    /// Reports whether `path` resolves to a known module, a known item, or an
    /// associated member of a known item (methods are not indexed).
    fn resolves(&self, path: &str) -> bool {
        if self.modules.contains(path) || self.items.contains(path) {
            return true;
        }
        let mut prefix = String::new();
        for segment in path.split("::") {
            if !prefix.is_empty() {
                prefix.push_str("::");
            }
            prefix.push_str(segment);
            if self.items.contains(&prefix) {
                return true;
            }
        }
        false
    }
}

/// Runs the documentation conformance check.
///
/// # Errors
/// Returns [`TaskError`] when a source file cannot be read, an API path in
/// prose does not exist, or the wiki sidebar is asymmetric.
pub fn run() -> Result<(), TaskError> {
    let root = repo_root()?;
    let index = build_api_index(&root.join("src"))?;

    let mut docs: Vec<PathBuf> = Vec::new();
    for candidate in ["README.md", "llms.txt"] {
        let path = root.join(candidate);
        if path.is_file() {
            docs.push(path);
        }
    }
    docs.extend(md_files(&root.join("wiki"))?);
    docs.extend(md_files(&root.join("skills"))?);

    let mut unknown: Vec<String> = Vec::new();
    for doc in &docs {
        unknown.extend(unknown_paths_in(doc, &index)?);
    }
    if !unknown.is_empty() {
        for entry in &unknown {
            eprintln!("docs-check: {entry}");
        }
        return Err(TaskError::DocsApiDrift {
            missing: format!("{} unknown API path(s), see above", unknown.len())
        });
    }

    let sidebar_issues =
        check_sidebar(&root.join("wiki").join("_Sidebar.md"), &root.join("wiki"))?;
    if !sidebar_issues.is_empty() {
        for entry in &sidebar_issues {
            eprintln!("docs-check: {entry}");
        }
        return Err(TaskError::DocsSidebar {
            detail: format!("{} sidebar issue(s), see above", sidebar_issues.len())
        });
    }

    println!(
        "docs-check: {} API paths referenced by docs all exist; wiki sidebar is symmetric",
        docs.len()
    );
    Ok(())
}

/// Recursively collects `.rs` files under `dir`.
fn rust_files(dir: &Path) -> Result<Vec<PathBuf>, TaskError> {
    let mut out = Vec::new();
    let entries = fs::read_dir(dir).map_err(|e| io_error("reading", dir, e))?;
    for entry in entries {
        let path = entry
            .map_err(|e| io_error("reading entry in", dir, e))?
            .path();
        if path.is_dir() {
            out.extend(rust_files(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
    Ok(out)
}

/// Recursively collects `.md` files under `dir`.
fn md_files(dir: &Path) -> Result<Vec<PathBuf>, TaskError> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(out);
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(md_files(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            out.push(path);
        }
    }
    Ok(out)
}

/// Builds the public API index by scanning `pub` declarations and re-exports.
fn build_api_index(src: &Path) -> Result<ApiIndex, TaskError> {
    let decl = Regex::new(
        r"(?m)^[ \t]*pub[ \t]+(?:default[ \t]+)?(?:async[ \t]+)?(?:const|static|fn|struct|enum|union|trait|type)[ \t]+([A-Za-z_]\w*)"
    )
    .map_err(|e| TaskError::RegexBuild {
        pattern: "api declaration".to_string(),
        error:   e
    })?;
    let mod_decl = Regex::new(r"(?m)^[ \t]*pub[ \t]+mod[ \t]+([A-Za-z_]\w*)").map_err(|e| {
        TaskError::RegexBuild {
            pattern: "module declaration".to_string(),
            error:   e
        }
    })?;
    let reexport = Regex::new(r"(?m)^[ \t]*pub[ \t]+use[ \t]+([^;]+);").map_err(|e| {
        TaskError::RegexBuild {
            pattern: "re-export".to_string(),
            error:   e
        }
    })?;

    let mut index = ApiIndex::default();
    for file in rust_files(src)? {
        let module = module_path(src, &file)?;
        let text = fs::read_to_string(&file).map_err(|e| io_error("reading", &file, e))?;
        for cap in decl.captures_iter(&text) {
            index.items.insert(qualify(&module, &cap[1]));
        }
        for cap in mod_decl.captures_iter(&text) {
            index.modules.insert(qualify(&module, &cap[1]));
        }
        for cap in reexport.captures_iter(&text) {
            for name in reexported_names(&cap[1]) {
                index.items.insert(qualify(&module, &name));
            }
        }
    }
    Ok(index)
}

/// Derives the module path (relative to the crate root) of a source file.
fn module_path(src: &Path, file: &Path) -> Result<String, TaskError> {
    let rel = file
        .strip_prefix(src)
        .map_err(|e| io_error("relativising", file, std::io::Error::other(e)))?;
    let mut segments: Vec<String> = rel
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    let stem = file
        .file_stem()
        .map_or_default(|s| s.to_string_lossy().into_owned());
    if stem != "mod" {
        segments.push(stem);
    }
    segments.retain(|s| s != "lib" && !s.is_empty());
    Ok(segments.join("::"))
}

fn qualify(module: &str, name: &str) -> String {
    if module.is_empty() {
        name.to_string()
    } else {
        format!("{module}::{name}")
    }
}

fn strip_crate_root(path: &str) -> String {
    for root in CRATE_ROOTS {
        let prefix = format!("{root}::");
        if let Some(rest) = path.strip_prefix(&prefix) {
            return rest.to_string();
        }
    }
    path.to_string()
}

/// Extracts the names a `pub use` statement introduces into its module.
///
/// Globs contribute nothing (their contents are not resolved); brace groups
/// expand to one name per entry; `as` renames win over the original segment.
fn reexported_names(statement: &str) -> Vec<String> {
    let text = statement.trim();
    let (path_part, alias) = match text.split_once(" as ") {
        Some((path, name)) => (path.trim(), Some(name.trim().to_string())),
        None => (text, None)
    };
    if path_part.ends_with("::*") {
        return Vec::new();
    }
    let path_part = path_part.strip_prefix("crate::").unwrap_or(path_part);
    let path_part = strip_crate_root(path_part);
    if let Some(brace_pos) = path_part.find('{') {
        let inner = path_part[brace_pos + 1..].trim_end_matches('}');
        return inner
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty() && !item.ends_with("::*"))
            .map(|item| match item.split_once(" as ") {
                Some((_, name)) => name.trim().to_string(),
                None => item.strip_prefix("self::").unwrap_or(item).to_string()
            })
            .collect();
    }
    let last = path_part.rsplit("::").next().unwrap_or_default();
    if last.is_empty() || last == "self" {
        return Vec::new();
    }
    vec![alias.unwrap_or_else(|| last.to_string())]
}

/// Reports `file:line: unknown API path ...` for every `::` path in Rust
/// code blocks of one document that the index cannot resolve.
fn unknown_paths_in(doc: &Path, index: &ApiIndex) -> Result<Vec<String>, TaskError> {
    let text = fs::read_to_string(doc).map_err(|e| io_error("reading", doc, e))?;
    let path_re = Regex::new(r"\b\w+(?:::\w+)+").map_err(|e| TaskError::RegexBuild {
        pattern: "qualified path".to_string(),
        error:   e
    })?;

    let mut issues = Vec::new();
    let mut in_rust_block = false;
    for (line_no, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("```") {
            let info = rest.split(',').next().unwrap_or("").trim();
            in_rust_block = matches!(info, "rust" | "no_run" | "ignore" | "should_panic");
            continue;
        }
        if !in_rust_block {
            continue;
        }
        for cap in path_re.find_iter(line) {
            let found = cap.as_str();
            let first = found.split("::").next().unwrap_or_default();
            let candidate = if CRATE_ROOTS.contains(&first) {
                strip_crate_root(found)
            } else if CHECKED_ROOTS.contains(&first) {
                found.to_string()
            } else {
                continue;
            };
            if !index.resolves(&candidate) {
                let name = doc
                    .file_name()
                    .map_or_else(|| found.to_string(), |n| n.to_string_lossy().into_owned());
                issues.push(format!(
                    "{name}:{}: unknown API path `{found}`",
                    line_no + 1
                ));
            }
        }
    }
    Ok(issues)
}

/// Verifies the wiki sidebar: balanced EN/RU link counts and no orphan pages.
fn check_sidebar(sidebar: &Path, wiki: &Path) -> Result<Vec<String>, TaskError> {
    let text = fs::read_to_string(sidebar).map_err(|e| io_error("reading", sidebar, e))?;
    let link_re = Regex::new(r"\]\([^)]*wiki/([^)]+)\)").map_err(|e| TaskError::RegexBuild {
        pattern: "sidebar link".to_string(),
        error:   e
    })?;

    let en_start = text.find("## 🇬🇧 English").unwrap_or_default();
    let ru_start = text.find("## 🇷🇺 Русский").unwrap_or(text.len());
    let ref_start = text.find("📚 Reference").unwrap_or(text.len());
    let en_links: Vec<String> = link_re
        .captures_iter(&text[en_start..ru_start])
        .map(|c| percent_decode(&c[1]))
        .collect();
    let ru_links: Vec<String> = link_re
        .captures_iter(&text[ru_start..ref_start])
        .map(|c| percent_decode(&c[1]))
        .collect();

    let mut issues = Vec::new();
    if en_links.len() != ru_links.len() {
        issues.push(format!(
            "_Sidebar.md links {} English pages but {} Russian pages",
            en_links.len(),
            ru_links.len()
        ));
    }

    let linked: HashSet<String> = en_links
        .iter()
        .chain(ru_links.iter())
        .map(|slug| format!("{slug}.md"))
        .collect();
    for page in md_files(wiki)? {
        let name = page
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
        if name != "_Sidebar.md" && !linked.contains(&name) {
            issues.push(format!("{name} is not linked from _Sidebar.md"));
        }
    }
    Ok(issues)
}

fn percent_decode(slug: &str) -> String {
    percent_encoding::percent_decode_str(slug)
        .decode_utf8_lossy()
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_resolves_exact_and_associated_paths() {
        let mut index = ApiIndex::default();
        index.modules.insert("api::device_storage".to_string());
        index.items.insert("webapp::TelegramWebApp".to_string());
        assert!(index.resolves("api::device_storage"));
        assert!(index.resolves("webapp::TelegramWebApp"));
        assert!(index.resolves("webapp::TelegramWebApp::send_data"));
        assert!(!index.resolves("api::device_storage::share_url"));
        assert!(!index.resolves("webapp::Missing"));
    }

    #[test]
    fn module_path_maps_files_to_crate_relative_modules() {
        let src = Path::new("/tmp/x/src");
        let file = Path::new("/tmp/x/src/api/device_storage.rs");
        assert_eq!(module_path(src, file).unwrap(), "api::device_storage");
        let lib = Path::new("/tmp/x/src/lib.rs");
        assert_eq!(module_path(src, lib).unwrap(), "");
    }

    #[test]
    fn strip_crate_root_only_touches_crate_names() {
        assert_eq!(strip_crate_root("telegram_webapp_sdk::api::x"), "api::x");
        assert_eq!(
            strip_crate_root("std::process::Command"),
            "std::process::Command"
        );
    }

    #[test]
    fn reexport_names_expand_braces_and_renames_and_skip_globs() {
        assert_eq!(
            reexported_names("webapp::TelegramWebApp"),
            ["TelegramWebApp"]
        );
        assert_eq!(
            reexported_names("theme::{ThemeState, use_theme}"),
            ["ThemeState", "use_theme"]
        );
        assert_eq!(reexported_names("foo::Bar as Baz"), ["Baz"]);
        assert_eq!(reexported_names("crate::macros::*"), [] as [String; 0]);
        assert_eq!(reexported_names("self"), [] as [String; 0]);
    }

    fn write_fixture(dir: &Path, rel: &str, body: &str) {
        let path = dir.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    }

    #[test]
    fn build_api_index_collects_declarations_modules_and_reexports() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        write_fixture(
            &src,
            "lib.rs",
            "pub mod api;\npub mod webapp;\npub use webapp::TelegramWebApp;\n"
        );
        write_fixture(
            &src,
            "api.rs",
            "pub mod sub;\npub async fn call() {}\nfn hidden() {}\n"
        );
        write_fixture(&src, "api/sub.rs", "pub struct Nested;\n");
        write_fixture(
            &src,
            "webapp.rs",
            "pub mod types;\npub use types::{Thing};\n"
        );
        write_fixture(&src, "webapp/types.rs", "pub struct Thing;\n");

        let index = build_api_index(&src).unwrap();
        assert!(index.items.contains("api::call"));
        assert!(index.items.contains("api::sub::Nested"));
        assert!(index.modules.contains("api::sub"));
        assert!(index.modules.contains("webapp::types"));
        assert!(index.items.contains("webapp::Thing"));
        assert!(index.items.contains("TelegramWebApp"));
        assert!(!index.items.contains("api::hidden"));
    }

    #[test]
    fn unknown_paths_flags_missing_and_ignores_valid_and_foreign() {
        let tmp = tempfile::tempdir().unwrap();
        let mut index = ApiIndex::default();
        index.modules.insert("api".to_string());
        index.items.insert("api::call".to_string());
        index.items.insert("webapp::TelegramWebApp".to_string());
        let doc = tmp.path().join("doc.md");
        fs::write(
            &doc,
            concat!(
                "# Title\n\n",
                "```rust\n",
                "use telegram_webapp_sdk::api::call;\n",
                "use telegram_webapp_sdk::api::ghost;\n",
                "let v = webapp::TelegramWebApp::send_data();\n",
                "let g = webapp::Ghost::new();\n",
                "```\n",
                "```bash\n",
                "webapp::NotCounted\n",
                "```\n"
            )
        )
        .unwrap();

        let issues = unknown_paths_in(&doc, &index).unwrap();
        assert_eq!(issues.len(), 2);
        assert!(issues[0].contains("api::ghost"));
        assert!(issues[0].contains(":5:"));
        assert!(issues[1].contains("webapp::Ghost"));
        assert!(issues[1].contains(":7:"));
    }

    #[test]
    fn sidebar_accepts_balanced_sections_with_all_pages_linked() {
        let tmp = tempfile::tempdir().unwrap();
        let wiki = tmp.path();
        fs::write(wiki.join("Home-en.md"), "# Home\n").unwrap();
        fs::write(wiki.join("Главная.md"), "# Главная\n").unwrap();
        fs::write(
            wiki.join("_Sidebar.md"),
            concat!(
                "## 🇬🇧 English\n\n- [Home](https://x/wiki/Home-en)\n\n",
                "## 🇷🇺 Русский\n\n- [Главная](https://x/wiki/%D0%93%D0%BB%D0%B0%D0%B2%D0%BD%D0%B0%D1%8F)\n\n",
                "## 📚 Reference\n"
            )
        )
        .unwrap();

        assert_eq!(
            check_sidebar(&wiki.join("_Sidebar.md"), wiki).unwrap(),
            [] as [String; 0]
        );
    }

    #[test]
    fn sidebar_reports_asymmetry_and_orphan_pages() {
        let tmp = tempfile::tempdir().unwrap();
        let wiki = tmp.path();
        fs::write(wiki.join("Home-en.md"), "# Home\n").unwrap();
        fs::write(wiki.join("Extra-en.md"), "# Extra\n").unwrap();
        fs::write(wiki.join("Orphan-en.md"), "# Orphan\n").unwrap();
        fs::write(wiki.join("Главная.md"), "# Главная\n").unwrap();
        fs::write(
            wiki.join("_Sidebar.md"),
            concat!(
                "## 🇬🇧 English\n\n- [Home](https://x/wiki/Home-en)\n",
                "- [Extra](https://x/wiki/Extra-en)\n\n",
                "## 🇷🇺 Русский\n\n- [Главная](https://x/wiki/%D0%93%D0%BB%D0%B0%D0%B2%D0%BD%D0%B0%D1%8F)\n\n",
                "## 📚 Reference\n"
            )
        )
        .unwrap();

        let issues = check_sidebar(&wiki.join("_Sidebar.md"), wiki).unwrap();
        assert!(
            issues
                .iter()
                .any(|i| i.contains("2 English pages but 1 Russian"))
        );
        assert!(
            issues
                .iter()
                .any(|i| i.contains("Orphan-en.md is not linked"))
        );
    }

    #[test]
    fn run_passes_on_the_real_repository() {
        run().unwrap();
    }
}
