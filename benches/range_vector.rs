use std::collections::HashMap;

use chrono::{prelude::*, Duration};
use core::{Matrix, RangeVector, Sample, TimeRange};
use criterion::{criterion_group, criterion_main, Criterion};

fn generate_range_vectors(series_count: usize, row_count: usize) -> (TimeRange, Vec<RangeVector>) {
    let time_range = TimeRange {
        start: Utc.timestamp(0, 0),
        end: Utc.timestamp(row_count as i64, 0),
        step: Duration::seconds(1),
    };

    let rv: Vec<RangeVector> = (0..series_count)
        .map(|i| {
            let samples = (time_range.start().timestamp_millis()
                ..=(time_range.end().timestamp_millis()))
                .step_by(time_range.step.num_milliseconds() as usize)
                .filter_map(|j| {
                    if j as usize % (i + 1) != 0 {
                        return None;
                    }
                    Some(Sample {
                        timestamp: j as f64,
                        value: ((j as usize / (i + 1)) as f64).to_string(),
                    })
                })
                .collect();

            let mut metric = HashMap::new();
            metric.insert("__name__".to_owned(), format!("every_{}", i));
            RangeVector { metric, samples }
        })
        .collect();

    (time_range, rv)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("matrix 100 series 1,000 rows", |b| {
        let (tr, rv) = generate_range_vectors(100, 1_000);
        b.iter(|| {
            let frames = Matrix::from(&rv).to_frames(&tr);
            assert_eq!(100, frames.len());
        });
    });

    c.bench_function("matrix 1,000 series 100 rows", |b| {
        let (tr, rv) = generate_range_vectors(1_000, 100);
        b.iter(|| {
            let frames = Matrix::from(&rv).to_frames(&tr);
            assert_eq!(1_000, frames.len());
        });
    });

    c.bench_function("matrix 100 series 100 rows", |b| {
        let (tr, rv) = generate_range_vectors(100, 100);
        b.iter(|| {
            let frames = Matrix::from(&rv).to_frames(&tr);
            assert_eq!(100, frames.len());
        });
    });

    c.bench_function("matrix 1 series 10,000 rows", |b| {
        let (tr, rv) = generate_range_vectors(1, 10_000);
        b.iter(|| {
            let frames = Matrix::from(&rv).to_frames(&tr);
            assert_eq!(1, frames.len());
        });
    });

    c.bench_function("matrix 10,000 series 1 rows", |b| {
        let (tr, rv) = generate_range_vectors(10_000, 1);
        b.iter(|| {
            let frames = Matrix::from(&rv).to_frames(&tr);
            assert_eq!(10_000, frames.len());
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
