use std::{fs, path::PathBuf};

use clap::Parser;
use sunspec_gen::{
    format::format_file,
    gen::{gen_model, gen_models_struct},
    manifest::write_model_features,
};

#[derive(Parser)]
struct Args {
    json_dir: String,
    target_dir: PathBuf,
    manifest_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut models = sunspec_gen::json::read_dir(&args.json_dir)?;
    models.sort_by_key(|g| g.id);

    if let Some(manifest_path) = &args.manifest_path {
        let model_ids = models.iter().map(|model| model.id).collect::<Vec<_>>();
        write_model_features(manifest_path, &model_ids)?;
    }

    fs::write(
        args.target_dir.join("mod.rs"),
        format_file(gen_models_struct(&models)?).as_bytes(),
    )?;
    for model in models {
        let filename = format!("model{}.rs", model.id);
        fs::write(
            args.target_dir.join(filename),
            format_file(gen_model(&model)?).as_bytes(),
        )?;
    }
    Ok(())
}
