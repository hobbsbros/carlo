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

1. Create a Cargo library package in the `subcommands` directory with the name `sc`, where `sc` is your desired subcommand name.  The package should have name `carlo-sc`.

2. Add `carlotk` to your subcommand library's `Cargo.toml` file.

3. Create a function in your subcommand library named `sc`.  It is recommended that you import `carlotk::prelude::*` so you can use the Carlo tokenizer, parser, and environment utilities.  Your function must have the following signature: `fn(CliArgs) -> ()`.

4. Create the following entry in `Cargo.toml`.

```
[dependencies.sc]
path = "./subcommands/sc"
```

5. In `src/main.rs`, add `subcommand sc` (on a new line) to the `include_subcommands!` macro invocation.