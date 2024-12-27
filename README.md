# hey-cli

Ask your CLI, next command will be auto-generated.

## Install

**With Shell:**

```sh
curl -fsSL http://hey_cli.zak-man.com/install.sh | sh
```

**With PowerShell:**

```powershell
irm http://hey_cli.zak-man.com/install.ps1 | iex
```

## Usage

ask it

```sh
hey show cpu usage
```

the next prompt will be auto-generated 🪄:

```sh
top -o cpu -s 5
```

## Features

- [x] it just works, no setup or registration needed
- [ ] extends its capabilities by extensions
- [x] supported shells
    - [x] fish
    - [ ] bash
    - [ ] zsh
    - [ ] powershell

## Contributing

Contributions are welcome, please read [`CONTRIBUTING.md`](https://github.com/ZibanPirate/hey-cli/blob/main/CONTRIBUTING.md) to get started.

## License

Licensed under MIT (twitter: [@zibanpirate](https://twitter.com/zibanpirate)).
