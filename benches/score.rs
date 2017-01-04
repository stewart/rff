#![feature(test)]
extern crate rff;
extern crate test;

use test::Bencher;

use rff::fuzzy::score::Score;

#[bench]
fn bench_score(b: &mut Bencher) {
    b.iter(|| Score::calculate("amor", "app/models/order.rb"))
}

#[bench]
fn bench_score_multiple(b: &mut Bencher) {
    b.iter(|| {
        Score::calculate("amor", "app/models/order.rb");
        Score::calculate("amor", "spec/models/order_spec.rb");
        Score::calculate("amor", "other_garbage.rb");
        Score::calculate("amor", "Gemfile");
        Score::calculate("amor", "node_modules/test/a/thing.js");
        Score::calculate("amor", "vendor/bundle/ruby/gem.rb")
    })
}

