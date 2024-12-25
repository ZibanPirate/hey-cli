# Develop locally

install `bacon`:

```sh
cargo install --locked bacon
```

then run code tasks inside `./cli` or `./server`

```sh
bacon [task]
```

# Code style

Rust files are formatted using

```sh
cargo fmt
```

Non-rust files are formatted using prettier

```sh
npx -y prettier --write .
```

# Deploy server (internal)

```sh
docker build --platform linux/amd64 . -t build_rust
docker run --platform linux/amd64 -v (pwd):/root/code -it build_rust
```
