use crate::utils::{PortTrait, ShellName, State};
use anyhow::Result;
use strum::IntoEnumIterator;

pub struct SetupScript;

impl State<()> for SetupScript {
    async fn next(self, port: &impl PortTrait) -> Result<()> {
        for shell_name in ShellName::iter() {
            port.log(format!("Installing setup script for shell: {shell_name}"));
            match shell_name {
                ShellName::Fish => {
                    let setup = shell_name.setup_script_content();
                    let fish_setup_relative_path = ".config/fish/functions/setup_hey_cli.fish";
                    let home_dir =
                        dirs::home_dir().ok_or(anyhow::anyhow!("Could not find home directory"))?;
                    let fish_setup_path = home_dir.join(fish_setup_relative_path);

                    port.overwrite_file(&fish_setup_path, setup)?;

                    let fish_config_path = home_dir.join(".config/fish/config.fish");
                    let config_source_line = format!("source ~/{}", fish_setup_relative_path);

                    port.remove_matches_from_file_content(&fish_config_path, &config_source_line)?;
                    port.append_to_file(&fish_config_path, &config_source_line)?;
                }
                ShellName::Zsh => {
                    let setup = shell_name.setup_script_content();
                    let zsh_setup_relative_path = ".config/zsh/functions/setup_hey_cli.zsh";
                    let home_dir =
                        dirs::home_dir().ok_or(anyhow::anyhow!("Could not find home directory"))?;
                    let zsh_setup_path = home_dir.join(zsh_setup_relative_path);

                    port.overwrite_file(&zsh_setup_path, setup)?;

                    let zshrc_path = home_dir.join(".zshrc");
                    let fpath_line = "fpath=(~/.config/zsh/functions $fpath)";
                    let source_line = format!("source ~/{}", zsh_setup_relative_path);
                    let autoload_line = "autoload -Uz hey";

                    for line in [fpath_line, &source_line, autoload_line] {
                        port.remove_matches_from_file_content(&zshrc_path, line)?;
                        port.append_to_file(&zshrc_path, line)?;
                    }
                }
                _ => {
                    // TODO: Implement setup for other shells
                    return Err(anyhow::anyhow!("{shell_name} shell is not yet supported"));
                }
            };
        }

        port.log("Setup script installed successfully");
        port.log("Please open new terminal session");
        Ok(())
    }
}
