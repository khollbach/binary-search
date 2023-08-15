use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use binary_search::iter_excl::binary_search as iter_excl;
use binary_search::iter_incl::binary_search as iter_incl;
use binary_search::rec_excl::binary_search as rec_excl;
use binary_search::tailrec_excl::binary_search as tailrec_excl;
use binary_search::tailrec_incl::binary_search as tailrec_incl;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_main!(benches);
criterion_group!(benches, benchmark);

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_search");

    for input in generate_inputs() {
        for (name, algo) in ALGOS {
            let n = input.sorted_nums.len();
            let id = BenchmarkId::new(name, n);
            group.bench_with_input(id, &input, |b, input| {
                b.iter(|| algo(&input.sorted_nums, input.target))
            });
        }
    }
}

const ALGOS: [(&str, fn(&[i32], i32) -> Option<usize>); 5] = [
    ("rec_excl", rec_excl),
    ("tailrec_excl", tailrec_excl),
    ("tailrec_incl", tailrec_incl),
    ("iter_excl", iter_excl),
    ("iter_incl", iter_incl),
];

struct Input {
    sorted_nums: Vec<i32>,
    target: i32,
}

/// Generate inputs of length [0, 2^0, ..., 2^i, ... 2^20].
fn generate_inputs() -> Vec<Input> {
    let mut inputs = vec![];
    inputs.push(Input {
        sorted_nums: vec![],
        target: 0,
    });

    // Largest input is 4 MiB = 2^20 * sizeof(i32).
    for k in 0..=20 {
        let n = 2usize.pow(k);

        // Generate n odd numbers: [1, 3, 5, ...,  2n - 1].
        let mut sorted_nums = Vec::with_capacity(n);
        for i in 0..n {
            let x = 2 * i + 1;
            sorted_nums.push(x as i32);
        }

        // Pick a target from [0, 2, ..., 2n].
        //
        // This makes sure the binary search takes logarithmic time, and doesn't
        // stop early.
        let hash = hash(k as u64) as usize;
        let index = hash % (n + 1);
        let target = (index * 2) as i32;

        inputs.push(Input {
            sorted_nums,
            target,
        })
    }

    inputs
}

/// Helper for `generate_inputs`.
fn hash(n: u64) -> u64 {
    // Note: the output of this may change between releases of the std lib.
    // But it'll be the same when re-running the same build of the program.
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(n);
    hasher.finish()
}
