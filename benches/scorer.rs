#![feature(test)]
extern crate rff;
extern crate test;

use test::Bencher;

use rff::scorer::{score, score_with_positions, compute_bonus};

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

#[bench]
fn bench_score_with_positions(b: &mut Bencher) {
    b.iter(|| score_with_positions("amor", "app/models/order.rb"))
}

#[bench]
fn bench_score_multiple_with_positions(b: &mut Bencher) {
    b.iter(|| {
        score_with_positions("amor", "app/models/order.rb");
        score_with_positions("amor", "spec/models/order_spec.rb");
        score_with_positions("amor", "other_garbage.rb");
        score_with_positions("amor", "Gemfile");
        score_with_positions("amor", "node_modules/test/a/thing.js");
        score_with_positions("amor", "vendor/bundle/ruby/gem.rb")
    })
}

#[bench]
fn bench_compute_bonus(b: &mut Bencher) {
    b.iter(|| compute_bonus("app/models/this/is/a/strangely/nested/path.rb"))
}

#[bench]
fn bench_compute_bonuses(b: &mut Bencher) {
    b.iter(|| {
        compute_bonus("app/models/order.rb");
        compute_bonus("spec/models/order_spec.rb");
        compute_bonus("other_garbage.rb");
        compute_bonus("Gemfile");
        compute_bonus("node_modules/test/a/thing.js");
        compute_bonus("vendor/bundle/ruby/gem.rb")
    })
}
