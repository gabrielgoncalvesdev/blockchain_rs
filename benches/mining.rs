use blockchain_rs::{Blockchain, Transaction};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_mining(c: &mut Criterion) {
    let mut group = c.benchmark_group("mine");
    for threads in [1usize, 2, 4, 8] {
        group.bench_function(format!("difficulty5_threads{threads}"), |b| {
            b.iter (|| {
                let mut bc = Blockchain::new(5, 2_000, 1_000);
                bc.add_block(vec![Transaction::new("a", "b", 1)], 1_500, threads);
                bc
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_mining);
criterion_main!(benches);