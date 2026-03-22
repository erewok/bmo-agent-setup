use anyhow::{Context, Result};
use bmo_agent_setup::claude_code::ClaudeCode;
use bmo_agent_setup::config::ConfigFile;
use clap::Parser;
use std::fs;
use std::path::Path;
use tracing::{debug, info};

/// Setup Claude Code environment with agents, skills, and configuration
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output directory for Claude Code setup
    #[arg(short, long, default_value = "./claude-code-env")]
    output: String,

    /// Path to TOML configuration file
    #[arg(short, long)]
    config: Option<String>,

    /// Enable statusline in claude.settings.json (overrides config file)
    #[arg(long)]
    with_statusline: Option<bool>,

    /// Enable always-thinking mode (overrides config file)
    #[arg(long)]
    with_thinking: Option<bool>,
}

fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let output_dir = &args.output;

    info!("Setting up Claude Code environment in: {}", output_dir);

    // Create output directory structure
    let output_path = Path::new(output_dir);
    fs::create_dir_all(output_path)?;
    let agents_dir = output_path.join("agents");
    let skills_dir = output_path.join("skills");
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
    generate_settings(output_path, &args)?;

    // Copy statusline script (if enabled via CLI or config)
    let copy_statusline_script = args.with_statusline.unwrap_or(false)
        || (args.config.is_some() && has_statusline_in_config(&args)?);

    if copy_statusline_script {
        info!("📊 Copying statusline script...");
        copy_statusline(output_path)?;
    }

    // Print installation instructions
    print_instructions(output_path, &args, copy_statusline_script)?;

    Ok(())
}

fn has_statusline_in_config(args: &Args) -> Result<bool> {
    if let Some(ref config_path) = args.config {
        let config = ConfigFile::from_file(config_path)?;
        Ok(config.statusline.map(|s| s.enabled).unwrap_or(false))
    } else {
        Ok(false)
    }
}

fn copy_agents(agents_dir: &Path) -> Result<()> {
    let source_agents = Path::new("agents");

    for entry in fs::read_dir(source_agents)? {
        let entry = entry?;
        let filename = entry.file_name();
        if entry.path().extension().is_some_and(|ext| ext == "md") {
            fs::copy(entry.path(), agents_dir.join(&filename))
                .with_context(|| format!("Failed to copy {}", filename.to_string_lossy()))?;
            debug!("  ✓ {}", filename.to_string_lossy());
        }
    }

    Ok(())
}

fn copy_skills(skills_dir: &Path) -> Result<()> {
    let source_skills = Path::new("skills");

    // Copy dev-init
    if source_skills.join("dev-init").exists() {
        copy_dir_recursive(
            &source_skills.join("dev-init"),
            &skills_dir.join("dev-init"),
        )?;
        debug!("  ✓ dev-init/");
    }

    // Copy dev-team
    if source_skills.join("dev-team").exists() {
        copy_dir_recursive(
            &source_skills.join("dev-team"),
            &skills_dir.join("dev-team"),
        )?;
        debug!("  ✓ dev-team/");
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
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

fn generate_settings(output_dir: &Path, args: &Args) -> Result<()> {
    let mut builder = ClaudeCode::new();

    // Load config file if provided
    if let Some(ref config_path) = args.config {
        info!("📄 Loading configuration from: {}", config_path);
        let config = ConfigFile::from_file(config_path)
            .with_context(|| format!("Failed to load config file: {}", config_path))?;
        builder = config.apply_to_builder(builder);
    }

    // CLI overrides
    // If no config file is provided, apply defaults
    if args.config.is_none() {
        // Default: enable thinking
        builder = builder.with_always_thinking_enabled(args.with_thinking.unwrap_or(true));
    } else {
        // With config file: only apply CLI if explicitly set
        if let Some(thinking) = args.with_thinking {
            builder = builder.with_always_thinking_enabled(thinking);
        }
    }

    // Statusline CLI override or default behavior
    let statusline_enabled = if let Some(enabled) = args.with_statusline {
        enabled
    } else if args.config.is_none() {
        false // Default: disabled
    } else {
        // Config file handled it, don't override
        return build_and_write_settings(builder, output_dir);
    };

    if statusline_enabled {
        let statusline_path = format!("{}/.claude/statusline.sh", std::env::var("HOME")?);
        builder = builder.with_status_line(&statusline_path);
    }

    build_and_write_settings(builder, output_dir)
}

fn build_and_write_settings(builder: ClaudeCode, output_dir: &Path) -> Result<()> {
    let settings = builder.build()?;
    let settings_json = serde_json::to_string_pretty(&settings)?;
    fs::write(output_dir.join("settings.json"), settings_json)?;
    debug!("  ✓ settings.json");
    Ok(())
}

fn copy_statusline(output_dir: &Path) -> Result<()> {
    let source_statusline = Path::new("src/statusline.sh");
    let dest_statusline = output_dir.join("statusline.sh");

    fs::copy(source_statusline, &dest_statusline).context("Failed to copy statusline.sh")?;

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

fn print_instructions(output_dir: &Path, _args: &Args, has_statusline: bool) -> Result<()> {
    let abs_path = output_dir
        .canonicalize()
        .unwrap_or(output_dir.to_path_buf());

    println!("\n✅ Claude Code environment created successfully!\n");
    println!("⚠️  WARNING: The install steps below will overwrite files in ~/.claude/");
    println!("    Back up any existing configuration before proceeding:");
    println!("    cp ~/.claude/settings.json ~/.claude/settings.json.bak\n");
    println!("To install on your machine:\n");
    println!("  1. Copy agent definitions:");
    println!("     cp -r {}/agents ~/.claude/", abs_path.display());
    println!();
    println!("  2. Copy skills:");
    println!("     cp -r {}/skills ~/.claude/", abs_path.display());
    println!();
    println!("  3. Copy settings:");
    println!("     cp {}/settings.json ~/.claude/", abs_path.display());
    println!();

    if has_statusline {
        println!("  4. Copy statusline script:");
        println!("     cp {}/statusline.sh ~/.claude/", abs_path.display());
        println!();
        println!("Or run all at once:");
        println!(
            "  cp -r {}/{{agents,skills}} ~/.claude/ && \\",
            abs_path.display()
        );
        println!(
            "  cp {}/{{settings.json,statusline.sh}} ~/.claude/",
            abs_path.display()
        );
    } else {
        println!("Or run all at once:");
        println!(
            "  cp -r {}/{{agents,skills}} ~/.claude/ && \\",
            abs_path.display()
        );
        println!("  cp {}/settings.json ~/.claude/", abs_path.display());
    }
    println!();

    Ok(())
}
