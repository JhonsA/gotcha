use clap::Parser;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Project path
    #[arg(short, long)]
    project_path: String,
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    name: String,
    dependencies: Option<HashMap<String, String>>,
    scripts: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct PackageLockJson {
    name: Option<String>,
    version: Option<String>,
    packages: Option<HashMap<String, LockPackage>>,
}

#[derive(Debug, Deserialize)]
struct LockPackage {
    version: Option<String>,
    resolved: Option<String>,
    integrity: Option<String>,
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

    let package_json: PackageJson =
        serde_json::from_str(&package_json_content).expect("Error parsing package.json");

    let package_lock_json_content =
        fs::read_to_string(&package_lock_json_path).expect("Error leyendo package-lock.json");

    let package_lock_json: PackageLockJson =
        serde_json::from_str(&package_lock_json_content).expect("Error parsing package-lock.json");

    analise_package_json(&package_json.scripts);
    analise_package_lock_json(&package_lock_json.packages);

    // println!("{:?}", package_json);
    // println!("{:?}", package_lock_json);
}

fn analise_package_json(scripts: &Option<HashMap<String, String>>) {
    let dangerous_scripts = ["postinstall", "preinstall", "prepare"];

    if let Some(scripts) = scripts {
        for key in dangerous_scripts {
            if let Some(script) = scripts.get(key) {
                println!("⚠ WARNING! {} script detected: {}", key, script);
            }
        }
    }
}

fn analise_package_lock_json(packages: &Option<HashMap<String, LockPackage>>) {
    if let Some(packages) = packages {
        for (path, pkg) in packages {
            println!("{} -> {:?}", path, pkg.version);
        }
    }
}
