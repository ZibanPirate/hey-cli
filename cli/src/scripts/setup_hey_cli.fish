function hey
    set hey_setup_version 0.1.0
    set name fish
    set hey_cli (which hey)

    set hey_output ($hey_cli --shell-name $name --setup-version $hey_setup_version $argv)
    set stdout ($hey_cli --get-stdout $hey_output)
    set prompt ($hey_cli --get-prompt $hey_output)

    # TODO: skip echoing stdout if it's empty
    echo $stdout
    commandline -i "$prompt"
end
