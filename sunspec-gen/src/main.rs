use std::{
    fs,
    io::Write as _,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
};

use clap::{Parser, Subcommand};
use sunspec_gen::{
    format::format_file,
    gen::{gen_model, gen_models_struct},
    manifest::update_model_features,
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

/// Generate the SunSpec models of the `sunspec` crate.
///
/// Run via `cargo models` from anywhere inside the workspace.
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Regenerate `src/models/` and the model features in `Cargo.toml`
    Gen,
    /// Fail if the generated files are not up to date
    Check,
    /// Pull the latest upstream models and regenerate
    Update,
}

fn main() -> Result<ExitCode> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("sunspec-gen lives inside the workspace");
    match Args::parse().command {
        Cmd::Gen => write(root)?,
        Cmd::Check => return check(root),
        Cmd::Update => {
            update_submodule(root)?;
            write(root)?;
        }
    }
    Ok(ExitCode::SUCCESS)
}

struct Output {
    models_dir: PathBuf,
    /// File name and content of every file in `src/models/`
    model_files: Vec<(String, String)>,
    manifest_path: PathBuf,
    manifest: String,
}

fn generate(root: &Path) -> Result<Output> {
    let mut models = sunspec_gen::json::read_dir(root.join("models/json").to_str().unwrap())?;
    models.sort_by_key(|g| g.id);

    let manifest_path = root.join("Cargo.toml");
    let model_ids = models.iter().map(|model| model.id).collect::<Vec<_>>();
    let manifest = update_model_features(&fs::read_to_string(&manifest_path)?, &model_ids)?;

    let mut model_files = vec![(
        "mod.rs".to_owned(),
        rustfmt(&format_file(gen_models_struct(&models)?))?,
    )];
    for model in &models {
        model_files.push((
            format!("model{}.rs", model.id),
            rustfmt(&format_file(gen_model(model)?))?,
        ));
    }

    Ok(Output {
        models_dir: root.join("src/models"),
        model_files,
        manifest_path,
        manifest,
    })
}

fn write(root: &Path) -> Result<()> {
    let output = generate(root)?;
    for path in existing_model_files(&output.models_dir)? {
        fs::remove_file(path)?;
    }
    for (name, content) in &output.model_files {
        fs::write(output.models_dir.join(name), content)?;
    }
    fs::write(&output.manifest_path, &output.manifest)?;
    Ok(())
}

fn check(root: &Path) -> Result<ExitCode> {
    let output = generate(root)?;
    let mut stale = Vec::new();

    let mut existing = existing_model_files(&output.models_dir)?;
    for (name, content) in &output.model_files {
        let path = output.models_dir.join(name);
        existing.retain(|p| p != &path);
        if fs::read_to_string(&path).ok().as_ref() != Some(content) {
            stale.push(path);
        }
    }
    // Files that would be removed by regenerating
    stale.extend(existing);
    if fs::read_to_string(&output.manifest_path)? != output.manifest {
        stale.push(output.manifest_path);
    }

    if stale.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    eprintln!("Generated files are out of date:");
    for path in stale {
        eprintln!("  {}", path.strip_prefix(root).unwrap_or(&path).display());
    }
    eprintln!("Run `cargo models gen` to regenerate them.");
    Ok(ExitCode::FAILURE)
}

fn existing_model_files(models_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(models_dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "rs") {
            paths.push(path);
        }
    }
    Ok(paths)
}

/// prettyplease output is close to but not identical to rustfmt output, so
/// run it through rustfmt to match what `cargo fmt` would produce.
fn rustfmt(source: &str) -> Result<String> {
    let mut child = Command::new(std::env::var_os("RUSTFMT").unwrap_or("rustfmt".into()))
        .args(["--edition", "2021"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    child.stdin.take().unwrap().write_all(source.as_bytes())?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err("rustfmt failed".into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn update_submodule(root: &Path) -> Result<()> {
    git(root, &["submodule", "update", "--init", "models"])?;
    let models = root.join("models");
    git(&models, &["checkout", "."])?;
    git(&models, &["checkout", "master"])?;
    git(&models, &["pull"])?;
    Ok(())
}

fn git(dir: &Path, args: &[&str]) -> Result<()> {
    let status = Command::new("git").args(args).current_dir(dir).status()?;
    if !status.success() {
        return Err(format!("`git {}` failed", args.join(" ")).into());
    }
    Ok(())
}
