# rff

`rff` is a fast, simple fuzzy text selector for the terminal, with UTF-8 support. It uses a similar scoring algorithm to [`fzy`][fzy-algo].

It currently has a very similar feature set to `fzy`, although this may change in the future.

### Installation

At the moment, installation is best done through Cargo:

    $ cargo install --git https://github.com/stewart/rff

If you have a Rust toolchain set up locally, you can also build from source with `cargo build --release`.

### Usage

`rff` is a drop-in replacement for other fuzzy finders such as [`fzy`][fzy] or [`selecta`][selecta].

For some examples of how these sorts of tools can be used, see [`selecta`'s examples][selecta-examples].

### Benchmarks

Both `fzy` and `rff` have a `--benchmark` mode, with runs the matching/scoring loop 100 times without printing anything.
This helps to minimize impact of startup cost and I/O, and better demonstrate actual matching/scoring speed.

This benchmark was run on Arch Linux:

    $ find ~/dev/linux -type f > files

    $ time fzy -e drivers --benchmark < files
    fzy -e drivers --benchmark < files  1.97s user 0.04s system 344% cpu 0.583 total

    $ time rff -s drivers --benchmark < files
    rff -s drivers --benchmark < files  4.99s user 0.07s system 356% cpu 1.421 total

Interestingly, the macOS implementation of `tolower(3)` appears to be not nearly as well optimized, resulting in much slower scoring for `fzy`:

    $ find ~/dev/linux -type f > files

    $ time fzy -e drivers --benchmark < files
    fzy -e drivers --benchmark < files  17.92s user 0.07s system 364% cpu 4.932 total

    $ time rff -s drivers --benchmark < files
    rff -s drivers --benchmark < files  8.93s user 0.08s system 349% cpu 2.578 total

[fzy]: https://github.com/jhawthorn/fzy
[fzy-algo]: https://github.com/jhawthorn/fzy/blob/master/ALGORITHM.md
[selecta]: https://github.com/garybernhardt/selecta
[selecta-examples]: https://github.com/garybernhardt/selecta/blob/master/EXAMPLES.md
