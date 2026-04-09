use clap::{Parser, Subcommand};
use std::path::Path;
use std::{fs, io};

const SEED_AGENTS: &str = include_str!("../seed/AGENTS.md");
const SEED_ARCHITECTURE: &str = include_str!("../seed/docs/ARCHITECTURE.md");
const SEED_DECISIONS_README: &str = include_str!("../seed/docs/decisions/README.md");

const CURRENT_SEED_GENERATION: u32 = 2;

struct SeedUpdate {
    generation: u32,
    description: &'static str,
}

const SEED_UPDATES: &[SeedUpdate] = &[SeedUpdate {
    generation: 2,
    description: "Add `anima check` awareness to Cultivation section",
}];

#[derive(Parser)]
#[command(name = "anima", about = "Plant seeds, not templates.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Plant a seed into the current project
    Init {
        /// Project name (defaults to current directory name)
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Observe the project's cultivation state
    Check,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { name } => {
            if let Err(e) = run_init(name) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        Command::Check => {
            if let Err(e) = run_check() {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    }
}

fn run_init(name: Option<String>) -> io::Result<()> {
    let project_name = match name {
        Some(n) => n,
        None => infer_project_name()?,
    };

    if Path::new("AGENTS.md").exists() {
        eprintln!("warning: AGENTS.md already exists, skipping");
        return Ok(());
    }

    let agents_content = SEED_AGENTS.replace("{project-name}", &project_name);

    fs::write("AGENTS.md", agents_content)?;
    fs::create_dir_all("docs/decisions")?;
    fs::write("docs/ARCHITECTURE.md", SEED_ARCHITECTURE)?;
    fs::write("docs/decisions/README.md", SEED_DECISIONS_README)?;

    println!("Seed planted for '{project_name}'.");
    println!();
    println!("  AGENTS.md              — start here");
    println!("  docs/ARCHITECTURE.md   — fills as architecture emerges");
    println!("  docs/decisions/        — record decisions as you go");

    Ok(())
}

fn infer_project_name() -> io::Result<String> {
    let cwd = std::env::current_dir()?;
    let name = cwd
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project");
    Ok(name.to_string())
}

// --- check command ---

fn run_check() -> io::Result<()> {
    println!("anima check");
    println!();

    if !Path::new("AGENTS.md").exists() {
        println!("  No seed found. Run `anima init` to plant one.");
        return Ok(());
    }

    let agents = fs::read_to_string("AGENTS.md")?;

    let phase = extract_phase(&agents);
    let phase_initial = phase.as_deref() == Some("Germination");
    let phase_display = match &phase {
        Some(p) if phase_initial => format!("{p} (initial)"),
        Some(p) => p.clone(),
        None => "unknown".to_string(),
    };

    let arch = observe_architecture()?;
    let decisions = count_decisions()?;
    let conventions = count_conventions(&agents);

    let planted_gen = detect_seed_generation(&agents);

    println!("  state:         {phase_display}");
    println!("  architecture:  {arch}");
    println!(
        "  decisions:     {}",
        if decisions == 0 {
            "none recorded".to_string()
        } else {
            format!("{decisions} recorded")
        }
    );
    println!(
        "  conventions:   {}",
        if conventions == 0 {
            "none yet".to_string()
        } else {
            format!("{conventions} established")
        }
    );
    if planted_gen < CURRENT_SEED_GENERATION {
        println!(
            "  seed:          v{planted_gen} \u{2192} v{CURRENT_SEED_GENERATION} available"
        );
    }
    println!();

    let mut dormant: Vec<&str> = Vec::new();
    if phase_initial {
        dormant.push("state");
    }
    if arch.contains("empty") || arch.contains("not found") {
        dormant.push("architecture");
    }
    if decisions == 0 {
        dormant.push("decisions");
    }
    if conventions == 0 {
        dormant.push("conventions");
    }

    if dormant.is_empty() {
        println!("  The harness is growing well.");
    } else if dormant.len() == 4 {
        println!("  The seed is planted but dormant. Growth begins with practice.");
    } else {
        println!("  Dormant areas: {}", dormant.join(", "));
    }

    if planted_gen < CURRENT_SEED_GENERATION {
        println!();
        println!("  Seed updates available:");
        for update in SEED_UPDATES {
            if update.generation > planted_gen {
                println!("    v{}: {}", update.generation, update.description);
            }
        }
    }

    Ok(())
}

fn extract_phase(content: &str) -> Option<String> {
    for line in content.lines() {
        let Some(start) = line.find("**Phase:") else {
            continue;
        };
        let after = line[start + 8..].trim_start();
        if let Some(end) = after.find(".**") {
            return Some(after[..end].to_string());
        }
        if let Some(end) = after.find("**") {
            return Some(after[..end].trim_end_matches('.').to_string());
        }
    }
    None
}

fn observe_architecture() -> io::Result<String> {
    let path = Path::new("docs/ARCHITECTURE.md");
    if !path.exists() {
        return Ok("not found".to_string());
    }
    let content = fs::read_to_string(path)?;
    if content.contains("This document is empty") {
        Ok("empty (initial)".to_string())
    } else {
        Ok("documented".to_string())
    }
}

fn count_decisions() -> io::Result<usize> {
    let dir = Path::new("docs/decisions");
    if !dir.exists() {
        return Ok(0);
    }
    let mut count = 0;
    for entry in fs::read_dir(dir)? {
        let name = entry?.file_name();
        let name = name.to_string_lossy();
        if name.ends_with(".md") && name.as_ref() != "README.md" {
            count += 1;
        }
    }
    Ok(count)
}

fn detect_seed_generation(content: &str) -> u32 {
    for line in content.lines().rev() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("<!-- anima:seed:") {
            if let Some(num_str) = rest.strip_suffix(" -->") {
                if let Ok(n) = num_str.parse::<u32>() {
                    return n;
                }
            }
        }
    }
    1
}

fn count_conventions(content: &str) -> usize {
    let mut in_section = false;
    let mut count = 0;
    for line in content.lines() {
        if line.starts_with("## Conventions") {
            in_section = true;
            continue;
        }
        if in_section && line.starts_with("## ") {
            break;
        }
        if in_section {
            let t = line.trim();
            if t.is_empty() || t.contains("None yet") {
                continue;
            }
            if t.starts_with("- ") || t.starts_with("* ") {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_phase_germination() {
        let content = "## State\n\n**Phase: Germination.**\n\nSome text.";
        assert_eq!(extract_phase(content), Some("Germination".to_string()));
    }

    #[test]
    fn extract_phase_custom() {
        let content = "## State\n\n**Phase: Rooting.** Two commands live.";
        assert_eq!(extract_phase(content), Some("Rooting".to_string()));
    }

    #[test]
    fn extract_phase_without_trailing_dot() {
        let content = "**Phase: Growing**";
        assert_eq!(extract_phase(content), Some("Growing".to_string()));
    }

    #[test]
    fn extract_phase_missing() {
        let content = "## State\n\nNo phase line here.";
        assert_eq!(extract_phase(content), None);
    }

    #[test]
    fn detect_generation_v2_marker() {
        let content = "Some content\n\n<!-- anima:seed:2 -->";
        assert_eq!(detect_seed_generation(content), 2);
    }

    #[test]
    fn detect_generation_v3_marker() {
        let content = "Content\n<!-- anima:seed:3 -->\n";
        assert_eq!(detect_seed_generation(content), 3);
    }

    #[test]
    fn detect_generation_no_marker() {
        let content = "## Cultivation\n\nJust directives, no marker.";
        assert_eq!(detect_seed_generation(content), 1);
    }

    #[test]
    fn detect_generation_malformed_marker() {
        let content = "<!-- anima:seed:abc -->";
        assert_eq!(detect_seed_generation(content), 1);
    }

    #[test]
    fn count_conventions_none() {
        let content = "## Conventions\n\n_None yet. Add conventions here._\n\n## Cultivation";
        assert_eq!(count_conventions(content), 0);
    }

    #[test]
    fn count_conventions_some() {
        let content = "\
## Conventions

- English is the primary language
- All knowledge lives in the repo
* Star-prefixed convention

## Cultivation";
        assert_eq!(count_conventions(content), 3);
    }

    #[test]
    fn count_conventions_at_end_of_file() {
        let content = "## Conventions\n\n- Only one convention";
        assert_eq!(count_conventions(content), 1);
    }

    #[test]
    fn seed_embeds_are_nonempty() {
        assert!(!SEED_AGENTS.is_empty());
        assert!(!SEED_ARCHITECTURE.is_empty());
        assert!(!SEED_DECISIONS_README.is_empty());
    }

    #[test]
    fn seed_agents_has_placeholder() {
        assert!(SEED_AGENTS.contains("{project-name}"));
    }

    #[test]
    fn seed_agents_has_generation_marker() {
        assert!(SEED_AGENTS.contains("<!-- anima:seed:"));
    }

    #[test]
    fn seed_generation_matches_current() {
        let detected = detect_seed_generation(SEED_AGENTS);
        assert_eq!(detected, CURRENT_SEED_GENERATION);
    }

    #[test]
    fn seed_updates_cover_all_generations() {
        for g in 2..=CURRENT_SEED_GENERATION {
            assert!(
                SEED_UPDATES.iter().any(|u| u.generation == g),
                "missing seed update entry for generation {g}"
            );
        }
    }
}
