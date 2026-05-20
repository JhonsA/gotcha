use clap::Parser;
use std::{fs, path::Path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Project path
    #[arg(short, long)]
    project_path: String,
}

fn main() {
    let args = Args::parse();

    let project_path = Path::new(&args.project_path);

    let package_json_path = project_path.join("package.json");

    if !package_json_path.exists() {
        eprintln!("Error: package.json not found in the specified project path.");
        std::process::exit(1);
    }

    let package_lock_json_path = project_path.join("package-lock.json");

    if !package_lock_json_path.exists() {
        eprintln!("Error: package-lock.json not found in the specified project path.");
        std::process::exit(1);
    }

    let package_json_content =
        fs::read_to_string(&package_json_path).expect("Error leyendo package.json");

    let package_lock_json_content =
        fs::read_to_string(&package_lock_json_path).expect("Error leyendo package-lock.json");

    println!("{}", package_json_content);
    println!("{}", package_lock_json_content);
}
