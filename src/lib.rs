use std::{
    hash::{BuildHasher, Hash, Hasher},
    path::Path,
};

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Clone)]
pub struct XXH3Hasher(u64);
impl BuildHasher for XXH3Hasher {
    type Hasher = XXH3Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.clone()
    }
}
impl Hasher for XXH3Hasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        self.0 = xxh3::hash64_with_seed(bytes, self.0);
    }
}
fn hashmap_from<K: Copy + Eq + Hash, V: Copy>(d: &[(K, V)]) -> HashMap<K, V, XXH3Hasher> {
    let mut m = HashMap::with_capacity_and_hasher(d.len(), XXH3Hasher(42));
    for (k, v) in d {
        m.insert(*k, *v);
    }
    m
}
pub static EXT_BY_MIME: Lazy<HashMap<&'static str, &'static [&'static str], XXH3Hasher>> =
    Lazy::new(|| hashmap_from(&EXT_BY_MIME_DATA));
pub static MIME_BY_EXT: Lazy<HashMap<&'static str, &'static str, XXH3Hasher>> =
    Lazy::new(|| hashmap_from(&MIME_BY_EXT_DATA));

pub fn get_extension(name: &str) -> Option<&[&str]> {
    gen_get_extension(name)
}

pub fn get_mime_type(ext: &str) -> Option<&str> {
    gen_get_mime_type(ext)
}

pub fn mime_for_path(path: &Path) -> Option<&str> {
    path.extension()
        .and_then(|s| s.to_str())
        .and_then(|ext| get_mime_type(ext))
}

#[cfg(test)]
mod test {
    use crate::{get_extension, get_mime_type, mime_for_path};
    use std::path::Path;

    #[test]
    fn test_by_ext() {
        assert_eq!(get_extension("text/css").unwrap(), ["css"]);
    }

    #[test]
    fn test_by_type() {
        assert_eq!(get_mime_type("css"), Some("text/css"));
    }

    #[test]
    fn test_by_path() {
        test_path_none("foo");
        test_path_none("/path/to/foo");
        test_path("foo.css", "text/css");
        test_path("/path/to/foo.css", "text/css");
        test_path("foo.html.css", "text/css");
        test_path("/path/to/foo.html.css", "text/css");
        test_path("/path/to.html/foo.css", "text/css");
    }

    fn test_path(path: &str, expected: &str) {
        assert_eq!(mime_for_path(Path::new(path)), Some(expected));
    }

    fn test_path_none(path: &str) {
        assert_eq!(mime_for_path(Path::new(path)), None);
    }
}
