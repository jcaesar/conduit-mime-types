use std::collections::{HashMap, HashSet};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Deserialize;

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
    solve(p1, "exts");
    solve(p2, "mimes");
}

fn solve(p1: HashSet<&String>, arg: &str) {
    let idxlen = 2;
    let basesize: usize = p1.iter().map(|v| v.len() + idxlen).sum();
    let longest = p1.iter().map(|v| v.len()).max().unwrap_or(0) + idxlen;
    let mut size = basesize;
    loop {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        'attempt: for _ in 0..1000 {
            let seed = rng.gen();
            let mut bitmap = vec![0; size];
            for ins in p1.iter() {
                let h = xxh3::hash64_with_seed(ins.as_bytes(), seed) as usize % (size - longest);
                let area = &mut bitmap[h..][..ins.len() + idxlen];
                if area.iter().all(|&b| b == 0) {
                    area.iter_mut().for_each(|b| *b = 1);
                } else {
                    continue 'attempt;
                }
            }
            println!(
                "Found solution for {arg} at size {size}, factor {}, seed {seed}",
                size as f64 / basesize as f64
            );
            return;
        }
        size = std::cmp::max((size as f64 * 1.1) as usize, size + 1);
    }
}
