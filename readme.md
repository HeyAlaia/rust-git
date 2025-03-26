# Rgit

Implementing Git using Rust—rebuilding everything with Rust! :)

## Table of Contents

- [Testing](#testing)
- [Maintainers](#maintainers)
- [License](#license)

## Testing

### cat-file
```bash
cargo run -- cat-file -p ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba
```

### hash-object
```bash
cargo run -- hash-object -w .\Cargo.lock
```

### ls-tree
```bash
# only print name
cargo run -- ls-tree --name-only a27c94699d80dbc8ce96190cc8e07eadfc035df4
# print all info by tree
cargo run -- ls-tree a27c94699d80dbc8ce96190cc8e07eadfc035df4
```

## Maintainers

[@HeyAlaia](https://github.com/HeyAlaia).

## License

[MIT](LICENSE) © Richard Littauer.