// Parallel executor: run operations across repos with configurable parallelism

use crate::core::{GitResult, GlobalOpts, Repo};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Execute a closure on each repo in parallel.
///
/// - `repos`: list of repos to process
/// - `opts`: global options (jobs, dry_run, quiet)
/// - `action_name`: human-readable action name for progress bar (e.g. "Checking out")
/// - `action`: closure that takes (&Repo) and returns GitResult
///
/// Returns results sorted by repo name.
pub fn execute_parallel<F>(
    repos: &[Repo],
    opts: &GlobalOpts,
    action_name: &str,
    action: F,
) -> Vec<GitResult>
where
    F: Fn(&Repo) -> GitResult + Send + Sync,
{
    if repos.is_empty() {
        return Vec::new();
    }

    // Configure thread pool
    let num_jobs = if opts.jobs == 0 {
        num_cpus()
    } else {
        opts.jobs
    };

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_jobs)
        .build()
        .expect("failed to create thread pool");

    let action = Arc::new(action);
    let total = repos.len();
    let completed = Arc::new(AtomicUsize::new(0));

    // Progress bar (skip in quiet or json mode)
    let progress_bar = if opts.quiet || opts.output == crate::core::OutputFormat::Json {
        None
    } else {
        let pb = ProgressBar::new(total as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} {prefix:<12} [{bar:30.cyan/blue}] {pos}/{len} ({msg})",
            )
            .unwrap()
            .progress_chars("=> "),
        );
        pb.set_prefix(action_name.to_string());
        Some(pb)
    };

    let results: Vec<GitResult> = pool.install(|| {
        repos
            .par_iter()
            .map(|repo| {
                let result = action(repo);

                // Update progress
                let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
                if let Some(ref pb) = progress_bar {
                    pb.set_message(repo.name.clone());
                    pb.inc(1);
                    if done == total {
                        pb.finish_with_message("done");
                    }
                }

                result
            })
            .collect()
    });

    // Sort results by repo name for deterministic output
    let mut results = results;
    results.sort_by(|a, b| a.repo_name.cmp(&b.repo_name));
    results
}

/// Execute a git command on a single repo (helper used by command implementations).
///
/// If dry_run is true, returns a dry-run result without executing.
pub fn exec_git_on_repo(
    repo: &Repo,
    opts: &GlobalOpts,
    args: &[&str],
    success_msg: &str,
    dry_run_msg: &str,
) -> GitResult {
    if opts.dry_run {
        return GitResult {
            repo_name: repo.name.clone(),
            success: true,
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
            message: format!("[DRY-RUN] {}", dry_run_msg),
        };
    }
    let result = crate::core::git::run_git(&repo.name, &repo.path, args);
    if result.success {
        GitResult {
            message: success_msg.to_string(),
            ..result
        }
    } else {
        result
    }
}

/// Get the number of available CPUs
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

/// Print a header for the operation (non-quiet mode)
pub fn print_header(opts: &GlobalOpts, title: &str) {
    if opts.quiet || opts.output == crate::core::OutputFormat::Json {
        return;
    }
    if opts.dry_run {
        println!("\n{} [DRY-RUN]", title);
    } else {
        println!("\n{}", title);
    }
    println!("{}", "=".repeat(title.chars().count() + 12));
}

/// Print skip info
pub fn print_skip_info(opts: &GlobalOpts, skip: &[String]) {
    if skip.is_empty() || opts.quiet || opts.output == crate::core::OutputFormat::Json {
        return;
    }
    println!("[SKIP] Directories: {}", skip.join(", "));
    println!();
}
