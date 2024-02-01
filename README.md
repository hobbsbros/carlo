# The Carlo Language

**Carlo** is a simple interpreted programming language designed to fill the role of an "engineer's calculator".

## What's in a Name?

Carlo is named after Blessed Carlo Acutis, an Italian Catholic website designer and the patron of computer programmers.  He succumbed to leukemia on October 12, 2006, and was beatified by the Catholic Church on October 10, 2020.

You may read more about Carlo Acutis [on Wikipedia](https://en.wikipedia.org/wiki/Carlo_Acutis).

## Package Structure

The complete Carlo language is contained in the `carlo` workspace.  The `carlo` workspace is composed of a series of Rust crates (packages).

- `carlotk`, the Carlo language toolkit, containing the Carlo language parser, CLI argument parser, and other utilities,

- `carlo-help`, the Carlo language help utility,

- `carlo-repl`, the Carlo language REPL,

- `carlo-run`, the Carlo langauge execution subcommand,

- `carlo-latex`, the Carlo LaTeX emitter, and

- `carlo-version`, the Carlo language version subcommand.

These packages are joined together in the `src/main.rs` executable, which invokes `carlotk::include_subcommands` macro to build the full `carlo` binary.

## License

The Carlo Language and all of its constitutive crates are licensed under the MIT license.