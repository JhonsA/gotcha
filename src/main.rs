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
    scripts: Option<HashMap<String, String>>,
    dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "optionalDependencies")]
    optional_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    peer_dependencies: Option<HashMap<String, String>>,
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

    // let package_lock_json_path = project_path.join("package-lock.json");
    // if !package_lock_json_path.exists() {
    //     eprintln!("Error: package-lock.json not found in the specified project path.");
    //     std::process::exit(1);
    // }

    let package_json_content =
        fs::read_to_string(&package_json_path).expect("Error leyendo package.json");

    let package_json: PackageJson =
        serde_json::from_str(&package_json_content).expect("Error parsing package.json");

    // let package_lock_json_content =
    //     fs::read_to_string(&package_lock_json_path).expect("Error leyendo package-lock.json");

    // let package_lock_json: PackageLockJson =
    //     serde_json::from_str(&package_lock_json_content).expect("Error parsing package-lock.json");

    analise_package_json_scripts(&package_json.scripts);

    analise_package_json_dependencies("dependencies", &package_json.dependencies);
    analise_package_json_dependencies("devDependencies", &package_json.dev_dependencies);
    analise_package_json_dependencies("optionalDependencies", &package_json.optional_dependencies);
    analise_package_json_dependencies("peerDependencies", &package_json.peer_dependencies);

    // println!("{:?}", package_json);
    // println!("{:?}", package_lock_json);
}

fn analise_package_json_scripts(scripts: &Option<HashMap<String, String>>) {
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
                        "⚠ WARNING! | SCRIPT | suspicious command pattern '{}' detected in script '{}': '{}'",
                        suspicious_pattern, script_name, command_text
                    );
                }
            }

            for suspicious_pattern in dangerous_scripts {
                if script_name == suspicious_pattern {
                    println!(
                        "⚠ WARNING! | SCRIPT | lifecycle hook '{}' detected with command '{}'",
                        script_name, command_text
                    );
                }
            }
        }
    }
}

fn analise_package_json_dependencies(
    section_name: &str,
    dependencies: &Option<HashMap<String, String>>,
) {
    let dangerous_dependencies_origins = [
        "git+", "github:", "link:", "http://", "https://", "npm:", "file:",
    ];

    let weak_control_versions = ["*", "latest", "next", "beta", "alpha", "rc"];

    if let Some(dependencies) = dependencies {
        for (dependency_name, version) in dependencies {
            let lowercase_version = version.to_lowercase();
            let normalized_version = lowercase_version.trim();

            for suspicious_pattern in dangerous_dependencies_origins {
                if normalized_version.starts_with(suspicious_pattern) {
                    println!(
                        "⚠ WARNING! | DEPENDENCY | {} '{}' uses non-standard source matching '{}': '{}'",
                        section_name, dependency_name, suspicious_pattern, normalized_version
                    );
                }
            }

            for weak_version in weak_control_versions {
                if normalized_version.contains(weak_version) {
                    println!(
                        "⚠ WARNING! | DEPENDENCY | {} '{}' uses weak version selector '{}' in '{}'",
                        section_name, dependency_name, weak_version, normalized_version
                    );
                }
            }
        }
    }
}
