// gitb pull / gitb fetch / gitb push: batch sync operations

use crate::core::executor::exec_git_on_repo;
use crate::core::{executor, git, output, GlobalOpts, Repo};

pub fn run_pull(repos: &[Repo], opts: &GlobalOpts) -> anyhow::Result<()> {
    executor::print_header(opts, "Pulling");
    executor::print_skip_info(opts, &opts.skip);

    let results = executor::execute_parallel(repos, opts, "Pull", |repo| {
        if opts.dry_run {
            return crate::core::GitResult::ok(
                &repo.name,
                &format!("[DRY-RUN] Would pull in {}", repo.name),
            );
        }

        let was_dirty = git::is_dirty(&repo.path) || git::has_untracked(&repo.path);

        if was_dirty && opts.force {
            git::run_git(&repo.name, &repo.path, &["checkout", "."]);
            git::run_git(&repo.name, &repo.path, &["clean", "-fd"]);
        }

        let stashed = if was_dirty && !opts.force {
            let stash_result = git::run_git(&repo.name, &repo.path, &["stash", "push", "-u"]);
            if !stash_result.success {
                return crate::core::GitResult::fail(
                    &repo.name,
                    "Failed to stash changes before pull",
                );
            }
            true
        } else {
            false
        };

        let pull_result = git::run_git(&repo.name, &repo.path, &["pull"]);

        if !pull_result.success {
            if stashed {
                git::run_git(&repo.name, &repo.path, &["stash", "pop"]);
            }
            return crate::core::GitResult {
                message: format!(
                    "Pull failed{}: {}",
                    if stashed { " (stash restored)" } else { "" },
                    pull_result.stderr.lines().last().unwrap_or("")
                ),
                ..pull_result
            };
        }

        if stashed {
            let pop_result = git::run_git(&repo.name, &repo.path, &["stash", "pop"]);
            if !pop_result.success {
                return crate::core::GitResult {
                    message: format!(
                        "Pulled, but stash pop failed: {}",
                        pop_result.stderr.lines().last().unwrap_or("")
                    ),
                    success: false,
                    ..pop_result
                };
            }
        }

        crate::core::GitResult::ok(
            &repo.name,
            &format!("Pulled{}", if stashed { " (stash restored)" } else { "" }),
        )
    });

    output::print_results(&results, opts.output, opts.quiet);
    Ok(())
}

pub fn run_fetch(repos: &[Repo], opts: &GlobalOpts) -> anyhow::Result<()> {
    executor::print_header(opts, "Fetching");
    executor::print_skip_info(opts, &opts.skip);

    let results = executor::execute_parallel(repos, opts, "Fetch", |repo| {
        exec_git_on_repo(
            repo,
            opts,
            &["fetch", "--all", "--prune"],
            "Fetched",
            "git fetch --all --prune",
        )
    });

    output::print_results(&results, opts.output, opts.quiet);
    Ok(())
}

pub fn run_push(repos: &[Repo], opts: &GlobalOpts) -> anyhow::Result<()> {
    executor::print_header(opts, "Pushing");
    executor::print_skip_info(opts, &opts.skip);

    let results = executor::execute_parallel(repos, opts, "Push", |repo| {
        exec_git_on_repo(repo, opts, &["push"], "Pushed", "git push")
    });

    output::print_results(&results, opts.output, opts.quiet);
    Ok(())
}
