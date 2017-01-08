#![feature(test)]
extern crate rff;
extern crate test;

use test::Bencher;

use rff::fuzzy::bonus::compute_bonus;

#[bench]
fn bench_compute_bonus(b: &mut Bencher) {
    b.iter(|| compute_bonus("app/models/order"))
}
