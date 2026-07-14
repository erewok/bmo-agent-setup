use anyhow::{Context, Result};
use bmo_agent_setup::claude_code::ClaudeCode;
use bmo_agent_setup::config::ConfigFile;
use clap::Parser;
use std::fs;
use std::path::Path;
use tracing::{debug, info};

/// Built-in default configuration, used automatically when no `--config`
/// flag is given. See bmo-config.default.toml for the documented source;
/// bmo-config.yolo-mode.toml and bmo-config.hardened.toml are alternate
/// presets a user can opt into with `--config`.
const DEFAULT_CONFIG_TOML: &str = include_str!("../bmo-config.default.toml");

/// Setup Claude Code environment with agents, skills, and configuration
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output directory for Claude Code setup
    #[arg(short, long, default_value = "./claude-code-env")]
    output: String,

    /// Path to TOML configuration file (defaults to the built-in
    /// bmo-config.default.toml if omitted)
    #[arg(short, long)]
    config: Option<String>,

    /// Enable statusline in settings.json (overrides config file)
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

    let config = load_config(&args)?;

    // Generate settings.json
    info!("⚙️  Generating settings.json...");
    generate_settings(output_path, &args, &config)?;

    // Copy statusline script if the resolved config (CLI override, else
    // config file, else built-in default) enables it
    let statusline_enabled = args
        .with_statusline
        .unwrap_or_else(|| config.statusline.as_ref().is_some_and(|s| s.enabled));

    if statusline_enabled {
        info!("📊 Copying statusline script...");
        copy_statusline(output_path)?;
    }

    // Print installation instructions
    print_instructions(output_path, statusline_enabled)?;

    Ok(())
}

/// Resolve the configuration to use: an explicit `--config` file, or the
/// built-in default embedded at compile time from bmo-config.default.toml.
fn load_config(args: &Args) -> Result<ConfigFile> {
    match &args.config {
        Some(path) => {
            info!("📄 Loading configuration from: {}", path);
            ConfigFile::from_file(path)
                .with_context(|| format!("Failed to load config file: {}", path))
        }
        None => {
            info!("📄 Loading built-in default configuration (bmo-config.default.toml)");
            ConfigFile::from_toml_str(DEFAULT_CONFIG_TOML)
                .context("Failed to parse the built-in default configuration")
        }
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

    // Copy documentation-driver
    if source_skills.join("documentation-driver").exists() {
        copy_dir_recursive(
            &source_skills.join("documentation-driver"),
            &skills_dir.join("documentation-driver"),
        )?;
        debug!("  ✓ documentation-driver/");
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

fn generate_settings(output_dir: &Path, args: &Args, config: &ConfigFile) -> Result<()> {
    let mut builder = ClaudeCode::new();
    builder = config.apply_to_builder(builder);

    // CLI overrides apply on top of whichever config was loaded (explicit
    // file or built-in default)
    if let Some(thinking) = args.with_thinking {
        builder = builder.with_always_thinking_enabled(thinking);
    }

    match args.with_statusline {
        Some(true) => {
            let statusline_path = format!("{}/.claude/statusline.sh", std::env::var("HOME")?);
            builder = builder.with_status_line(&statusline_path);
        }
        Some(false) => {
            builder = builder.without_status_line();
        }
        None => {
            // Leave whatever the config produced
        }
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

fn print_instructions(output_dir: &Path, has_statusline: bool) -> Result<()> {
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
