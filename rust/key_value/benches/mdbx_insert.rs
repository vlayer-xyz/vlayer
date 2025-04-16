#![allow(clippy::unwrap_used)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use key_value::{Database, Mdbx};
use rand::{RngCore, SeedableRng, rngs::StdRng};
use tempfile::tempdir;

const KEY_SIZE: usize = 32;
const VALUE_SIZE: usize = 1024;
const TEST_TABLE: &str = "test";

fn generate_random_entry(rng: &mut StdRng) -> ([u8; KEY_SIZE], Vec<u8>) {
    let mut key = [0; KEY_SIZE];
    let mut value = vec![0; VALUE_SIZE];
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut value);
    (key, value)
}

fn benchmark_insert(c: &mut Criterion) {
    c.bench_function("mdbx_insert_10000", |b| {
        let seed = 42;
        let mut rng = StdRng::seed_from_u64(seed);
        let entries: Vec<_> = (0..10_000)
            .map(|_| generate_random_entry(&mut rng))
            .collect();

        b.iter(|| {
            let temp_dir = tempdir().unwrap();
            let db_path = temp_dir.path().to_path_buf();

            let mut db = Mdbx::open(&db_path).unwrap();
            let mut tx = db.begin_rw().unwrap();
            tx.create_table(TEST_TABLE).unwrap();
            tx.commit().unwrap();

            for (key, value) in &entries {
                let mut tx = db.begin_rw().unwrap();
                #[allow(clippy::unit_arg)]
                black_box(tx.insert(TEST_TABLE, key, value).unwrap());
                tx.commit().unwrap();
            }
        })
    });
}

criterion_group!(benches, benchmark_insert);
criterion_main!(benches);
