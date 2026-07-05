// gitb exec <cmd...>: execute arbitrary git command across repos

use crate::core::{executor, git, output, GlobalOpts, Repo};

pub fn run(repos: &[Repo], opts: &GlobalOpts, args: &[String]) -> anyhow::Result<()> {
    if args.is_empty() {
        anyhow::bail!("No command specified. Usage: gitb exec <git command> [args...]");
    }

    let cmd_display = format!("git {}", args.join(" "));
    executor::print_header(opts, &format!("Executing: {}", cmd_display));
    executor::print_skip_info(opts, &opts.skip);

    // Convert args to &[&str] for git wrapper
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let results = executor::execute_parallel(repos, opts, "Exec", |repo| {
        if opts.dry_run {
            return crate::core::GitResult {
                repo_name: repo.name.clone(),
                success: true,
                exit_code: 0,
                stdout: String::new(),
                stderr: String::new(),
                message: format!("[DRY-RUN] {} in {}", cmd_display, repo.name),
            };
        }

        let result = git::run_git(&repo.name, &repo.path, &arg_refs);

        // For exec, we want to show the stdout output too
        crate::core::GitResult {
            message: if result.success {
                if result.stdout.is_empty() {
                    "OK".to_string()
                } else {
                    result.stdout.lines().take(1).collect::<Vec<_>>().join("")
                }
            } else {
                result.message
            },
            ..result
        }
    });

    // For exec, also print full stdout in verbose/table mode
    if opts.verbose && opts.output != crate::core::OutputFormat::Quiet {
        for r in &results {
            if !r.stdout.is_empty() {
                println!("\n--- {} ---", r.repo_name);
                print!("{}", r.stdout);
            }
        }
    }

    output::print_results(&results, opts.output, opts.quiet);
    Ok(())
}
