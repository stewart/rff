#![feature(test)]
extern crate rff;
extern crate test;

use test::Bencher;

use rff::scorer::score;

#[bench]
fn bench_score(b: &mut Bencher) {
    b.iter(|| score("amor", "app/models/order.rb"))
}

#[bench]
fn bench_score_empty_needle(b: &mut Bencher) {
    b.iter(|| score("", "app/models/order.rb"))
}

#[bench]
fn bench_score_matching(b: &mut Bencher) {
    b.iter(|| score("app/models/order.rb", "app/models/order.rb"))
}

#[bench]
fn bench_score_huge_haystack(b: &mut Bencher) {
    let huge_string = "X".repeat(1025);
    b.iter(|| score("amor", &huge_string))
}

#[bench]
fn bench_score_multiple(b: &mut Bencher) {
    b.iter(|| {
        score("amor", "app/models/order.rb");
        score("amor", "spec/models/order_spec.rb");
        score("amor", "other_garbage.rb");
        score("amor", "Gemfile");
        score("amor", "node_modules/test/a/thing.js");
        score("amor", "vendor/bundle/ruby/gem.rb")
    })
}
