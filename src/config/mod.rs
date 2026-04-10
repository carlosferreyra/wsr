use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

const SCHEMA_URL: &str = "https://wsr.dev/schema/wsr.json";
const CONFIG_FILE: &str = "wsr.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub provider: Provider,
    pub sandbox: SandboxConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    #[default]
    Github,
    Gitlab,
    Bitbucket,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub allowed_hosts: Vec<String>,
    pub secrets_from: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            schema: SCHEMA_URL.to_string(),
            provider: Provider::Github,
            sandbox: SandboxConfig {
                allowed_hosts: vec![],
                secrets_from: ".env.wsr".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load(root: &Path) -> anyhow::Result<Self> {
        let path = root.join(CONFIG_FILE);
        let contents = fs::read_to_string(&path)?;
        let config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn generate(root: &Path) -> anyhow::Result<Self> {
        let config = Config::default();
        let path = root.join(CONFIG_FILE);
        let contents = serde_json::to_string_pretty(&config)?;
        fs::write(&path, contents)?;
        Ok(config)
    }
}
