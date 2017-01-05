#![feature(test)]
extern crate rff;
extern crate test;

use test::Bencher;

use rff::choice::Choice;

#[bench]
fn create_choice(b: &mut Bencher) {
    b.iter(|| Choice::new("app/models", "app/models/order"))
}
