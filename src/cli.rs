// CLI definitions using clap derive

use crate::core::OutputFormat;
use clap::{Parser, Subcommand};

/// Blazing-fast cross-platform multi-repository git batch tool.
#[derive(Parser, Debug)]
#[command(
    name = "gitb",
    version,
    about = "Blazing-fast multi-repo git batch tool",
    long_about = "gitb manages multiple git repositories in parallel.\nBuilt in Rust for maximum performance. Zero config required."
)]
pub struct Cli {
    /// Number of parallel jobs (0 = auto-detect CPU count)
    #[arg(long, short = 'j', global = true, default_value = "0")]
    pub jobs: usize,

    /// Dry-run mode: show what would happen without executing
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Skip directories (comma-separated, e.g. -s dir1,dir2)
    #[arg(long, short = 's', value_delimiter = ',', global = true)]
    pub skip: Vec<String>,

    /// Max recursion depth for repo discovery (default: 1 = immediate children)
    #[arg(long, short = 'd', global = true, default_value = "1")]
    pub depth: usize,

    /// Output format
    #[arg(long, short = 'o', value_enum, global = true, default_value = "table")]
    pub output: OutputFormat,

    /// Force operation (discard uncommitted changes)
    #[arg(long, short = 'f', global = true)]
    pub force: bool,

    /// Verbose output
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    /// Quiet mode (only show errors)
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,

    /// Filter repos by group name (from gitb.toml)
    #[arg(long, short = 'g', global = true)]
    pub group: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Switch to a branch across all repos (supports fuzzy matching)
    Checkout {
        /// Branch name to switch to (supports partial/fuzzy match)
        branch: String,
    },

    /// Create and switch to a new branch across all repos
    Create {
        /// Branch name to create
        branch: String,
    },

    /// Show colored multi-repo status overview
    Status,

    /// Pull from remote across all repos
    Pull,

    /// Fetch from remote across all repos
    Fetch,

    /// Push to remote across all repos
    Push,

    /// Execute an arbitrary git command across all repos
    Exec {
        /// Git command and args (e.g. `gitb exec log --oneline -5`)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Branch operations across repos
    Branch {
        #[command(subcommand)]
        action: BranchAction,
    },

    /// Commit changes across all repos
    Commit {
        /// Commit message
        #[arg(short = 'm', long)]
        message: String,

        /// Stage all changes before committing (git add -A)
        #[arg(long, short = 'a')]
        all: bool,
    },

    /// Stash operations across repos
    Stash {
        #[command(subcommand)]
        action: Option<StashAction>,
    },

    /// Smart rebase: stash → rebase → unstash across all repos
    Rebase {
        /// Branch to rebase onto (default: upstream)
        #[arg(long, short = 'b')]
        branch: Option<String>,
    },

    /// Show diff across all repos
    Diff,

    /// Show log across all repos
    Log {
        /// Number of commits to show per repo
        #[arg(long, short = 'n', default_value = "5")]
        number: usize,
    },

    /// Health check: detect ahead/behind/dirty/unpushed repos
    Doctor,

    /// Manage repo groups
    Group {
        #[command(subcommand)]
        action: GroupAction,
    },

    /// Initialize a workspace config (gitb.toml) interactively
    Init,

    /// Generate shell completion scripts
    Completion {
        /// Shell type
        shell: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum BranchAction {
    /// List all branches across repos
    List,
    /// Delete a branch across all repos
    Delete {
        /// Branch name to delete
        name: String,
        /// Force delete (git branch -D)
        #[arg(long, short = 'f')]
        force: bool,
        /// Also delete from remote
        #[arg(long)]
        remote: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum StashAction {
    /// Stash changes (default action)
    Push,
    /// Pop the latest stash
    Pop,
    /// List stashes
    List,
    /// Clear all stashes
    Clear,
}

#[derive(Subcommand, Debug, Clone)]
pub enum GroupAction {
    /// Add a new group
    Add {
        /// Group name
        name: String,
        /// Comma-separated repo names to include
        #[arg(value_delimiter = ',')]
        repos: Vec<String>,
    },
    /// List all groups
    List,
    /// Remove a group
    Remove {
        /// Group name
        name: String,
    },
    /// Show repos in a group
    Show {
        /// Group name
        name: String,
    },
}
