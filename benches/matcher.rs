#![feature(test)]
extern crate rff;
extern crate test;

use test::Bencher;

use rff::matcher::matches;

#[bench]
fn bench_matches(b: &mut Bencher) {
    b.iter(|| matches("amor", "app/models/order.rb"))
}

#[bench]
fn bench_matches_utf8(b: &mut Bencher) {
    b.iter(|| matches("ß", "WEIẞ"))
}

#[bench]
fn bench_matches_mixed(b: &mut Bencher) {
    b.iter(|| matches("abc", "abØ"))
}

#[bench]
fn bench_matches_more_specific(b: &mut Bencher) {
    b.iter(|| matches("app/models", "app/models/order.rb"))
}

#[bench]
fn bench_matches_mixed_case(b: &mut Bencher) {
    b.iter(|| matches("AMOr", "App/Models/Order.rb"))
}

#[bench]
fn bench_matches_multiple(b: &mut Bencher) {
    b.iter(|| {
        matches("amor", "app/models/order.rb");
        matches("amor", "spec/models/order_spec.rb");
        matches("amor", "other_garbage.rb");
        matches("amor", "Gemfile");
        matches("amor", "node_modules/test/a/thing.js");
        matches("amor", "vendor/bundle/ruby/gem.rb")
    })
}

#[bench]
fn bench_matches_eq(b: &mut Bencher) {
    b.iter(|| {
        matches("Gemfile", "Gemfile");
        matches("gemfile", "Gemfile")
    })
}
