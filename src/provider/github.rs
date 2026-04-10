use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

// ── Trigger ──────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    Push(Option<PushFilter>),
    PullRequest(Option<PullRequestFilter>),
    WorkflowDispatch,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PushFilter {
    pub branches: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "branches-ignore")]
    pub branches_ignore: Option<Vec<String>>,
    #[serde(rename = "tags-ignore")]
    pub tags_ignore: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PullRequestFilter {
    pub branches: Option<Vec<String>>,
    #[serde(rename = "branches-ignore")]
    pub branches_ignore: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
    pub types: Option<Vec<String>>,
}

// ── Step ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub name: Option<String>,
    pub run: Option<String>,
    #[serde(rename = "uses")]
    pub uses: Option<String>,
    #[serde(rename = "with")]
    pub with: Option<HashMap<String, serde_yaml::Value>>,
    pub env: Option<HashMap<String, String>>,
    #[serde(rename = "if")]
    pub condition: Option<String>,
    #[serde(rename = "continue-on-error")]
    pub continue_on_error: Option<bool>,
    #[serde(rename = "working-directory")]
    pub working_directory: Option<String>,
    pub shell: Option<String>,
}

// ── Matrix ───────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Matrix {
    #[serde(flatten)]
    pub dimensions: HashMap<String, serde_yaml::Value>,
    pub include: Option<Vec<HashMap<String, serde_yaml::Value>>>,
    pub exclude: Option<Vec<HashMap<String, serde_yaml::Value>>>,
}

/// Matrix can be a structured object or a raw `${{ expression }}` string.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatrixValue {
    Structured(Matrix),
    Expression(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Strategy {
    pub matrix: Option<MatrixValue>,
    #[serde(rename = "fail-fast")]
    pub fail_fast: Option<bool>,
}

// ── Job ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub name: Option<String>,
    #[serde(rename = "runs-on")]
    pub runs_on: serde_yaml::Value,
    pub needs: Option<JobNeeds>,
    pub steps: Option<Vec<Step>>,
    pub outputs: Option<HashMap<String, String>>,
    pub env: Option<HashMap<String, String>>,
    #[serde(rename = "if")]
    pub condition: Option<String>,
    pub strategy: Option<Strategy>,
}

/// `needs:` can be a single string or a list of strings.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobNeeds {
    One(String),
    Many(Vec<String>),
}

impl JobNeeds {
    pub fn as_slice(&self) -> Vec<&str> {
        match self {
            JobNeeds::One(s) => vec![s.as_str()],
            JobNeeds::Many(v) => v.iter().map(|s| s.as_str()).collect(),
        }
    }
}

// ── On (triggers) ────────────────────────────────────────────────────────────

/// The `on:` field supports many forms in GH Actions YAML — we keep it as raw
/// Value and provide helpers to extract what we need.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum On {
    /// `on: push`
    Single(String),
    /// `on: [push, pull_request]`
    List(Vec<String>),
    /// `on:\n  push:\n    branches: [main]`
    Map(HashMap<String, Option<serde_yaml::Value>>),
}

impl On {
    /// Return all trigger names present in the `on:` field.
    pub fn trigger_names(&self) -> Vec<String> {
        match self {
            On::Single(s) => vec![s.clone()],
            On::List(v) => v.clone(),
            On::Map(m) => m.keys().cloned().collect(),
        }
    }
}

// ── Workflow ──────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub name: Option<String>,
    #[serde(rename = "on")]
    pub on: On,
    pub env: Option<HashMap<String, String>>,
    pub jobs: HashMap<String, Job>,
}

// ── Parser ───────────────────────────────────────────────────────────────────

pub struct WorkflowParser;

impl WorkflowParser {
    pub fn parse(path: &Path) -> anyhow::Result<Workflow> {
        let contents = std::fs::read_to_string(path)?;
        let workflow: Workflow = serde_yaml::from_str(&contents)?;
        Ok(workflow)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_release_workflow() {
        let path = Path::new(".github/workflows/release.yml");
        let workflow = WorkflowParser::parse(path).expect("failed to parse release.yml");

        assert!(workflow.name.is_some());
        assert!(!workflow.jobs.is_empty());

        let triggers = workflow.on.trigger_names();
        assert!(triggers.iter().any(|t| t == "push" || t == "pull_request"));
    }
}
