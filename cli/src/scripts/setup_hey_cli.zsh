hey() {
    local hey_setup_version="0.1.0"
    local name="zsh"
    # TODO: check if `hey` is in path, otherwise print install instructions
    local hey_cli=$(command which hey)

    local hey_output=$($hey_cli --shell-name $name --setup-version $hey_setup_version $@)
    local stdout=$($hey_cli --get-stdout $hey_output)
    local prompt=$($hey_cli --get-prompt $hey_output)

    # TODO: skip echoing stdout if it's empty
    echo $stdout
    print -z $prompt
}
