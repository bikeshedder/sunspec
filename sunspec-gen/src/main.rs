use std::{fs, path::PathBuf};

use clap::Parser;
use proc_macro2::TokenStream;
use sunspec_gen::gen::{gen_model_struct, gen_models_struct};

#[derive(Parser)]
struct Args {
    smdx_dir: String,
    target_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut models = sunspec_gen::json::read_dir(&args.smdx_dir)?;
    models.sort_by_key(|g| g.id);

    fs::write(
        args.target_dir.join("mod.rs"),
        ts_to_str(gen_models_struct(&models)?).as_bytes(),
    )?;
    for model in models {
        let filename = format!("model{}.rs", model.id);
        fs::write(
            args.target_dir.join(filename),
            ts_to_str(gen_model_struct(&model)?).as_bytes(),
        )?;
    }
    Ok(())
}

fn ts_to_str(stream: TokenStream) -> String {
    let file = syn::parse_file(&stream.to_string()).unwrap();
    let code = prettyplease::unparse(&file);
    code.to_string()
}
