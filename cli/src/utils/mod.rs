use anyhow::Result;
use hey_cli_common::{GetCliPromptRequestBody, GetCliPromptResponse};
use nest_struct::nest_struct;
use std::{path::Path, sync::Mutex};
use strum_macros::{Display, EnumIter, EnumString};

pub trait State<N> {
    async fn next(self, port: &impl PortTrait) -> Result<N>;
}

#[nest_struct]
pub struct Shell {
    pub setup_version: String,
    pub name: nest! {
        #[derive(EnumString, EnumIter, Display, Debug)]
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
            ShellName::Zsh => self
                .name
                .setup_script_content()
                .lines()
                .find_map(|line| {
                    if line.contains("local hey_setup_version=") {
                        Some(line.split('"').nth(1).unwrap())
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
    pub fn setup_script_content(&self) -> &str {
        match self {
            ShellName::Fish => include_str!("../scripts/setup_hey_cli.fish"),
            ShellName::Zsh => include_str!("../scripts/setup_hey_cli.zsh"),
            _ => todo!("implement setup_script_content for {:}", self),
        }
    }
}

#[derive(Debug)]
pub struct Port {
    pub logs: Vec<String>,
    pub final_prompt: Option<String>,
}

impl Port {
    pub fn new_mutex() -> Mutex<Self> {
        Mutex::new(Self {
            logs: vec![],
            final_prompt: None,
        })
    }
}

pub trait PortTrait {
    fn log(&self, log: impl Into<String>);
    fn set_final_prompt(&self, prompt: String);
    fn to_stdout_format(&self) -> impl Into<String>;
    fn overwrite_file(&self, path: &Path, content: &str) -> Result<()>;
    fn remove_matches_from_file_content(&self, path: &Path, pattern: &str) -> Result<()>;
    fn append_to_file(&self, path: &Path, content: &str) -> Result<()>;
    async fn ask_server_for_prompt(
        &self,
        query: GetCliPromptRequestBody,
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
    fn remove_matches_from_file_content(&self, _: &Path, _: &str) -> Result<()> {
        Ok(())
    }
    #[cfg(not(test))]
    fn remove_matches_from_file_content(&self, path: &Path, pattern: &str) -> Result<()> {
        use regex::Regex;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if !path.exists() {
            std::fs::write(path, "")?;
        }
        let content = std::fs::read_to_string(path)?;
        let escaped_pattern = regex::escape(pattern);
        let re = Regex::new(&escaped_pattern)?;
        let new_content = re.replace_all(&content, "").to_string();
        std::fs::write(path, new_content)?;
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
        query: GetCliPromptRequestBody,
    ) -> Result<GetCliPromptResponse> {
        use hey_cli_common::CliPrompt;

        Ok(GetCliPromptResponse {
            prompt: CliPrompt {
                value: format!("echo \"{}\"", query.ask),
            },
        })
    }
    #[cfg(not(test))]
    async fn ask_server_for_prompt(
        &self,
        query: GetCliPromptRequestBody,
    ) -> Result<GetCliPromptResponse> {
        #[cfg(not(debug_assertions))]
        let server_url = "https://hey-cli.zak-man.com";
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

        let url = format!("{server_url}/cli-prompt");
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .json(&query)
            .send()
            .await?
            .json::<GetCliPromptResponse>()
            .await?;

        Ok(resp)
    }
}
