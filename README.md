# hey-cli

Ask your CLI, next command will be auto-generated.

## Deploy server

```sh
docker build --platform linux/amd64 . -t build_rust
docker run --platform linux/amd64 -v (pwd):/root/code -it build_rust
```
