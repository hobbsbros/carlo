# Developer's Guide

## Incrementing the Version

The following instructions will guide you to incrementing the Carlo language
version number.

1. Open `src/main.rs`.  Change the `VERSION` string to the new version number.

2. Open `Cargo.toml`.  Change the version number towards the top of the file.

## Adding a Subcommand

The following instructions will guide you to adding a Carlo langauge subcommand.

1. Create a Cargo library package in the `subcommands` directory with the name `sc`, where `sc` is your desired subcommand name.

2. Create a function in your subcommand library named `sc::sc`.  It is recommended that you import `carlotk::prelude::*` so you can use the Carlo tokenizer, parser, and environment utilities.  Your function must have the following signature: `fn(CliArgs) -> ()`.

3. Create the following entry in `Cargo.toml`.

```
[dependencies.sc]
path = "./subcommands/sc"
```

4. In `src/main.rs`, add `subcommand sc` (on a new line) to the `include_subcommands!` macro invocation.