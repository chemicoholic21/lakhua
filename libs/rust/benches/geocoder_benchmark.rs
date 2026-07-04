use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lakhua::{geocode, geocode_h3, GeocodeOptions};

fn bench_geocode_h3(c: &mut Criterion) {
    let options = GeocodeOptions::default();

    c.bench_function("geocode_h3_res5_match", |b| {
        b.iter(|| geocode_h3(black_box("853d838bfffffff"), &options))
    });

    c.bench_function("geocode_h3_res4_match", |b| {
        b.iter(|| geocode_h3(black_box("843d839ffffffff"), &options))
    });

    let no_fallback = GeocodeOptions {
        resolution: 5,
        fallback: false,
        debug: false,
    };
    c.bench_function("geocode_h3_no_fallback", |b| {
        b.iter(|| geocode_h3(black_box("853d838bfffffff"), &no_fallback))
    });
}

fn bench_geocode_latlon(c: &mut Criterion) {
    let options = GeocodeOptions::default();

    c.bench_function("geocode_delhi", |b| {
        b.iter(|| geocode(black_box(28.6139), black_box(77.2090), &options))
    });

    c.bench_function("geocode_mumbai", |b| {
        b.iter(|| geocode(black_box(19.0760), black_box(72.8777), &options))
    });
}

criterion_group!(benches, bench_geocode_h3, bench_geocode_latlon);
criterion_main!(benches);
