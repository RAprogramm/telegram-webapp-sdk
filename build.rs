fn main() {
    println!("cargo:rustc-check-cfg=cfg(has_doc_auto_cfg)");

    if version_check::Channel::read().is_some_and(|channel| channel.supports_features()) {
        println!("cargo:rustc-cfg=has_doc_auto_cfg");
    }
}
