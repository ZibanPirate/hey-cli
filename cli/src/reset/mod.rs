use crate::utils::{PortTrait, ShellName, State};
use anyhow::Result;
use strum::IntoEnumIterator;

pub struct Reset;

impl State<()> for Reset {
    async fn next(self, port: &impl PortTrait) -> Result<()> {
        port.log("Resetting hey-cli setup for all shells...");

        let home_dir = dirs::home_dir().ok_or(anyhow::anyhow!("Could not find home directory"))?;

        for shell_name in ShellName::iter() {
            port.log(format!("Cleaning up {shell_name} shell configuration..."));

            match shell_name {
                ShellName::Fish => {
                    let fish_setup_path =
                        home_dir.join(".config/fish/functions/setup_hey_cli.fish");
                    let fish_config_path = home_dir.join(".config/fish/config.fish");
                    let config_source_line =
                        format!("source ~/{}", ".config/fish/functions/setup_hey_cli.fish");

                    port.remove_matches_from_file_content(&fish_config_path, &config_source_line)?;

                    if let Err(e) = std::fs::remove_file(&fish_setup_path) {
                        port.log(format!(
                            "Note: Could not remove setup file ({}): {e}",
                            fish_setup_path.display()
                        ));
                    }
                }
                ShellName::Zsh => {
                    let zsh_setup_path = home_dir.join(".config/zsh/functions/setup_hey_cli.zsh");
                    let zshrc_path = home_dir.join(".zshrc");
                    let fpath_line = "fpath=(~/.config/zsh/functions $fpath)";
                    let source_line =
                        format!("source ~/{}", ".config/zsh/functions/setup_hey_cli.zsh");
                    let autoload_line = "autoload -Uz hey";

                    for line in [fpath_line, &source_line, autoload_line] {
                        port.remove_matches_from_file_content(&zshrc_path, line)?;
                    }

                    if let Err(e) = std::fs::remove_file(&zsh_setup_path) {
                        port.log(format!(
                            "Note: Could not remove setup file ({}): {e}",
                            zsh_setup_path.display()
                        ));
                    }
                }
                _ => {
                    // todo: Implement cleanup for other shells
                    port.log(format!("No cleanup needed for {shell_name} shell"));
                }
            };
        }

        port.log("Reset completed successfully");
        port.log("Please open new terminal session for changes to take effect");
        Ok(())
    }
}
