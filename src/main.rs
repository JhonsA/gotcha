use std::fs;

fn main() {
    let package_json_content = fs::read_to_string("")
        .expect("Algo salio mal leyendo el archivo");

    let package_lock_json_content = fs::read_to_string("n")
        .expect("Algo salio mal leyendo el archivo");

    println!("Contenido del package.json: {}", package_json_content);
    println!("Contenido del package-lock.json: {}", package_lock_json_content);
}

