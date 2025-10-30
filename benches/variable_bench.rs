use criterion::{criterion_group, criterion_main, Criterion};

fn stack_variable(c: &mut Criterion) {
    c.bench_function("stack integer", |b| {
        b.iter(|| {
            let x = 42;
            x
        })
    });
}

fn heap_string(c: &mut Criterion) {
    c.bench_function("heap string", |b| {
        b.iter(|| {
            let s = String::from("Rust");
            s
        })
    });
}

fn clone_string(c: &mut Criterion) {
    c.bench_function("clone string", |b| {
        let s1 = String::from("Rust");
        b.iter(|| s1.clone())
    });
}

fn reference_borrow(c: &mut Criterion) {
    c.bench_function("reference borrow", |b| {
        let s1 = String::from("Rust");
        b.iter(|| {
            let s2 = &s1;
            s2
        })
    });
}

criterion_group!(benches, stack_variable, heap_string, clone_string, reference_borrow);
criterion_main!(benches);
