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

This benchmark was run on Ubuntu 16.04 LTS, on a fresh 20-core Linode instance:

    $ find ./linux -type f > files

    $ time fzy -e 'drivers' --benchmark < files
    real    0m0.421s
    user    0m2.880s
    sys     0m0.110s

    $ time rff -s 'drivers' --benchmark < files
    real    0m0.620s
    user    0m7.730s
    sys     0m0.537s

Interestingly, the macOS implementation of `tolower(3)` appears to be not nearly as well optimized, resulting in much slower scoring for `fzy`:

    λ find ~/dev/linux -type f > files

    λ time fzy -e drivers --benchmark < files
    fzy -e drivers --benchmark < files  12.06s user 0.07s system 733% cpu 1.653 total

    λ time rff -s drivers --benchmark < files
    rff -s drivers --benchmark < files  6.79s user 0.25s system 747% cpu 0.941 total

[fzy]: https://github.com/jhawthorn/fzy
[fzy-algo]: https://github.com/jhawthorn/fzy/blob/master/ALGORITHM.md
[selecta]: https://github.com/garybernhardt/selecta
[selecta-examples]: https://github.com/garybernhardt/selecta/blob/master/EXAMPLES.md
