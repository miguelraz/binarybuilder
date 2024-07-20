use clap::Parser;
use target_triple;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    manifest: clap_cargo::Manifest,
    #[clap(flatten)]
    workspace: clap_cargo::Workspace,
    #[clap(flatten)]
    features: clap_cargo::Features,
}

const TARGET: &'static str = target_triple::TARGET;

fn main() {
    let args = Cli::parse();
    println!("{TARGET}");
    println!("args = {:#?}", args);
    
}