#![feature(test)]
extern crate rff;
extern crate test;

use test::Bencher;

use rff::fuzzy::Score;

#[bench]
fn score(b: &mut Bencher) {
    b.iter(|| Score::new("amor", "app/models/order.rb"))
}

#[bench]
fn score_multiple(b: &mut Bencher) {
    b.iter(|| {
        Score::new("amor", "app/models/order.rb");
        Score::new("amor", "spec/models/order_spec.rb");
        Score::new("amor", "other_garbage.rb");
        Score::new("amor", "Gemfile");
        Score::new("amor", "node_modules/test/a/thing.js");
        Score::new("amor", "vendor/bundle/ruby/gem.rb")
    })
}

#[bench]
fn score_with_positions(b: &mut Bencher) {
    b.iter(|| Score::with_positions("amor", "app/models/order.rb"))
}

#[bench]
fn score_multiple_with_positions(b: &mut Bencher) {
    b.iter(|| {
        Score::with_positions("amor", "app/models/order.rb");
        Score::with_positions("amor", "spec/models/order_spec.rb");
        Score::with_positions("amor", "other_garbage.rb");
        Score::with_positions("amor", "Gemfile");
        Score::with_positions("amor", "node_modules/test/a/thing.js");
        Score::with_positions("amor", "vendor/bundle/ruby/gem.rb")
    })
}
