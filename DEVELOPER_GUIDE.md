# Developer's Guide

## Incrementing the Version

The following instructions will guide you to incrementing the Carlo language
version number.

1. Open `src/main.rs`.  Change the `VERSION` string to the new version number.

2. Open `Cargo.toml`.  Change the version number towards the top of the file.

## Adding a Subcommand

The following instructions will guide you to adding a Carlo langauge subcommand.

1. Create `src/subcommands/your-new-subcommand.rs`.  Write a new function corresponding
to your subcommand

2. Open `src/main.rs`.  Add `Subcommand::YourNewSubcommand` to the `main` function match
statement.

3. Open `src/lib.rs`.  Add `./subcommands/help_your-new-subcommand.txt` to the `HELP`
hashmap.

4. Open `src/subcommands/mod.rs`.  Add `YourNewSubcommand` to the `Subcommand` enumeration.
Add `"your-new-subcommand" => YourNewSubcommand` in the `From<&str>` trait implementation
block.

5. Open `src/help.txt`.  Add `your-new-subcommand` to the help menu.

6. Create a file called `src/subcommands/help_your-new-subcommand.txt`.  Add a help menu.