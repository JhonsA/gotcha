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
    dependencies: Option<HashMap<String, String>>,
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
    // analise_package_lock_json(&package_lock_json.packages);

    // println!("{:?}", package_json);
    // println!("{:?}", package_lock_json);
}

fn analise_package_json(scripts: &Option<HashMap<String, String>>) {
    let dangerous_scripts = [
        "postinstall",
        "preinstall",
        "prepare",
        "install",
        "prepublish",
        "prepublishOnly",
        "prepack",
        "postpack",
        "publish",
        "postpublish",
    ];

    let dangerours_content = [
        "curl",
        "wget",
        "bash -c",
        "sh -c",
        "powershell",
        "invoke-webrequest",
        "mshta",
        "nc",
        "node -e",
        "python -c",
    ];

    if let Some(scripts) = scripts {
        for (script_name, command_text) in scripts {
            let lowercase_command = command_text.to_lowercase();
            let normalized_command = lowercase_command.trim();
            for suspicious_pattern in dangerours_content {
                if normalized_command.contains(suspicious_pattern) {
                    println!(
                        "⚠ WARNING! {} script detected with suspicious command: {}",
                        suspicious_pattern, script_name
                    );
                }
            }

            for suspicious_pattern in dangerous_scripts {
                if script_name == suspicious_pattern {
                    println!(
                        "⚠ WARNING! {} script detected: {}",
                        suspicious_pattern, script_name
                    );
                }
            }
        }
    }
}

// fn analise_package_lock_json(packages: &Option<HashMap<String, LockPackage>>) {
//     if let Some(packages) = packages {
//         for (path, pkg) in packages {
//             println!(
//                 "{} -> {:?} {:?} {:?} {:?}",
//                 path, pkg.version, pkg.resolved, pkg.integrity, pkg.dependencies
//             );
//         }
//     }
// }
