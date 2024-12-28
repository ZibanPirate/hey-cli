#!/bin/sh

# exit when any command fails
set -e

# map to rust target
if [ "$OS" = "Windows_NT" ]; then
	target="x86_64-pc-windows-msvc.exe"
else
	case $(uname -sm) in
	"Darwin x86_64") target="x86_64-apple-darwin" ;;
	"Darwin arm64") target="aarch64-apple-darwin" ;;
	"Linux aarch64") target="aarch64-unknown-linux-gnu" ;;
	*) target="x86_64-unknown-linux-gnu" ;;
	esac
fi

# vars
hey_cli_uri="https://github.com/ZibanPirate/hey-cli/releases/latest/download/hey-${target}"
hey_cli_install="${HEY_CLI_INSTALL:-$HOME/.hey_cli}"
bin_dir="$hey_cli_install/bin"
exe="$bin_dir/hey"

# ensure bin directory exists
if [ ! -d "$bin_dir" ]; then
	mkdir -p "$bin_dir"
fi

# download and move to bin
if ! curl --fail --location --progress-bar --output "$exe" "$hey_cli_uri"; then
    echo "Error: Failed to download hey-cli."
    echo "Please check your internet connection and ensure you can access:"
    echo "$hey_cli_uri"
    echo
    echo "If the problem persists, please report this issue at:"
    echo "https://github.com/ZibanPirate/hey-cli/issues"
    exit 1
fi

# set permissions
chmod +x "$exe"

# add to PATH if not already present
shell_config_files="$HOME/.profile $HOME/.bashrc $HOME/.zshrc $HOME/.config/fish/config.fish"
for config_file in $shell_config_files; do
    if [ -f "$config_file" ]; then
        # echo \$HOME instead of the actual path
        if ! grep -q "$bin_dir" "$config_file"; then
            if [ "${config_file##*.}" = "fish" ]; then
                echo "\nset -gx PATH \"$bin_dir\" \$PATH" >> "$config_file"
            else
                echo "\nexport PATH=\"$bin_dir:\$PATH\"" >> "$config_file"
            fi
            echo "Added to PATH in $config_file."
        fi
    fi
done

# print success message
echo "hey_cli was installed successfully to $exe"
echo "Please restart your terminal."
echo
echo "Stuck? contact me on: https://zak-man.com"
