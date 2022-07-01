#![feature(test)]
extern crate test;

#[cfg(test)]
mod bench {
    use super::*;
    use std::{
        cmp::min,
        collections::{hash_map::DefaultHasher, HashMap},
        hash::{Hash, Hasher},
    };
    use test::Bencher;
    static TEST_EXT: [&'static str; 5] = ["css", "html", "png", "wtfnope", "avif"];
    static TEST_MIME: [&'static str; 5] = [
        "application/thisonedoesn'texist",
        "application/vnd.wap.sic",
        "audio/evrcb",
        "application/x-perl",
        "video/h265",
    ];

    #[bench]
    fn bench_phf_m2e(b: &mut Bencher) {
        b.iter(|| {
            for kw in test::black_box(TEST_EXT) {
                test::black_box(conduit_mime_types::get_extension(kw));
            }
        });
    }

    #[bench]
    fn bench_phf_e2m(b: &mut Bencher) {
        b.iter(|| {
            for kw in test::black_box(TEST_MIME) {
                test::black_box(conduit_mime_types::get_mime_type(kw));
            }
        });
    }

    #[bench]
    fn bench_match_e2m(b: &mut Bencher) {
        b.iter(|| {
            for kw in test::black_box(TEST_EXT) {
                test::black_box(cmtm::get_mime_type(kw));
            }
        });
    }

    #[bench]
    fn bench_match_m2e(b: &mut Bencher) {
        b.iter(|| {
            for kw in test::black_box(TEST_MIME) {
                test::black_box(cmtm::get_extension(kw));
            }
        });
    }

    #[bench]
    fn bench_hash(b: &mut Bencher) {
        b.iter(|| {
            for kw in test::black_box(TEST_MIME) {
                let mut h = DefaultHasher::default();
                kw.hash(&mut h);
                test::black_box(h.finish());
            }
        });
    }

    #[bench]
    fn bench_hash_short(b: &mut Bencher) {
        b.iter(|| {
            for kw in test::black_box(TEST_MIME) {
                let mut b = [0u8; 8];
                let kw = &kw.as_bytes()[kw.len() / 2..];
                let kw = &kw[..min(8, kw.len())];
                b[..kw.len()].copy_from_slice(kw);
                test::black_box(u64::from_ne_bytes(b) % 3000);
            }
        });
    }

    #[bench]
    fn bench_xxh3(b: &mut Bencher) {
        b.iter(|| {
            for kw in test::black_box(TEST_MIME) {
                test::black_box(xxh3::hash64_with_seed(kw.as_bytes(), 42));
            }
        });
    }

    #[bench]
    fn bench_xxh3_mod(b: &mut Bencher) {
        b.iter(|| {
            for kw in test::black_box(TEST_MIME) {
                test::black_box(xxh3::hash64_with_seed(kw.as_bytes(), 42) % 1337);
            }
        });
    }
}
