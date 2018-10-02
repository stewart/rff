#[macro_use]
extern crate criterion;
extern crate rff;

use criterion::Criterion;

fn bench_matches(c: &mut Criterion) {
    use rff::matches;

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

fn bench_bonus(c: &mut Criterion) {
    use rff::bonus;

    c.bench_function("bonus-baseline", |b| {
        let haystack = "app/models/order.rb";
        b.iter(|| bonus::compute(haystack));
    });

    c.bench_function("bonus-react", |b| {
        let haystack = "app/webpack/components/utility/MediaQuery.tsx";
        b.iter(|| bonus::compute(haystack));
    });

    c.bench_function("bonus-linux-worst-case", |b| {
        let haystack = "arch/cris/include/arch-v32/mach-a3/mach/hwregs/iop/asm/iop_sap_out_defs_asm.h";
        b.iter(|| bonus::compute(haystack));
    });

    c.bench_function("bonus-node_modules-worst-case", |b| {
        let haystack = "node_modules/stylelint-scss/node_modules/stylelint/lib/rules/declaration-block-no-redundant-longhand-properties/index.js";
        b.iter(|| bonus::compute(haystack));
    });
}

criterion_group!(benches, bench_matches, bench_bonus);
criterion_main!(benches);
