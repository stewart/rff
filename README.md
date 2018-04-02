# rff

`rff` is a fast, simple fuzzy text selector for the terminal, with full UTF-8 support. It uses [**`fzy`'s advanced scoring algorithm**][fzy-algo].

### Installation

[**Pre-compiled binaries**](https://github.com/stewart/rff/releases) are available, starting with version 0.3.0.

If you have a Rust toolchain installed, you can also build `rff` from source via Cargo:

    $ cargo install rff

Additionally, you can tell `rustc` to build for your specific CPU architecture, for a possible minor speedup:

    $ RUSTFLAGS="-C target-cpu=native" cargo install rff

### Usage

`rff` is a drop-in replacement for other fuzzy finders such as [`fzy`][fzy] or [`selecta`][selecta].

For some examples of how these sorts of tools can be used, see [`selecta`'s examples][selecta-examples].

### Benchmarks

Both `fzy` and `rff` have a `--benchmark` mode, which runs the matching/scoring loop 100 times, without printing anything.
This helps minimize impact of startup cost and I/O, and better demonstrate actual matching/scoring speed.

To ensure a large enough corpus, such that multithreaded optimizations can take place, these tests were run against a listing of files in the Linux kernel source tree.

The results indicate that in general, `rff` is _slightly_ slower that `fzy` on Linux, and significantly faster on macOS.
However, in real use, both of these tool will be more than fast enough for most projects.
We're testing against the entire Linux kernel, in an arbitrary benchmark mode, after all!

A tradeoff is that `rff` supports UTF-8 characters, both in input and search terms, while `fzy` focuses on ASCII, though there are plans to support wide characters eventually.

The [`hyperfine`](https://github.com/sharkdp/hyperfine) tool was used to generate these results.

This benchmark was run on on a fresh 20-core Linode instance running Ubuntu 16.04 LTS:

    $ find ~/dev/linux -type f > files

    $ hyperfine --warmup 5 'fzy -e drivers --benchmark < files' 'rff -s drivers --benchmark < files'
    Benchmark #1: fzy -e drivers --benchmark < files

      Time (mean ± σ):     423.9 ms ±   7.8 ms

      Range (min … max):   407.7 ms … 432.9 ms

    Benchmark #2: rff -s drivers --benchmark < files

      Time (mean ± σ):     529.7 ms ±   7.2 ms

      Range (min … max):   512.2 ms … 536.4 ms

Interestingly, the macOS implementation of `tolower(3)` appears to be poorly optimized, resulting in much slower scoring for `fzy`:

    λ find ~/dev/linux -type f > files

    $ hyperfine --warmup 5 'fzy -e drivers --benchmark < files' 'rff -s drivers --benchmark < files'
    Benchmark #1: fzy -e drivers --benchmark < files

      Time (mean ± σ):     1.764 s ± 0.337 s

      Range (min … max):   1.653 s … 2.722 s

    Benchmark #2: rff -s drivers --benchmark < files

      Time (mean ± σ):     1.019 s ± 0.004 s

      Range (min … max):   1.015 s … 1.027 s

If you've got any ideas for how `rff` can get even faster, please reach out and let me know!
This is an area of programming that I'm not entirely familiar with, and building this tool has been a fun learning experience!

[fzy]: https://github.com/jhawthorn/fzy
[fzy-algo]: https://github.com/jhawthorn/fzy/blob/master/ALGORITHM.md
[selecta]: https://github.com/garybernhardt/selecta
[selecta-examples]: https://github.com/garybernhardt/selecta/blob/master/EXAMPLES.md
