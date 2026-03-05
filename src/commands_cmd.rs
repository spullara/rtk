use anyhow::Result;

struct CommandInfo {
    name: &'static str,
    category: &'static str,
    savings: &'static str,
    description: &'static str,
}

static COMMANDS: &[CommandInfo] = &[
    // Git
    CommandInfo {
        name: "git",
        category: "Git",
        savings: "70-80%",
        description: "Git commands with compact output",
    },
    CommandInfo {
        name: "gh",
        category: "Git",
        savings: "80-87%",
        description: "GitHub CLI (pr, issue, run, repo)",
    },
    // Cargo/Rust
    CommandInfo {
        name: "cargo",
        category: "Cargo/Rust",
        savings: "80-90%",
        description: "Cargo commands (build, test, clippy, check)",
    },
    // Go
    CommandInfo {
        name: "go",
        category: "Go",
        savings: "80-90%",
        description: "Go commands (test, build, vet)",
    },
    CommandInfo {
        name: "golangci-lint",
        category: "Go",
        savings: "85%",
        description: "golangci-lint with compact output",
    },
    // Python
    CommandInfo {
        name: "ruff",
        category: "Python",
        savings: "80%",
        description: "Ruff linter/formatter",
    },
    CommandInfo {
        name: "pytest",
        category: "Python",
        savings: "90%",
        description: "Pytest test runner",
    },
    CommandInfo {
        name: "mypy",
        category: "Python",
        savings: "80%",
        description: "Mypy type checker",
    },
    CommandInfo {
        name: "pip",
        category: "Python",
        savings: "70-85%",
        description: "pip/uv package manager",
    },
    // JS/TS
    CommandInfo {
        name: "tsc",
        category: "JS/TS",
        savings: "83%",
        description: "TypeScript compiler",
    },
    CommandInfo {
        name: "next",
        category: "JS/TS",
        savings: "87%",
        description: "Next.js build",
    },
    CommandInfo {
        name: "lint",
        category: "JS/TS",
        savings: "84%",
        description: "ESLint/Biome linter",
    },
    CommandInfo {
        name: "prettier",
        category: "JS/TS",
        savings: "70%",
        description: "Prettier format checker",
    },
    CommandInfo {
        name: "format",
        category: "JS/TS",
        savings: "70%",
        description: "Universal format checker",
    },
    CommandInfo {
        name: "playwright",
        category: "JS/TS",
        savings: "94%",
        description: "Playwright E2E tests",
    },
    CommandInfo {
        name: "vitest",
        category: "JS/TS",
        savings: "99%",
        description: "Vitest test runner",
    },
    CommandInfo {
        name: "prisma",
        category: "JS/TS",
        savings: "88%",
        description: "Prisma CLI",
    },
    CommandInfo {
        name: "pnpm",
        category: "JS/TS",
        savings: "70-90%",
        description: "pnpm package manager",
    },
    CommandInfo {
        name: "npm",
        category: "JS/TS",
        savings: "50%",
        description: "npm run with filtered output",
    },
    CommandInfo {
        name: "npx",
        category: "JS/TS",
        savings: "varies",
        description: "npx with intelligent routing",
    },
    // Files
    CommandInfo {
        name: "ls",
        category: "Files",
        savings: "70%",
        description: "Directory listing",
    },
    CommandInfo {
        name: "tree",
        category: "Files",
        savings: "70%",
        description: "Directory tree",
    },
    CommandInfo {
        name: "read",
        category: "Files",
        savings: "60%",
        description: "File reading with filtering",
    },
    CommandInfo {
        name: "grep",
        category: "Files",
        savings: "70%",
        description: "Compact grep/ripgrep",
    },
    CommandInfo {
        name: "find",
        category: "Files",
        savings: "70%",
        description: "Find files with tree output",
    },
    CommandInfo {
        name: "diff",
        category: "Files",
        savings: "70%",
        description: "Ultra-condensed diff",
    },
    CommandInfo {
        name: "json",
        category: "Files",
        savings: "80%",
        description: "JSON structure viewer",
    },
    CommandInfo {
        name: "wc",
        category: "Files",
        savings: "50%",
        description: "Word/line/byte count",
    },
    CommandInfo {
        name: "log",
        category: "Files",
        savings: "80%",
        description: "Log deduplication",
    },
    CommandInfo {
        name: "deps",
        category: "Files",
        savings: "60%",
        description: "Project dependency summary",
    },
    CommandInfo {
        name: "env",
        category: "Files",
        savings: "70%",
        description: "Environment variables (filtered)",
    },
    CommandInfo {
        name: "smart",
        category: "Files",
        savings: "60%",
        description: "Heuristic file summary",
    },
    CommandInfo {
        name: "summary",
        category: "Files",
        savings: "50%",
        description: "Command output summary",
    },
    // Infra
    CommandInfo {
        name: "docker",
        category: "Infra",
        savings: "85%",
        description: "Docker commands",
    },
    CommandInfo {
        name: "kubectl",
        category: "Infra",
        savings: "85%",
        description: "Kubernetes commands",
    },
    CommandInfo {
        name: "aws",
        category: "Infra",
        savings: "75%",
        description: "AWS CLI",
    },
    CommandInfo {
        name: "psql",
        category: "Infra",
        savings: "70%",
        description: "PostgreSQL client",
    },
    // Network
    CommandInfo {
        name: "curl",
        category: "Network",
        savings: "70%",
        description: "Curl with JSON detection",
    },
    CommandInfo {
        name: "wget",
        category: "Network",
        savings: "50%",
        description: "Download (strip progress)",
    },
    // Generic
    CommandInfo {
        name: "err",
        category: "Generic",
        savings: "80%",
        description: "Show only errors/warnings",
    },
    CommandInfo {
        name: "test",
        category: "Generic",
        savings: "90%",
        description: "Show only test failures",
    },
    CommandInfo {
        name: "proxy",
        category: "Generic",
        savings: "0%",
        description: "Execute without filtering",
    },
    // Meta
    CommandInfo {
        name: "init",
        category: "Meta",
        savings: "\u{2014}",
        description: "Initialize rtk in project",
    },
    CommandInfo {
        name: "gain",
        category: "Meta",
        savings: "\u{2014}",
        description: "Token savings analytics",
    },
    CommandInfo {
        name: "config",
        category: "Meta",
        savings: "\u{2014}",
        description: "Show/create config file",
    },
    CommandInfo {
        name: "verify",
        category: "Meta",
        savings: "\u{2014}",
        description: "Verify hook integrity",
    },
    CommandInfo {
        name: "discover",
        category: "Meta",
        savings: "\u{2014}",
        description: "Find missed savings",
    },
    CommandInfo {
        name: "learn",
        category: "Meta",
        savings: "\u{2014}",
        description: "Learn CLI corrections",
    },
    CommandInfo {
        name: "commands",
        category: "Meta",
        savings: "\u{2014}",
        description: "This command",
    },
    CommandInfo {
        name: "rewrite",
        category: "Meta",
        savings: "\u{2014}",
        description: "Rewrite raw \u{2192} rtk (for hooks)",
    },
    CommandInfo {
        name: "hook-audit",
        category: "Meta",
        savings: "\u{2014}",
        description: "Hook rewrite audit metrics",
    },
    CommandInfo {
        name: "cc-economics",
        category: "Meta",
        savings: "\u{2014}",
        description: "Claude Code economics",
    },
];

pub fn run(short: bool) -> Result<()> {
    if short {
        let names: Vec<&str> = COMMANDS
            .iter()
            .filter(|c| c.category != "Meta" && c.name != "proxy")
            .map(|c| c.name)
            .collect();
        println!("{}", names.join(","));
        return Ok(());
    }

    println!(
        "rtk commands \u{2014} {}+ proxied commands\n",
        COMMANDS.len()
    );
    println!("  {:<16} {:<10} DESCRIPTION", "COMMAND", "SAVINGS");

    let mut current_category = "";
    for cmd in COMMANDS {
        if cmd.category != current_category {
            current_category = cmd.category;
            println!(
                "  \u{2500}\u{2500}\u{2500} {} \u{2500}\u{2500}\u{2500}",
                current_category
            );
        }
        println!("  {:<16} {:<10} {}", cmd.name, cmd.savings, cmd.description);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_succeeds() {
        assert!(run(false).is_ok());
    }

    #[test]
    fn test_run_short_succeeds() {
        assert!(run(true).is_ok());
    }

    #[test]
    fn test_short_excludes_meta_commands() {
        let names: Vec<&str> = COMMANDS
            .iter()
            .filter(|c| c.category != "Meta" && c.name != "proxy")
            .map(|c| c.name)
            .collect();
        // Meta commands should not appear
        assert!(!names.contains(&"init"));
        assert!(!names.contains(&"gain"));
        assert!(!names.contains(&"config"));
        assert!(!names.contains(&"commands"));
        // proxy doesn't filter, exclude it too
        assert!(!names.contains(&"proxy"));
        // Proxied commands should appear
        assert!(names.contains(&"git"));
        assert!(names.contains(&"cargo"));
    }

    #[test]
    fn test_commands_list_not_empty() {
        assert!(!COMMANDS.is_empty());
    }

    #[test]
    fn test_all_fields_non_empty() {
        for cmd in COMMANDS {
            assert!(!cmd.name.is_empty(), "command name must not be empty");
            assert!(
                !cmd.category.is_empty(),
                "category must not be empty for {}",
                cmd.name
            );
            assert!(
                !cmd.savings.is_empty(),
                "savings must not be empty for {}",
                cmd.name
            );
            assert!(
                !cmd.description.is_empty(),
                "description must not be empty for {}",
                cmd.name
            );
        }
    }

    #[test]
    fn test_categories_are_grouped() {
        let mut seen_categories = Vec::new();
        let mut current = "";
        for cmd in COMMANDS {
            if cmd.category != current {
                assert!(
                    !seen_categories.contains(&cmd.category),
                    "category '{}' appears in multiple non-contiguous groups",
                    cmd.category
                );
                seen_categories.push(current);
                current = cmd.category;
            }
        }
    }
}
