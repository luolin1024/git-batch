// Core module: shared types, repo discovery, git wrapper, parallel executor, output

pub mod discovery;
pub mod executor;
pub mod git;
pub mod output;

use std::path::PathBuf;

/// A discovered git repository
#[derive(Debug, Clone)]
pub struct Repo {
    /// Absolute path to the repo directory
    pub path: PathBuf,
    /// Directory name (display name)
    pub name: String,
}

/// Result of executing a git command on a repo
#[derive(Debug, Clone)]
pub struct GitResult {
    pub repo_name: String,
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    /// Human-readable status message for display
    pub message: String,
}

impl GitResult {
    pub fn ok(repo_name: &str, message: &str) -> Self {
        Self {
            repo_name: repo_name.to_string(),
            success: true,
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
            message: message.to_string(),
        }
    }

    pub fn fail(repo_name: &str, message: &str) -> Self {
        Self {
            repo_name: repo_name.to_string(),
            success: false,
            exit_code: 1,
            stdout: String::new(),
            stderr: String::new(),
            message: message.to_string(),
        }
    }
}

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, serde::Serialize)]
pub enum OutputFormat {
    Table,
    Json,
    Quiet,
}

/// Global options shared across all commands
#[derive(Debug, Clone)]
pub struct GlobalOpts {
    /// Parallel job count (0 = auto = num_cpus)
    pub jobs: usize,
    /// Dry-run mode: print what would happen without executing
    pub dry_run: bool,
    /// Directories to skip (comma-separated names)
    pub skip: Vec<String>,
    /// Max recursion depth for repo discovery (0 = current dir only, 1 = one level down, etc.)
    pub depth: usize,
    /// Output format
    pub output: OutputFormat,
    /// Verbose logging
    pub verbose: bool,
    /// Quiet mode (errors only)
    pub quiet: bool,
    /// Force flag (discard changes)
    pub force: bool,
}

impl Default for GlobalOpts {
    fn default() -> Self {
        Self {
            jobs: 0,
            dry_run: false,
            skip: Vec::new(),
            depth: 1,
            output: OutputFormat::Table,
            verbose: false,
            quiet: false,
            force: false,
        }
    }
}
