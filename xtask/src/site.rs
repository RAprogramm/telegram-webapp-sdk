// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Multilingual documentation site builder.
//!
//! Parses `wiki/_Sidebar.md` to discover languages and page order, generates
//! one mdBook per language, rewrites wiki-style links to book-relative links,
//! and assembles the final site with a landing page at the output root.
//!
//! Example: `cargo run -p xtask -- site --out "$RUNNER_TEMP/site"`.

use std::{collections::HashMap, fmt::Write as _, fs, path::Path, process::Command};

use percent_encoding::percent_decode_str;
use regex::Regex;

use crate::{TaskError, io_error, repo_root};

const REPO_URL: &str = "https://github.com/RAprogramm/telegram-webapp-sdk";
const WIKI_URL: &str = "https://github.com/RAprogramm/telegram-webapp-sdk/wiki/";
const SITE_BASE_URL: &str = "https://raprogramm.github.io/telegram-webapp-sdk/";
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
    let out_dir = out_dir.map_or_else(|| root.join("target").join("site"), Path::to_path_buf);

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

    write_file(&out_dir.join("llms.txt"), &llms_txt(&books))?;
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

/// Renders the `llms.txt` index of the built site in the llmstxt.org format:
/// project summary, one section per language book, and optional links.
///
/// # Examples
/// ```text
/// # telegram-webapp-sdk
/// > Rust SDK for building Telegram Mini Apps ...
/// ## Documentation (English)
/// - [Home](https://raprogramm.github.io/telegram-webapp-sdk/en/Home-en.html)
/// ```
fn llms_txt(books: &HashMap<String, Vec<SidebarItem>>) -> String {
    let mut text = String::from(
        "# telegram-webapp-sdk\n\n> Rust SDK for building Telegram Mini Apps (WebApps) \
         targeting wasm32-unknown-unknown: full `Telegram.WebApp` JS API bindings, Yew and \
         Leptos integrations, routing macros, a mock runtime for local development, and \
         typed errors that explain how to recover.\n\n"
    );
    for (name, code) in LANGUAGES {
        let Some(items) = books.get(*code) else {
            continue;
        };
        let _ = write!(text, "## Documentation ({name})\n\n");
        for item in items {
            if item.kind == ItemKind::Part {
                continue;
            }
            let Some(slug) = item.slug.as_deref() else {
                continue;
            };
            let _ = writeln!(
                text,
                "- [{}]({SITE_BASE_URL}{code}/{slug}.html)",
                item.title
            );
        }
        text.push('\n');
    }
    text.push_str(
        "## Optional\n\n\
         - [API reference](https://docs.rs/telegram-webapp-sdk)\n\
         - [Crate page](https://crates.io/crates/telegram-webapp-sdk)\n\
         - [Source repository]({REPO_URL})\n\
         - [WebApp API coverage checklist]({REPO_URL}/blob/main/WEBAPP_API.md)\n"
    );
    text
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
                let _ = write!(summary, "[{}]({slug}.md)\n\n", item.title);
            }
            ItemKind::Part => {
                let _ = write!(summary, "# {}\n\n", item.title);
            }
            ItemKind::Page => {
                let slug = item.slug.as_deref().unwrap_or_default();
                let _ = writeln!(summary, "- [{}]({slug}.md)", item.title);
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{TaskError, build_slug_index, llms_txt, parse_sidebar, rewrite_links, slug_of};

    const SIDEBAR: &str = "## 🌐 Language\n\n\
        [🇬🇧 English](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/Home-en) | \
        [🇷🇺 Русский](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/Главная)\n\n\
        ---\n\n## 🇬🇧 English\n\n\
        **[Home](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/Home-en)**\n\n\
        **Getting Started**\n\
        - [Installation](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/Installation-en)\n\n\
        ---\n\n## 🇷🇺 Русский\n\n\
        **[Главная](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/Главная)**\n";

    #[test]
    fn parse_sidebar_discovers_books_and_items() {
        let books = parse_sidebar(SIDEBAR).expect("sidebar parses");
        let english = books.get("en").expect("english book present");
        assert_eq!(english.len(), 3);
        assert_eq!(english[0].title, "Home");
        assert_eq!(english[0].slug.as_deref(), Some("Home-en"));
        assert_eq!(english[1].title, "Getting Started");
        assert_eq!(english[1].slug, None);
        assert_eq!(english[2].slug.as_deref(), Some("Installation-en"));
        let russian = books.get("ru").expect("russian book present");
        assert_eq!(russian[0].slug.as_deref(), Some("Главная"));
    }

    #[test]
    fn parse_sidebar_without_language_sections_yields_nothing() {
        let books = parse_sidebar("## 🌐 Language\n\n- [x](y)\n").expect("parses");
        assert!(books.is_empty());
    }

    #[test]
    fn slug_of_decodes_percent_escapes_and_rejects_invalid_utf8() {
        let decoded = slug_of("https://host/wiki/%D0%93%D0%BB%D0%B0%D0%B2%D0%BD%D0%B0%D1%8F")
            .expect("valid utf-8");
        assert_eq!(decoded, "Главная");
        assert!(matches!(
            slug_of("https://host/wiki/%FF"),
            Err(TaskError::SlugDecode { .. })
        ));
    }

    #[test]
    fn rewrite_links_rewrites_known_and_keeps_unknown_targets() {
        let books = parse_sidebar(SIDEBAR).expect("sidebar parses");
        let slug_lang = build_slug_index(&books);
        let text = "[en](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/Installation-en) \
                    [anchor](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/Home-en#top) \
                    [ru](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/Главная) \
                    [ext](https://example.com/x) [rel](unknown-page)";
        let rewritten = rewrite_links(text, "en", &slug_lang).expect("regex builds");
        assert!(rewritten.contains("](Installation-en.md)"));
        assert!(rewritten.contains("](Home-en.md#top)"));
        assert!(rewritten.contains("](../ru/Главная.html)"));
        assert!(rewritten.contains("](https://example.com/x)"));
        assert!(rewritten.contains("](unknown-page)"));
    }

    #[test]
    fn llms_txt_lists_pages_per_language_and_optional_links() {
        let books = parse_sidebar(SIDEBAR).expect("sidebar parses");
        let text = llms_txt(&books);
        assert!(text.starts_with("# telegram-webapp-sdk\n"));
        assert!(text.contains("## Documentation (English)"));
        assert!(text.contains(
            "- [Home](https://raprogramm.github.io/telegram-webapp-sdk/en/Home-en.html)"
        ));
        assert!(text.contains("## Documentation (Русский)"));
        assert!(text.contains(
            "- [Главная](https://raprogramm.github.io/telegram-webapp-sdk/ru/Главная.html)"
        ));
        assert!(text.contains("## Optional"));
        assert!(!text.contains("Getting Started"));
    }

    #[test]
    fn llms_txt_without_books_omits_language_sections() {
        let text = llms_txt(&HashMap::new());
        assert!(!text.contains("## Documentation"));
        assert!(text.contains("## Optional"));
    }
}
