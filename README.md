# rff

`rff` is a fast, simple fuzzy selector for the terminal with an advanced scoring algorithm and full UTF-8 support.

### Installation

[**Pre-compiled binaries**](https://github.com/stewart/rff/releases) are available for common architectures, starting with version 0.3.0.

If you have a Rust toolchain installed, you can also build `rff` from source via Cargo:

    $ cargo install rff

### Usage

`rff` is a drop-in replacement for other fuzzy selection tools such as [`fzy`][fzy] or [`selecta`][selecta].

[selecta]: https://github.com/garybernhardt/selecta
[fzy]: https://github.com/jhawthorn/fzy

Its interface is straightforward:

- pass it a set of choices on `STDIN`
- it will present a fuzzy selection interface to the user, and block until they make a selection or quit with `^C`
- it will print the user's selection on `STDOUT`

As an example, you can say:

    $ vim $(find . -type f | rff)

Which prompts the user to select a file in or below the current directory, and then opens the selected file in `vim`.

`rff` supports these keys:

- `^N` to select the next match
- `^P` to select the previous match
- `^U` to clear the search query
- `^C`, `^D`, and `Esc` to exit without selecting a match

### Scoring

`rff` is currently based on [`fzy`][fzy]'s scoring algoritm. For details on how this is better than most fuzzy finders, see `fzy`'s [`ALGORITHM.md`][fzy-algorithm].

[fzy-algorithm]: https://github.com/jhawthorn/fzy/blob/master/ALGORITHM.md
