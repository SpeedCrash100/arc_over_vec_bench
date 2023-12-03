use std::{rc::Rc, sync::Arc};

use arc_over_vec_bench::{generate_vec, DataType};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

struct BenchmarkDescription {
    datas: Vec<Vec<DataType>>,
}

fn bench_arc(c: &mut Criterion, description: &BenchmarkDescription) {
    let mut datas: Vec<Arc<[DataType]>> = vec![];

    for d in &description.datas {
        datas.push(d.clone().into());
    }

    let mut group = c.benchmark_group("Arc");

    for d in datas {
        let size = d.len();
        let id = BenchmarkId::from_parameter(size);

        group.throughput(criterion::Throughput::Elements(size as u64));
        group.bench_with_input(id, &d, |b, i| {
            b.iter(|| {
                let data = i.clone();
                black_box(data);
            })
        });
    }
}
fn bench_rc(c: &mut Criterion, description: &BenchmarkDescription) {
    let mut datas: Vec<Rc<[DataType]>> = vec![];

    for d in &description.datas {
        datas.push(d.clone().into());
    }

    let mut group = c.benchmark_group("Rc");

    for d in datas {
        let size = d.len();
        let id = BenchmarkId::from_parameter(size);

        group.throughput(criterion::Throughput::Elements(size as u64));
        group.bench_with_input(id, &d, |b, i| {
            b.iter(|| {
                let data = i.clone();
                black_box(data);
            })
        });
    }
}
fn bench_vec(c: &mut Criterion, description: &BenchmarkDescription) {
    let mut datas: Vec<Vec<DataType>> = vec![];

    for d in &description.datas {
        datas.push(d.clone());
    }

    let mut group = c.benchmark_group("Vec");

    for d in datas {
        let size = d.len();
        let id = BenchmarkId::from_parameter(size);

        group.throughput(criterion::Throughput::Elements(size as u64));
        group.bench_with_input(id, &d, |b, i| {
            b.iter(|| {
                let data = i.clone();
                black_box(data);
            })
        });
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    static ELEMENT_SIZES: &[usize] = &[0, 1, 10, 100, 1000, 10000, 100000, 1000000];
    let mut datas = vec![];

    for count in ELEMENT_SIZES {
        datas.push(generate_vec(*count));
    }

    let desc = BenchmarkDescription {
        datas: datas.clone(),
    };

    bench_arc(c, &desc);
    bench_rc(c, &desc);
    bench_vec(c, &desc);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
