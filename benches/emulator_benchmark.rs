use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gameboy_dmg_emulator::GameBoy;

fn emulator_benchmark(c: &mut Criterion) {
    let mut gameboy = GameBoy::new();
    
    c.bench_function("emulator_step", |b| {
        b.iter(|| {
            gameboy.step();
        })
    });
    
    c.bench_function("emulator_frame", |b| {
        b.iter(|| {
            for _ in 0..70224 {
                gameboy.step();
            }
        })
    });
}

criterion_group!(benches, emulator_benchmark);
criterion_main!(benches);