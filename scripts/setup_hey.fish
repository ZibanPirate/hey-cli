# version=0.1.0
function hey
    set hey_cli (which hey)
    set hey_output ($hey_cli --shell fish $argv)
    commandline -r "$hey_output"
end
