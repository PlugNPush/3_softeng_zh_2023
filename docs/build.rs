use mdbook::MDBook;

fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let md = MDBook::load(dir).expect("should load the book");
    md.build().expect("should build the book");
}
