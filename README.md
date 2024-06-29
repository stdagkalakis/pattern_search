# Pattern search

This rust cli tool was created for training purposes. It reads a word and searches
for it inside the provide file path. It then return the line number and the full text of the line
where the pattern was found.


## Run

```bash
 cargo run <pattern> <path>
```

## Run with logs

```bash
 RUST_LOG=info cargo run <pattern> <path>
```

## Run tests

```bash
 cargo test
```