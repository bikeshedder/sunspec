use clap::Parser;
use sunspec_gen::gen::gen_code;

#[derive(Parser)]
struct Args {
    smdx_dir: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let docs = sunspec_gen::smdx::read_dir(&args.smdx_dir);
    let mut model_groups = docs?
        .into_iter()
        .flat_map(|doc| doc.into_model_groups())
        .collect::<Vec<_>>();
    model_groups.sort_by_key(|g| g.model.id);
    println!("{}", gen_code(&model_groups)?);
    Ok(())
}
