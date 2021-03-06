#![feature(test)]

extern crate test;

use rust_bench_test::fibonacci;
use test::Bencher;

#[bench]
fn bench_fib_10(b: &mut Bencher) {
    b.iter(|| {
        let _ = fibonacci(10);
    });
}

#[bench]
fn bench_fib_20(b: &mut Bencher) {
    b.iter(|| {
        let _ = fibonacci(20);
    });
}
