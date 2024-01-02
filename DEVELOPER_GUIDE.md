# Developer's Guide

## Incrementing the Version

The following instructions will guide you to incrementing the Carlo language
version number.

1. Open `src/main.rs`.  Change the `VERSION` string to the new version number.

2. Open `Cargo.toml`.  Change the version number towards the top of the file.

## Adding a Subcommand

The following instructions will guide you to adding a Carlo langauge subcommand.

1. Open `src/main.rs`.  Write a new function corresponding to your subcommand and
add `Subcommand::YourNewSubcommand` to the `main` function match statement.

2. Open `src/cli.rs`.  Add `YourNewSubcommand` to the `Subcommand` enumeration.
Add `"your-new-subcommand" => YourNewSubcommand` in the `From<&str>` trait
implementation block.

3. Open `src/help.txt`.  Add `your-new-subcommand` to the help menu.