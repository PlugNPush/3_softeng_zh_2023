static DIST_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../app/dist");

fn main() {
    // Ensure dist folder is present so the embedded frontend
    // doesn't fail the build.
    std::fs::create_dir_all(DIST_DIR).unwrap();
}
