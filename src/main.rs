use anyhow::{Context, Result};
use clap::Parser;
use bmo_agent_setup::claude_code::ClaudeCode;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info};
use tracing_subscriber;

/// Setup Claude Code environment with agents, skills, and configuration
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output directory for Claude Code setup
    #[arg(short, long, default_value = "./claude-code-env")]
    output: PathBuf,
}

fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let output_dir = &args.output;

    info!("Setting up Claude Code environment in: {}", output_dir.display());

    // Create output directory structure
    fs::create_dir_all(output_dir)?;
    let agents_dir = output_dir.join("agents");
    let skills_dir = output_dir.join("skills");
    fs::create_dir_all(&agents_dir)?;
    fs::create_dir_all(&skills_dir)?;

    // Copy agent files
    info!("📋 Copying agent definitions...");
    copy_agents(&agents_dir)?;

    // Copy skill directories
    info!("🛠️  Copying skills...");
    copy_skills(&skills_dir)?;

    // Generate claude.settings.json
    info!("⚙️  Generating claude.settings.json...");
    generate_settings(output_dir)?;

    // Copy statusline script
    info!("📊 Copying statusline script...");
    copy_statusline(output_dir)?;

    // Print installation instructions
    print_instructions(output_dir)?;

    Ok(())
}

fn copy_agents(agents_dir: &PathBuf) -> Result<()> {
    let source_agents = PathBuf::from("agents");

    for entry in fs::read_dir(&source_agents)? {
        let entry = entry?;
        let filename = entry.file_name();
        if entry.path().extension().map_or(false, |ext| ext == "md") {
            fs::copy(entry.path(), agents_dir.join(&filename))
                .with_context(|| format!("Failed to copy {}", filename.to_string_lossy()))?;
            debug!("  ✓ {}", filename.to_string_lossy());
        }
    }

    Ok(())
}

fn copy_skills(skills_dir: &PathBuf) -> Result<()> {
    let source_skills = PathBuf::from("skills");

    // Copy dev-init
    if source_skills.join("dev-init").exists() {
        copy_dir_recursive(&source_skills.join("dev-init"), &skills_dir.join("dev-init"))?;
        debug!("  ✓ dev-init/");
    }

    // Copy dev-team
    if source_skills.join("dev-team").exists() {
        copy_dir_recursive(&source_skills.join("dev-team"), &skills_dir.join("dev-team"))?;
        debug!("  ✓ dev-team/");
    }

    Ok(())
}

fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

fn generate_settings(output_dir: &PathBuf) -> Result<()> {
    let settings = ClaudeCode::new()
        .with_always_thinking_enabled(true)
        .build()?;

    let settings_json = serde_json::to_string_pretty(&settings)?;
    fs::write(output_dir.join("claude.settings.json"), settings_json)?;
    debug!("  ✓ claude.settings.json");

    Ok(())
}

fn copy_statusline(output_dir: &PathBuf) -> Result<()> {
    let source_statusline = PathBuf::from("src/statusline.sh");
    let dest_statusline = output_dir.join("statusline.sh");

    fs::copy(&source_statusline, &dest_statusline)
        .context("Failed to copy statusline.sh")?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest_statusline)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest_statusline, perms)?;
    }

    debug!("  ✓ statusline.sh");
    Ok(())
}

fn print_instructions(output_dir: &PathBuf) -> Result<()> {
    let abs_path = output_dir.canonicalize().unwrap_or(output_dir.clone());

    info!("\n✅ Claude Code environment created successfully!\n");
    info!("To install on your machine:\n");
    info!("  1. Copy agent definitions:");
    info!("     cp -r {}/agents ~/.claude/", abs_path.display());
    info!("");
    info!("  2. Copy skills:");
    info!("     cp -r {}/skills ~/.claude/", abs_path.display());
    info!("");
    info!("  3. Copy settings:");
    info!("     cp {}/claude.settings.json ~/.claude/", abs_path.display());
    info!("");
    info!("  4. Copy statusline script:");
    info!("     cp {}/statusline.sh ~/.claude/", abs_path.display());
    info!("");
    info!("Or run all at once:");
    info!("  cp -r {}/{{agents,skills}} ~/.claude/ && \\", abs_path.display());
    info!("  cp {}/{{claude.settings.json,statusline.sh}} ~/.claude/", abs_path.display());
    info!("");

    Ok(())
}
