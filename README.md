# rff

`rff` is a fast, simple fuzzy text selector for the terminal. It uses a similar
scoring algorithm to [`fzy`][fzy].

### Installation

    $ cargo install --git https://github.com/stewart/rff

### Usage

Probably don't - this is still pretty new, and may have edge cases I've not run into yet.

If you do wish to use it, `rff` is a drop-in replacement for other fuzzy finders such as [`fzy`][fzy] or [`selecta`][selecta].

### UTF-8 Support

Generally, one of the most performance critical component of a fuzzy finder algorithms is matching - determining which possible choices are eligible for scoring.

`fzy` approaches this problem by using [`strpbrk(3)`][strpbrk] to [case-insensitively compare strings][fzy-match]. The glibc implementation of this is extremely fast, at the expense of UTF-8 support.

`rff` uses Rust's built-in `std::ascii::AsciiExt` module to perform [ASCII character comparison][rff-match] for speed, falling back to a UTF-8 friendly approach if all other options have been exhausted.

For more information on the full scoring algorithm used, please see `fzy`'s [`ALGORITHM.md`][fzy-algo].

### Benchmarks

Both `fzy` and `rff` have a `--benchmark` mode, with runs the matching/scoring loop 100 times without printing anything.
This helps to minimize impact of startup cost and I/O, and better demonstrate actual matching/scoring speed.

This benchmark was run on Arch Linux:

    $ find ~/dev/linux -type f > files

    $ cat files | time fzy -e drivers --benchmark
    fzy -e drivers --benchmark  1.52s user 0.01s system 202% cpu 0.755 total

    $ cat files | time rff -s drivers --benchmark
    rff -s drivers --benchmark  5.75s user 0.09s system 347% cpu 1.685 total

Interestingly, the macOS implementation of `tolower(3)` appears to be not nearly as well optimized, resulting in much slower scoring for `fzy`:

    $ find ~/dev/linux -type f > files

    $ cat files | time fzy -e drivers --benchmark
    fzy -e "drivers" --benchmark  18.50s user 0.06s system 364% cpu 5.090 total

    $ cat files | time rff -s drivers --benchmark
    rff -s "drivers" --benchmark  10.56s user 0.09s system 338% cpu 3.146 total

### TODO

- [ ] speed up scoring algorithm

[fzy]: https://github.com/jhawthorn/fzy
[fzy-algo]: https://github.com/jhawthorn/fzy/blob/master/ALGORITHM.md
[fzy-match]: https://github.com/jhawthorn/fzy/blob/9d16ab4997ce6eb211ff3fdf06275d3f6bf5ebdc/src/match.c#L13-L28
[rff-match]: https://github.com/stewart/rff/blob/8a35ca735e2e7a09277e3718fcc34472943c40d8/src/fuzzy/mod.rs#L37-L43
[selecta]: https://github.com/garybernhardt/selecta
[strpbrk]: https://linux.die.net/man/3/strpbrk
