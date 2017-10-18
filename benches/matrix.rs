#![feature(test)]
extern crate rff;
extern crate test;

use test::Bencher;

use rff::matrix::Matrix;

#[bench]
fn bench_matrix_new(b: &mut Bencher) {
    b.iter(|| Matrix::new(10, 256))
}

#[bench]
fn bench_matrix_new_large_haystack(b: &mut Bencher) {
    b.iter(|| Matrix::new(10, 10240))
}
