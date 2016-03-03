fn main() {
    manage_docs();
}

#[cfg(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))]
fn manage_docs () {
    extern crate lgpl_docs;
    const PATH: &'static str = "src";
    const IGNORES: &'static [&'static str] = &[
        "event.rs",
        "lib.rs",
        "prelude.rs",
        "rt.rs",
    ];
    lgpl_docs::purge(PATH, IGNORES);
    if cfg!(feature = "embed-lgpl-docs") {
        lgpl_docs::embed(lgpl_docs::GDK_DOCS, PATH, IGNORES);
    }
}

#[cfg(not(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs")))]
fn manage_docs() { }
