# Developer's Guide

## Incrementing the Version

The following instructions will guide you to incrementing the Carlo language
version number.

1. Open `carlotk/src/lib.rs`.  Change the `VERSION` string to the new version number.

2. Open `carlotk/Cargo.toml`.  Change the version number under the `[package]` key.

3. Open `Cargo.toml` and change *both* the package version (under `[package]`) and the `carlotk` dependency version (under `[dependencies.carlotk]`) to the new version number.

As a general rule, to minimize confusion and maximally facilitate dependency resolution, *the Carlo binary and the CarloTK library must be at the same version*.

## Adding a Subcommand

The following instructions will guide you to adding a Carlo langauge subcommand.

1. Create a Cargo library package `carlo-sc`, where `sc` is your desired subcommand name.

2. Add the following to your subcommand library's `Cargo.toml` file.

```
[dependencies.carlotk]
version = "0.12.0"
```

3. Create a function in your subcommand library named `sc`.  It is recommended that you import `carlotk::prelude::*` so you can use the Carlo tokenizer, parser, and environment utilities.  Your function must have the following signature: `fn(CliArgs) -> ()`.

4. Create a function in your subcommand library named `helpme`.  This function will print help information to the screen.  Your function must have the following signature: `fn() -> !`.  It is recommended you use `carlotk::prelude::printhelp`, which has signature `fn(&str) -> !`.

5. Create the following entry in `Cargo.toml`.  Replace `x.x.x` with the version number of your subcommand's crate.

```
[dependencies.sc]
package = "carlo-sc"
version = "x.x.x"
```

6. In `src/main.rs`, add `subcommand sc` (on a new line) to the `include_subcommands!` macro invocation.