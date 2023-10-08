use mdbook::MDBook;

static BOOK_DIR: &str = env!("CARGO_MANIFEST_DIR");
static CONFIG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/book.toml");

fn main() {
    let mut config = mdbook::Config::from_disk(CONFIG_PATH).expect("should load config");

    // Running mdbook normally, we want the pdf-copy backend to be enabled.
    // This ensures we don't forget to update the PDF when we update the documentation.
    // However, when building the documentation as a dependency of the backend,
    // we don't want to regenerate the PDF. Otherwise the PDF often gets regenerated
    // spuriously and it's annoying to have to `git reset` it every time.
    config
        .set("output.pdf-copy.command", "echo")
        .expect("should disable pdf-copy backend");
    
    let md = MDBook::load_with_config(BOOK_DIR, config).expect("should load book");
    md.build().expect("should build the book");
}
