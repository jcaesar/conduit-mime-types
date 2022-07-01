use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

fn main() {
    #[derive(Deserialize)]
    struct E {
        #[serde(default)]
        extensions: Vec<String>,
    }
    let mime: HashMap<String, E> = serde_json::from_slice(
        &std::fs::read("/home/julius/code/conduit-mime-types/data/mime.json").unwrap(),
    )
    .unwrap();

    let p1 = mime
        .values()
        .flat_map(|v| v.extensions.iter())
        .collect::<HashSet<_>>();
    let p2 = mime.keys().collect::<HashSet<_>>();
    let xxh3 = |ins: &str, seed: u64| xxh3::hash64_with_seed(ins.as_bytes(), seed);
    let _dh = |ins: &str, seed: u64| {
        let mut h = DefaultHasher::default();
        seed.hash(&mut h);
        ins.hash(&mut h);
        h.finish()
    };
    solve(&p1, "exts/xxh3", xxh3);
    //solve(&p1, "exts/siph", dh);
    solve(&p2, "mimes/xxh3", xxh3);
    //solve(&p2, "mimes/siph", dh);
}

fn solve(p1: &HashSet<&String>, arg: &str, hasher: impl Fn(&str, u64) -> u64) {
    let idxlen = 2;
    let align = 3;
    let basesize: usize = p1.iter().map(|v| v.len() + idxlen).sum();
    let longest = p1.iter().map(|v| v.len()).max().unwrap_or(0) + idxlen;
    let mut size = basesize;
    let mut p1 = p1.iter().collect::<Vec<_>>();
    let mut foundqual = 10.0;
    loop {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let (nh, sw) = if size < 2 << (16 + align) {
            (4, 16)
        } else if size < 2 << (21 + align) {
            (3, 21)
        } else if size < 2 << (26 + align) {
            (2, 32)
        } else {
            println!(
                "Giving up on {arg} at factor {}",
                size as f64 / basesize as f64
            );
            return;
        };
        'attempt: for _ in 0..3000 {
            let seed = rng.gen();
            p1.sort();
            p1.shuffle(&mut ChaCha8Rng::seed_from_u64(seed));
            let mut bitmap = vec![0u8; size];
            let mut stats = vec![0usize; nh];
            let mut sol = vec![b' '; size];
            for ins in p1.iter() {
                let h = hasher(ins, seed);
                if (0..nh).all(|s| {
                    let h1 = (h >> (s * sw)) & (u64::MAX >> (64 - ((s + 1) * sw)));
                    let h1 = h1 as usize % ((size - longest) >> align);
                    let h1 = h1 << align;
                    let area = &mut bitmap[h1..][..ins.len() + idxlen];
                    if area.iter().all(|&b| b == 0) {
                        area.iter_mut().for_each(|b| *b = 1);
                        let solarea = &mut sol[h1..][..ins.len() + idxlen];
                        solarea[..ins.len()].copy_from_slice(ins.as_bytes());
                        solarea[ins.len()..][..2].copy_from_slice(b"XX");
                        stats[s] += 1;
                        false
                    } else {
                        //println!("{ins} collides at {h1} {h} {s}");
                        true
                    }
                }) {
                    continue 'attempt;
                }
            }
            let thisqual = (stats
                .iter()
                .enumerate()
                .map(|(i, c)| c * (i + 1))
                .sum::<usize>() as f64)
                / p1.len() as f64;
            if thisqual < foundqual / 1.00 || thisqual < 1.08 {
                println!(
                    "Found solution for {arg} at size {size} with seed {seed}, factor {}, collisions: {stats:?}, quality {thisqual}, longest {longest}",
                    size as f64 / basesize as f64,
                );
            }
            foundqual = if thisqual < foundqual {
                thisqual
            } else {
                foundqual
            };
            if thisqual < 1.08 {
                //hexdump::hexdump(&sol);
                return;
            }
        }
        size = std::cmp::max((size as f64 * 1.05) as usize, size + 1);
    }
}
