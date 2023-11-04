use std::{fs, path::PathBuf};

use clap::Parser;
use sunspec_gen::gen::{gen_model_struct, gen_models_struct};

#[derive(Parser)]
struct Args {
    smdx_dir: String,
    target_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let docs = sunspec_gen::smdx::read_dir(&args.smdx_dir);
    let mut model_groups = docs?
        .into_iter()
        .flat_map(|doc| doc.into_model_groups())
        .collect::<Vec<_>>();
    model_groups.sort_by_key(|g| g.model.id);

    fs::write(
        args.target_dir.join("mod.rs"),
        gen_models_struct(&model_groups)?.as_bytes(),
    )?;
    for model_group in model_groups {
        let filename = format!("model{}.rs", model_group.model.id);
        fs::write(
            args.target_dir.join(filename),
            gen_model_struct(&model_group)?.as_bytes(),
        )?;
    }
    Ok(())
}
