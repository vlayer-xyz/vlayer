use criterion::{black_box, criterion_group, criterion_main, Criterion};
use key_value::{Database, Mdbx};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use tempfile::tempdir;

const KEY_SIZE: usize = 32;
const VALUE_SIZE: usize = 1024;
const TEST_TABLE: &str = "test";

fn generate_random_key(rng: &mut StdRng) -> [u8; KEY_SIZE] {
    let mut key = [0u8; KEY_SIZE];
    rng.fill_bytes(&mut key);
    key
}

fn generate_random_value(rng: &mut StdRng) -> Vec<u8> {
    let mut value = vec![0u8; VALUE_SIZE];
    rng.fill_bytes(&mut value);
    value
}

fn benchmark_insert(c: &mut Criterion) {
    c.bench_function("mdbx_insert_10000", |b| {
        b.iter(|| {
            let temp_dir = tempdir().unwrap();
            let db_path = temp_dir.path().to_path_buf();

            let mut db = Mdbx::open(&db_path).unwrap();
            let mut tx = db.begin_rw().unwrap();
            tx.create_table(TEST_TABLE).unwrap();
            tx.commit().unwrap();

            let seed: u64 = 42;
            let mut rng = StdRng::seed_from_u64(seed);

            for _ in 0..10_000 {
                let key = generate_random_key(&mut rng);
                let value = generate_random_value(&mut rng);
                let mut tx = db.begin_rw().unwrap();
                black_box(tx.insert(TEST_TABLE, &key, &value).unwrap());
                tx.commit().unwrap();
            }
        })
    });
}

criterion_group!(benches, benchmark_insert);
criterion_main!(benches);
