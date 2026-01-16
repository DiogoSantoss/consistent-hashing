

# Learnings

To create this project folder
```
cargo new consistent-hashing
```

## Modules



## Crates

Two types of crates:
- binary - compile to an executable
- library - just define functionality

Most of the time "crate" means "library".

## Packages

A package is a bundle of one or more crates.
A package can contain as many binary crates as you like but at most only one library crate.

Convention: `src/main.rs` is the crate root of a binary crate.

For library crates is `src/lib.rs`
