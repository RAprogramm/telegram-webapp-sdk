#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
# SPDX-License-Identifier: MIT

"""Build the multilingual documentation site from wiki/ with mdBook.

Parses wiki/_Sidebar.md to discover languages and page order, generates one
mdBook per language, rewrites wiki-style links to book-relative links, and
assembles the final site with a landing page at the output root.

Usage: python3 site/build.py --out <output-directory>
"""

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from urllib.parse import unquote

REPO_URL = "https://github.com/RAprogramm/telegram-webapp-sdk"
WIKI_URL = REPO_URL + "/wiki/"
LANGUAGES = {
    "English": "en",
    "Русский": "ru",
}

HOME_RE = re.compile(r"^\*\*\[(.+?)\]\((\S+?)\)\*\*$")
PART_RE = re.compile(r"^\*\*(.+?)\*\*$")
PAGE_RE = re.compile(r"^- \[(.+?)\]\((\S+?)\)$")
HEADING_RE = re.compile(r"^## (.+)$")
LINK_RE = re.compile(r"\]\(([^)\s]+)\)")


def slug_of(url):
    return unquote(url.rsplit("/", 1)[-1])


def parse_sidebar(text):
    """Return {lang: [(kind, title, slug?), ...]} in sidebar order."""
    books = {}
    current = None
    for raw in text.splitlines():
        line = raw.strip()
        heading = HEADING_RE.match(line)
        if heading:
            current = None
            for name, code in LANGUAGES.items():
                if name in heading.group(1):
                    current = code
                    books[code] = []
            continue
        if current is None:
            continue
        m = HOME_RE.match(line)
        if m:
            books[current].append(("home", m.group(1), slug_of(m.group(2))))
            continue
        m = PART_RE.match(line)
        if m:
            books[current].append(("part", m.group(1), None))
            continue
        m = PAGE_RE.match(line)
        if m:
            books[current].append(("page", m.group(1), slug_of(m.group(2))))
    return books


def rewrite_links(text, lang, slug_lang):
    """Rewrite wiki links: same book -> page.md, other book -> ../lang/page.html."""

    def repl(match):
        target = match.group(1)
        if target.startswith(WIKI_URL):
            target = target[len(WIKI_URL):]
        elif "://" in target or target.startswith(("#", "mailto:")):
            return match.group(0)
        slug, _, anchor = target.partition("#")
        slug = unquote(slug)
        if slug not in slug_lang:
            return match.group(0)
        suffix = "#" + anchor if anchor else ""
        if slug_lang[slug] == lang:
            return "](" + slug + ".md" + suffix + ")"
        return "](../" + slug_lang[slug] + "/" + slug + ".html" + suffix + ")"

    return LINK_RE.sub(repl, text)


def book_toml(lang):
    return (
        "[book]\n"
        'title = "telegram-webapp-sdk"\n'
        'language = "' + lang + '"\n'
        'src = "src"\n'
        "\n"
        "[output.html]\n"
        'site-url = "/telegram-webapp-sdk/' + lang + '/"\n'
        'git-repository-url = "' + REPO_URL + '"\n'
        'edit-url-template = "' + REPO_URL + '/edit/main/wiki/{path}"\n'
        "\n"
        "[output.html.playground]\n"
        "runnable = false\n"
    )


def build_book(lang, items, wiki_dir, work_dir, out_dir, slug_lang):
    book_dir = work_dir / lang
    src_dir = book_dir / "src"
    src_dir.mkdir(parents=True)
    summary = ["# Summary", ""]
    for kind, title, slug in items:
        if kind == "home":
            summary += ["[" + title + "](" + slug + ".md)", ""]
        elif kind == "part":
            summary += ["# " + title, ""]
        else:
            summary.append("- [" + title + "](" + slug + ".md)")
    (src_dir / "SUMMARY.md").write_text("\n".join(summary) + "\n", encoding="utf-8")
    for kind, _title, slug in items:
        if kind == "part":
            continue
        text = (wiki_dir / (slug + ".md")).read_text(encoding="utf-8")
        (src_dir / (slug + ".md")).write_text(
            rewrite_links(text, lang, slug_lang), encoding="utf-8"
        )
    (book_dir / "book.toml").write_text(book_toml(lang), encoding="utf-8")
    subprocess.run(
        ["mdbook", "build", "--dest-dir", str(out_dir / lang)],
        cwd=book_dir,
        check=True,
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent
    wiki_dir = repo_root / "wiki"
    out_dir = args.out.resolve()

    books = parse_sidebar((wiki_dir / "_Sidebar.md").read_text(encoding="utf-8"))
    missing = sorted(set(LANGUAGES.values()) - set(books))
    if missing:
        sys.exit("languages missing from wiki/_Sidebar.md: " + ", ".join(missing))

    slug_lang = {
        slug: lang
        for lang, items in books.items()
        for kind, _title, slug in items
        if kind != "part"
    }

    if out_dir.exists():
        shutil.rmtree(out_dir)
    out_dir.mkdir(parents=True)

    with tempfile.TemporaryDirectory() as work:
        for lang, items in books.items():
            build_book(lang, items, wiki_dir, Path(work), out_dir, slug_lang)

    shutil.copy(repo_root / "site" / "landing.html", out_dir / "index.html")
    shutil.copy(repo_root / "tg-webapp-sdk.png", out_dir / "logo.png")
    print("site built at", out_dir)


if __name__ == "__main__":
    main()
