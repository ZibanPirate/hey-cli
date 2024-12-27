use crate::utils::{PortTrait, ShellName, State};
use anyhow::Result;

pub struct SetupScript(pub Option<ShellName>);

impl State<()> for SetupScript {
    async fn next(self, port: &impl PortTrait) -> Result<()> {
        let shell_name = match self.0 {
            Some(shell_name) => shell_name,
            None => ShellName::detect(port)?,
        };

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
            _ => {
                return Err(anyhow::anyhow!("{shell_name} shell is not yet supported"));
            }
        };

        port.log("Setup script installed successfully");
        port.log("Please open new terminal session");
        Ok(())
    }
}
