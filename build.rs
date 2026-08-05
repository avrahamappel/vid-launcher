/// Emit a hash of the thumbnails module as an environment variable, which
/// is in turn consumed by the thumbnails module for use as a cache key
/// for thumbnail storage.
///
/// This ensures that the thumbnail cache is invalidated whenever any of the
/// thumbnail generation logic changes.
fn main() {
    println!("cargo::rerun-if-changed=src/thumbnails.rs");

    let thumbnail_mod_hash = md5::compute(include_bytes!("src/thumbnails.rs"));

    // Only use the first 24 bits / 6 hex chars (easy to read)
    let thumbnail_cache_key = &format!("{thumbnail_mod_hash:x}")[..6];

    println!("cargo::rustc-env=THUMBNAIL_CACHE_KEY={thumbnail_cache_key}");
}
