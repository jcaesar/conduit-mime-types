use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::Deserialize;
use std::any::Any;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

static JSON: &str = include_str!("data/mime.json");

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(default)]
    extensions: Vec<String>,
}

fn main() {
    let json: BTreeMap<String, Record> = serde_json::from_str(JSON).unwrap();

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut used_exts = HashSet::new();
    let mut ext_by_type = HashMap::new();
    let mut type_by_ext = HashMap::new();

    for (mime_type, record) in json.iter() {
        let exts = &record.extensions;

        for ext in exts {
            if used_exts.insert(ext) {
                type_by_ext.insert(ext.as_str(), format!(r#""{}""#, mime_type));
            }
        }

        ext_by_type.insert(
            mime_type.as_str(),
            format!(
                "&[{}]",
                exts.iter()
                    .map(|ext| format!("\"{}\"", ext))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );
    }
    let type_sol = solve(&ext_by_type, 480037, 6595994924711974694);
    let ext_sol = solve(&type_by_ext, 33888, 13595991666906612394);

    let type_ret = type_sol
        .2
        .into_iter()
        .map(|k| ext_by_type.get(k).unwrap().as_str())
        .collect::<Vec<_>>();
    writeln!(
        &mut file,
        r#"fn gen_get_extension(name: &str) -> Option<&[&str]> {{
            let seed = {};
            let lookup = {:?};
            static RET: [&'static [&'static str]; {}] = [{}];
            do_static_lookup(seed, {}, &lookup, &RET, name)
        }}"#,
        type_sol.0,
        type_sol.1,
        type_ret.len(),
        type_ret.join(","),
        26,
    )
    .unwrap();

    let ext_ret = ext_sol
        .2
        .into_iter()
        .map(|k| type_by_ext.get(k).unwrap().as_str())
        .collect::<Vec<_>>()
        .join(",");
    writeln!(
        &mut file,
        r#"fn gen_get_mime_type(ext: &str) -> Option<&str> {{
            let seed = {};
            let lookup = {:?};
            let ret = [{}];
            do_static_lookup(seed, {}, &lookup, &ret, ext)
        }}"#,
        ext_sol.0, ext_sol.1, ext_ret, 86
    )
    .unwrap();
}

fn solve<'a>(
    p1: &HashMap<&'a str, impl Any>,
    size: usize,
    seed: u64,
) -> (u64, Vec<u8>, Vec<&'a str>) {
    assert!(p1.len() < 1 << 16);
    let idxlen = 2;
    let align = 3;
    let basesize: usize = p1.keys().map(|v| v.len() + idxlen).sum();
    let longest = p1.keys().map(|v| v.len()).max().unwrap_or(0) + idxlen;
    let mut p1 = p1.keys().cloned().collect::<Vec<_>>();
    let (nh, sw) = if size < 2 << (16 + align) {
        (4, 16)
    } else if size < 2 << (21 + align) {
        (3, 21)
    } else if size < 2 << (26 + align) {
        (2, 32)
    } else {
        panic!("Giving up at factor {}", size as f64 / basesize as f64);
    };
    p1.sort();
    p1.shuffle(&mut ChaCha8Rng::seed_from_u64(seed));
    let mut bitmap = vec![0u8; size];
    let mut sol = vec![b' '; size];
    for (i, ins) in p1.iter().enumerate() {
        let h = xxh3::hash64_with_seed(ins.as_bytes(), seed);
        if (0..nh).all(|s| {
            let h1 = (h >> (s * sw)) & (u64::MAX >> (64 - ((s + 1) * sw)));
            let h1 = h1 as usize % ((size - longest) >> align);
            let h1 = h1 << align;
            let area = &mut bitmap[h1..][..ins.len() + idxlen];
            if area.iter().all(|&b| b == 0) {
                area.iter_mut().for_each(|b| *b = 1);
                let solarea = &mut sol[h1..][..ins.len() + idxlen];
                solarea[..ins.len()].copy_from_slice(ins.as_bytes());
                solarea[ins.len()..][..2].copy_from_slice(&u16::to_le_bytes(i as u16));
                false
            } else {
                //println!("{ins} collides at {h1} {h} {s}");
                true
            }
        }) {
            panic!("Unavoidable colision");
        }
    }
    return (seed, sol, p1);
}
