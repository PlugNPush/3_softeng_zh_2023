use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "book/html"]
pub struct EmbeddedDocs;
