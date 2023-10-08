use std::{collections::HashMap, path::PathBuf};

use mdbook::MDBook;

static BOOK_DIR: &str = env!("CARGO_MANIFEST_DIR");
static CONFIG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/book.toml");

fn main() {
    let mut config = mdbook::Config::from_disk(CONFIG_PATH).expect("should load config");

    // Running mdbook normally, we want the pdf backends to be enabled.
    // This ensures we don't forget to update the PDF when we update the documentation.
    // However, when building the documentation as a dependency of the backend,
    // we don't want to regenerate the PDF. Otherwise the PDF often gets regenerated
    // spuriously and it's annoying to have to `git reset` it every time.
    //
    // Therefore, we construct a new output table that only contains the HTML backend.
    let mut output_table = HashMap::new();
    output_table.insert("html", config.get("output.html").unwrap().clone());
    config.set("output", output_table).unwrap();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let build_dir = PathBuf::from(&out_dir).join("book");

    config.build.build_dir = build_dir.clone();

    // Somehow, it wasn't possible to write `#[folder = env!("OUT_DIR")]`
    // or similar directly in the `lib.rs` file, so we do a little detour.
    std::fs::write(
        out_dir + "/lib.rs",
        format!(
            r#"
            #[derive(rust_embed::RustEmbed)]
            #[folder = "{build_dir}"]
            pub struct EmbeddedDocs;
            "#,
            build_dir = build_dir.display()
        ),
    )
    .expect("should write lib.rs");

    let md = MDBook::load_with_config(BOOK_DIR, config).expect("should load book");
    md.build().expect("should build the book");
}
