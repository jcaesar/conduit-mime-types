#![feature(test)]
extern crate test;

#[cfg(test)]
mod bench {
    static TEST_EXT: [&'static str; 5] = ["css", "html", "png", "wtfnope", "avif"];
    static TEST_MIME: [&'static str; 5] = [
        "application/thisonedoesn'texist",
        "application/vnd.wap.sic",
        "audio/evrcb",
        "application/x-perl",
        "video/h265",
    ];
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_phf_e2m(b: &mut Bencher) {
        b.iter(|| {
            for kw in test::black_box(TEST_EXT) {
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
    fn bench_phf_m2e(b: &mut Bencher) {
        b.iter(|| {
            for kw in test::black_box(TEST_MIME) {
                test::black_box(conduit_mime_types::get_extension(kw));
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
}
