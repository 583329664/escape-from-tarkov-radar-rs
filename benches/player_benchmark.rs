use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use domain::{application::{memory_operations::MemoryOperations, operations::Operations}, models::player::Player};
use external_memory_lib::MemoryConfigurer;

fn criterion_benchmark(c: &mut Criterion) {
    let memory = Arc::new(
    MemoryConfigurer::default()
            .configure("EscapeFromTarkov.exe", "UnityPlayer.dll", 0x17FFD28)
            .build()
            .unwrap(),
    );

    let shared_state = Arc::new(MemoryOperations::new(memory).unwrap());

    c.bench_function("players without cache", |b| b.iter(|| {
        black_box(shared_state.update_players(&[]));
    }));

    let players = shared_state.update_players(&[]).unwrap();

    c.bench_function("players with cache", |b| b.iter(|| {
        black_box(shared_state.update_players(&players));
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);