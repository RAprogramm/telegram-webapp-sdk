// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Multilingual documentation site builder.
//!
//! Parses `wiki/_Sidebar.md` to discover languages and page order, generates
//! one mdBook per language, rewrites wiki-style links to book-relative links,
//! and assembles the final site with a landing page at the output root.
//!
//! Example: `cargo run -p xtask -- site --out "$RUNNER_TEMP/site"`.

use std::{collections::HashMap, fs, path::Path, process::Command};

use percent_encoding::percent_decode_str;
use regex::Regex;

use crate::{TaskError, io_error, repo_root};

const REPO_URL: &str = "https://github.com/RAprogramm/telegram-webapp-sdk";
const WIKI_URL: &str = "https://github.com/RAprogramm/telegram-webapp-sdk/wiki/";
const LANGUAGES: &[(&str, &str)] = &[("English", "en"), ("Русский", "ru")];

const HOME_PATTERN: &str = r"^\*\*\[(.+?)\]\((\S+?)\)\*\*$";
const PART_PATTERN: &str = r"^\*\*(.+?)\*\*$";
const PAGE_PATTERN: &str = r"^- \[(.+?)\]\((\S+?)\)$";
const HEADING_PATTERN: &str = r"^## (.+)$";
const LINK_PATTERN: &str = r"\]\(([^)\s]+)\)";

/// One sidebar entry: the home page, a part heading, or a regular page.
struct SidebarItem {
    kind:  ItemKind,
    title: String,
    slug:  Option<String>
}

/// Kind of a sidebar entry.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ItemKind {
    Home,
    Part,
    Page
}

/// Builds the documentation site into `out_dir` (default `target/site`).
///
/// Reads the page structure from `wiki/_Sidebar.md`, generates one mdBook per
/// language into a temporary directory, runs `mdbook build` for each book,
/// and copies the landing page and logo to the output root.
///
/// # Errors
/// Returns [`TaskError`] when the sidebar is incomplete, a file operation
/// fails, or `mdbook` is missing or fails.
pub fn run(out_dir: Option<&Path>) -> Result<(), TaskError> {
    let root = repo_root()?;
    let wiki_dir = root.join("wiki");
    let out_dir = match out_dir {
        Some(dir) => dir.to_path_buf(),
        None => root.join("target").join("site")
    };

    let sidebar = read_file(&wiki_dir.join("_Sidebar.md"))?;
    let books = parse_sidebar(&sidebar)?;
    let mut missing: Vec<&str> = LANGUAGES
        .iter()
        .filter(|(_name, code)| !books.contains_key(*code))
        .map(|(_name, code)| *code)
        .collect();
    missing.sort_unstable();
    if !missing.is_empty() {
        return Err(TaskError::LanguagesMissing {
            missing: missing.join(", ")
        });
    }

    let slug_lang = build_slug_index(&books);

    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).map_err(|error| io_error("remove", &out_dir, error))?;
    }
    fs::create_dir_all(&out_dir).map_err(|error| io_error("create", &out_dir, error))?;

    let work = tempfile::tempdir().map_err(TaskError::TempDir)?;
    for (lang, items) in &books {
        build_book(lang, items, &wiki_dir, work.path(), &out_dir, &slug_lang)?;
    }

    copy_file(
        &root.join("site").join("landing.html"),
        &out_dir.join("index.html")
    )?;
    copy_file(&root.join("tg-webapp-sdk.png"), &out_dir.join("logo.png"))?;
    println!("site built at {}", out_dir.display());
    Ok(())
}

/// Percent-decodes the last path segment of a wiki URL into a page slug.
fn slug_of(url: &str) -> Result<String, TaskError> {
    let last = url.rsplit('/').next().unwrap_or(url);
    percent_decode_str(last)
        .decode_utf8()
        .map(std::borrow::Cow::into_owned)
        .map_err(|error| TaskError::SlugDecode {
            value: last.to_string(),
            error
        })
}

/// Parses `wiki/_Sidebar.md` into ordered entries per language code.
fn parse_sidebar(text: &str) -> Result<HashMap<String, Vec<SidebarItem>>, TaskError> {
    let heading_re = build_regex(HEADING_PATTERN)?;
    let home_re = build_regex(HOME_PATTERN)?;
    let part_re = build_regex(PART_PATTERN)?;
    let page_re = build_regex(PAGE_PATTERN)?;

    let mut books: HashMap<String, Vec<SidebarItem>> = HashMap::new();
    let mut current: Option<&str> = None;
    for raw in text.lines() {
        let line = raw.trim();
        if let Some(caps) = heading_re.captures(line) {
            current = None;
            let heading = caps.get(1).map_or("", |m| m.as_str());
            for (name, code) in LANGUAGES {
                if heading.contains(name) {
                    current = Some(code);
                    books.entry((*code).to_string()).or_default();
                }
            }
            continue;
        }
        let Some(lang) = current else { continue };
        let lang = lang.to_string();
        if let Some(caps) = home_re.captures(line) {
            let slug = slug_of(caps.get(2).map_or("", |m| m.as_str()))?;
            push_item(&mut books, &lang, ItemKind::Home, &caps[1], Some(slug));
            continue;
        }
        if let Some(caps) = part_re.captures(line) {
            push_item(&mut books, &lang, ItemKind::Part, &caps[1], None);
            continue;
        }
        if let Some(caps) = page_re.captures(line) {
            let slug = slug_of(caps.get(2).map_or("", |m| m.as_str()))?;
            push_item(&mut books, &lang, ItemKind::Page, &caps[1], Some(slug));
        }
    }
    Ok(books)
}

fn push_item(
    books: &mut HashMap<String, Vec<SidebarItem>>,
    lang: &str,
    kind: ItemKind,
    title: &str,
    slug: Option<String>
) {
    books
        .entry(lang.to_string())
        .or_default()
        .push(SidebarItem {
            kind,
            title: title.to_string(),
            slug
        });
}

/// Maps every page slug to the language book it belongs to.
fn build_slug_index(books: &HashMap<String, Vec<SidebarItem>>) -> HashMap<String, String> {
    books
        .iter()
        .flat_map(|(lang, items)| {
            items
                .iter()
                .filter(|item| item.kind != ItemKind::Part)
                .filter_map(move |item| item.slug.clone().map(|slug| (slug, lang.clone())))
        })
        .collect()
}

/// Rewrites wiki links: same book becomes `slug.md`, other book becomes
/// `../lang/slug.html`; anything else is left untouched.
fn rewrite_links(
    text: &str,
    lang: &str,
    slug_lang: &HashMap<String, String>
) -> Result<String, TaskError> {
    let link_re = build_regex(LINK_PATTERN)?;
    let rewritten = link_re.replace_all(text, |caps: &regex::Captures| {
        let full = &caps[0];
        let target = &caps[1];
        let target = if let Some(rest) = target.strip_prefix(WIKI_URL) {
            rest
        } else if target.contains("://")
            || target.starts_with('#')
            || target.starts_with("mailto:")
        {
            return full.to_string();
        } else {
            target
        };
        let (slug_raw, anchor) = match target.split_once('#') {
            Some((slug, anchor)) => (slug, Some(anchor)),
            None => (target, None)
        };
        let Ok(slug) = slug_of(slug_raw) else {
            return full.to_string();
        };
        let Some(target_lang) = slug_lang.get(&slug) else {
            return full.to_string();
        };
        let suffix = anchor.map_or(String::new(), |anchor| format!("#{anchor}"));
        if target_lang == lang {
            format!("]({slug}.md{suffix})")
        } else {
            format!("](../{target_lang}/{slug}.html{suffix})")
        }
    });
    Ok(rewritten.into_owned())
}

/// Renders the `book.toml` contents for one language book.
fn book_toml(lang: &str) -> String {
    format!(
        "[book]\ntitle = \"telegram-webapp-sdk\"\nlanguage = \"{lang}\"\nsrc = \"src\"\n\n\
         [output.html]\nsite-url = \"/telegram-webapp-sdk/{lang}/\"\n\
         git-repository-url = \"{REPO_URL}\"\n\
         edit-url-template = \"{REPO_URL}/edit/main/wiki/{{path}}\"\n\n\
         [output.html.playground]\nrunnable = false\n"
    )
}

/// Generates one mdBook (SUMMARY, pages with rewritten links, book.toml) in
/// `work_dir/<lang>` and builds it into `out_dir/<lang>`.
fn build_book(
    lang: &str,
    items: &[SidebarItem],
    wiki_dir: &Path,
    work_dir: &Path,
    out_dir: &Path,
    slug_lang: &HashMap<String, String>
) -> Result<(), TaskError> {
    let book_dir = work_dir.join(lang);
    let src_dir = book_dir.join("src");
    fs::create_dir_all(&src_dir).map_err(|error| io_error("create", &src_dir, error))?;

    let mut summary = String::from("# Summary\n\n");
    for item in items {
        match item.kind {
            ItemKind::Home => {
                let slug = item.slug.as_deref().unwrap_or_default();
                summary.push_str(&format!("[{}]({slug}.md)\n\n", item.title));
            }
            ItemKind::Part => summary.push_str(&format!("# {}\n\n", item.title)),
            ItemKind::Page => {
                let slug = item.slug.as_deref().unwrap_or_default();
                summary.push_str(&format!("- [{}]({slug}.md)\n", item.title));
            }
        }
    }
    write_file(&src_dir.join("SUMMARY.md"), &summary)?;

    for item in items {
        let Some(slug) = item.slug.as_deref() else {
            continue;
        };
        let text = read_file(&wiki_dir.join(slug.to_owned() + ".md"))?;
        let rewritten = rewrite_links(&text, lang, slug_lang)?;
        write_file(&src_dir.join(slug.to_owned() + ".md"), &rewritten)?;
    }

    write_file(&book_dir.join("book.toml"), &book_toml(lang))?;

    let status = Command::new("mdbook")
        .arg("build")
        .arg("--dest-dir")
        .arg(out_dir.join(lang))
        .current_dir(&book_dir)
        .spawn()
        .map_err(|error| TaskError::MdbookSpawn {
            error
        })?
        .wait()
        .map_err(|error| TaskError::MdbookSpawn {
            error
        })?;
    if !status.success() {
        return Err(TaskError::MdbookFailed {
            lang:   lang.to_string(),
            status: status.to_string()
        });
    }
    Ok(())
}

fn build_regex(pattern: &str) -> Result<Regex, TaskError> {
    Regex::new(pattern).map_err(|error| TaskError::RegexBuild {
        pattern: pattern.to_string(),
        error
    })
}

fn read_file(path: &Path) -> Result<String, TaskError> {
    fs::read_to_string(path).map_err(|error| io_error("read", path, error))
}

fn write_file(path: &Path, contents: &str) -> Result<(), TaskError> {
    fs::write(path, contents).map_err(|error| io_error("write", path, error))
}

fn copy_file(from: &Path, to: &Path) -> Result<(), TaskError> {
    fs::copy(from, to)
        .map_err(|error| TaskError::Io {
            action: format!("copy `{}`", from.display()),
            path: to.display().to_string(),
            error
        })
        .map(|_| ())
}
