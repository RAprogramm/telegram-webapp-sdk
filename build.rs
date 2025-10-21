// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

fn main() {
    println!("cargo:rustc-check-cfg=cfg(has_doc_cfg)");
    println!("cargo:rustc-check-cfg=cfg(has_doc_auto_cfg)");

    if version_check::supports_feature("doc_cfg").unwrap_or(false) {
        println!("cargo:rustc-cfg=has_doc_cfg");
    }

    if version_check::supports_feature("doc_auto_cfg").unwrap_or(false) {
        println!("cargo:rustc-cfg=has_doc_auto_cfg");
    }
}
