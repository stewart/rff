#[macro_use]
extern crate criterion;
extern crate rff;

use criterion::*;
use rff::matches;

fn bench_matches(c: &mut Criterion) {
    c.bench_function("matches-baseline", |b| {
        b.iter(|| matches("amo", "app/models/order.rb"))
    });

    c.bench_function("matches-german", |b| {
        b.iter(|| matches("ß", "WEIẞ"));
    });

    c.bench_function("matches-mixed", |b| {
        b.iter(|| matches("abc", "abØ"));
    });

    c.bench_function("matches-more-specific", |b| {
        b.iter(|| matches("app/models", "app/models/order.rb"));
    });

    c.bench_function("matches-mixed-case", |b| {
        b.iter(|| matches("AMOr", "App/Models/Order.rb"));
    });

    c.bench_function("matches-multiple", |b| {
        b.iter(|| {
            matches("amor", "app/models/order.rb");
            matches("amor", "spec/models/order_spec.rb");
            matches("amor", "other_garbage.rb");
            matches("amor", "Gemfile");
            matches("amor", "node_modules/test/a/thing.js");
            matches("amor", "vendor/bundle/ruby/gem.rb");
        });
    });

    c.bench_function("matches-equal", |b| {
        b.iter(|| {
            matches("Gemfile", "Gemfile");
            matches("gemfile", "Gemfile");
        });
    });
}

criterion_group!(benches, bench_matches);
criterion_main!(benches);
