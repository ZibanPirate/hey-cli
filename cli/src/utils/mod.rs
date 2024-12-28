use anyhow::Result;
use hey_cli_common::{GetCliPromptRequestQuery, GetCliPromptResponse};
use nest_struct::nest_struct;
use std::{collections::HashMap, path::Path, str::pattern::Pattern, sync::Mutex};
use strum_macros::{Display, EnumString};

pub trait State<N> {
    async fn next(self, port: &impl PortTrait) -> Result<N>;
}

#[nest_struct]
pub struct Shell {
    pub setup_version: String,
    pub name: nest! {
        #[derive(EnumString, Display)]
        #[strum(serialize_all = "snake_case")]
        pub enum ShellName {
            Fish,
            Bash,
            Zsh,
            PowerShell,
        }
    },
}

impl Shell {
    pub fn expected_setup_version(&self) -> &str {
        match self.name {
            ShellName::Fish => self
                .name
                .setup_script_content()
                .lines()
                .find_map(|line| {
                    if line.contains("set hey_setup_version") {
                        Some(line.split_whitespace().last().unwrap())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| panic!("Could not find setup version for {:}", self.name)),
            _ => todo!("implement expected_setup_version for {:}", self.name),
        }
    }
}

impl ShellName {
    pub fn detect(port: &impl PortTrait) -> Result<Self> {
        if port.get_env_var("FISH_VERSION").is_some()
            || port
                .get_env_var("SHELL")
                .unwrap_or_default()
                .ends_with("/fish")
        {
            return Ok(Self::Fish);
        }

        if port.get_env_var("BASH_VERSION").is_some() {
            return Ok(Self::Bash);
        }

        if port.get_env_var("ZSH_VERSION").is_some() {
            return Ok(Self::Zsh);
        }

        if port.get_env_var("PSModulePath").is_some() {
            return Ok(Self::PowerShell);
        }

        Err(anyhow::anyhow!("No supported shell detected"))
    }

    pub fn setup_script_content(&self) -> &str {
        match self {
            ShellName::Fish => include_str!("../scripts/setup_hey_cli.fish"),
            _ => todo!("implement setup_script_content for {:}", self),
        }
    }
}

#[derive(Debug)]
pub struct Port {
    pub logs: Vec<String>,
    pub final_prompt: Option<String>,
    pub env_vars: HashMap<String, String>,
}

impl Port {
    pub fn new_mutex() -> Mutex<Self> {
        let env_vars = std::env::vars().collect();
        Self::new_mutex_with_env_vars(env_vars)
    }

    pub fn new_mutex_with_env_vars(
        env_vars: Vec<(impl Into<String>, impl Into<String>)>,
    ) -> Mutex<Self> {
        let env_vars = env_vars
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        Mutex::new(Self {
            logs: vec![],
            final_prompt: None,
            env_vars,
        })
    }
}

pub trait PortTrait {
    fn log(&self, log: impl Into<String>);
    fn set_final_prompt(&self, prompt: String);
    fn to_stdout_format(&self) -> impl Into<String>;
    fn get_env_var(&self, key: &str) -> Option<String>;
    fn overwrite_file(&self, path: &Path, content: &str) -> Result<()>;
    fn remove_matches_from_file_content(&self, path: &Path, pattern: impl Pattern) -> Result<()>;
    fn append_to_file(&self, path: &Path, content: &str) -> Result<()>;
    async fn ask_server_for_prompt(
        &self,
        query: GetCliPromptRequestQuery,
    ) -> Result<GetCliPromptResponse>;
}

// TODO: on non-test env, log directly to stdout, both logs and final_prompt
impl PortTrait for Mutex<Port> {
    fn log(&self, log: impl Into<String>) {
        let log = log.into();
        let mut port = self.lock().unwrap();
        port.logs.push(log);
    }

    // TODO: try to enforce this to be called only once using Rust's type system
    fn set_final_prompt(&self, prompt: String) {
        let mut port = self.lock().unwrap();
        port.final_prompt = Some(prompt);
    }

    fn to_stdout_format(&self) -> impl Into<String> {
        let port = self.lock().unwrap();
        let logs = port.logs.join("\n");
        let final_prompt = match port.final_prompt.as_ref() {
            Some(prompt) => format!("\nhey-cli-prompt-start\n{}", prompt),
            None => "".to_string(),
        };

        format!("{logs}{final_prompt}")
    }

    fn get_env_var(&self, key: &str) -> Option<String> {
        let port = self.lock().unwrap();
        port.env_vars.get(key).cloned()
    }

    #[cfg(test)]
    fn overwrite_file(&self, _: &Path, _: &str) -> Result<()> {
        Ok(())
    }
    #[cfg(not(test))]
    fn overwrite_file(&self, path: &Path, content: &str) -> Result<()> {
        use std::io::prelude::Write;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(content.as_bytes())?;

        Ok(())
    }

    #[cfg(test)]
    fn remove_matches_from_file_content(&self, _: &Path, _: impl Pattern) -> Result<()> {
        Ok(())
    }
    #[cfg(not(test))]
    fn remove_matches_from_file_content(&self, path: &Path, pattern: impl Pattern) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if !path.exists() {
            std::fs::write(path, "")?;
        }

        let mut content = std::fs::read_to_string(path)?;
        content.remove_matches(pattern);

        std::fs::write(path, content)?;

        Ok(())
    }

    #[cfg(test)]
    fn append_to_file(&self, _: &Path, _: &str) -> Result<()> {
        Ok(())
    }
    #[cfg(not(test))]
    fn append_to_file(&self, path: &Path, content: &str) -> Result<()> {
        use std::io::prelude::Write;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let original_content = std::fs::read_to_string(path)?;
        let ends_with_newline = original_content.ends_with('\n');

        let mut file = std::fs::OpenOptions::new().append(true).open(path)?;

        if !ends_with_newline {
            file.write_all(b"\n")?;
        }
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    #[cfg(test)]
    async fn ask_server_for_prompt(
        &self,
        query: GetCliPromptRequestQuery,
    ) -> Result<GetCliPromptResponse> {
        use hey_cli_common::CliPrompt;

        Ok(GetCliPromptResponse {
            prompt: CliPrompt {
                value: format!("echo \"{}\"", query.q),
            },
        })
    }
    #[cfg(not(test))]
    async fn ask_server_for_prompt(
        &self,
        query: GetCliPromptRequestQuery,
    ) -> Result<GetCliPromptResponse> {
        use urlencoding::encode;

        #[cfg(not(debug_assertions))]
        let server_url = "http://hey_cli.zak-man.com";
        #[cfg(debug_assertions)]
        let server_url = "http://0.0.0.0:3000";

        #[cfg(debug_assertions)]
        {
            // wait on /health and retry every 1 seconds
            let url = format!("{server_url}/health");
            loop {
                let resp = reqwest::get(&url).await;
                match resp {
                    Ok(_) => {
                        tracing::info!("Server is up.");
                        break;
                    }
                    Err(e) => {
                        tracing::warn!("Server is not up yet: {e}");
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }

        let ask = encode(&query.q);
        let url = format!("{server_url}/cli-prompt?q={ask}");
        let resp = reqwest::get(url).await?;
        let resp = resp.json::<GetCliPromptResponse>().await?;

        Ok(resp)
    }
}
