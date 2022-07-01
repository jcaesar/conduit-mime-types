use std::{convert::TryInto, path::Path};

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

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

fn do_static_lookup<T: Copy>(
    seed: u64,
    longest: usize,
    table: &[u8],
    ret: &[T],
    lookup: &str,
) -> Option<T> {
    let align = 3;
    let idxlen = 2;
    let size = table.len();
    let (nh, sw) = if size < 2 << (16 + align) {
        (4, 16)
    } else if size < 2 << (21 + align) {
        (3, 21)
    } else if size < 2 << (26 + align) {
        (2, 32)
    } else {
        panic!();
    };
    let h = xxh3::hash64_with_seed(lookup.as_bytes(), seed);
    for s in 0..nh {
        let h1 = (h >> (s * sw)) & (u64::MAX >> (64 - ((s + 1) * sw)));
        let h1 = h1 as usize % ((size - longest) >> align);
        let h1 = h1 << align;
        if h1 + lookup.len() + idxlen >= table.len() {
            continue;
        }
        if &table[h1..][..lookup.len()] == lookup.as_bytes() {
            let idx = u16::from_le_bytes(table[h1 + lookup.len()..][..2].try_into().unwrap());
            return Some(ret[idx as usize]);
        }
    }
    None
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
