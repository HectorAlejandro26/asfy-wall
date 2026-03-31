use rand::seq::SliceRandom;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};

fn main() {
    // 1. Leer argumentos de la terminal
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Falta el directorio. Uso: {} <ruta_a_directorio>", args[0]);
        exit(1);
    }
    let wallpaper_dir = &args[1];

    // 2. Escanear el directorio buscando archivos
    let paths: Vec<PathBuf> = fs::read_dir(wallpaper_dir)
        .unwrap_or_else(|_| {
            eprintln!("No pude leer el directorio: {}", wallpaper_dir);
            exit(1);
        })
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        // Solo archivos
        .filter(|path| path.is_file())
        .collect();

    if paths.is_empty() {
        eprintln!("Directorio vacío o sin archivos válidos. Aburrido.");
        exit(1);
    }

    // 3. Obtener aleatorio
    let mut rng = rand::thread_rng();
    let chosen_wallpaper = paths.choose(&mut rng).unwrap();

    let chosen_transition = "wipe";

    println!(
        "Pintando: {:?} con transición '{}'",
        chosen_wallpaper.file_name().unwrap(),
        chosen_transition
    );

    // 4. Ejecutar awww
    let status = Command::new("awww")
        .arg("img")
        .arg(chosen_wallpaper)
        .arg("--transition-type")
        .arg(chosen_transition)
        // Opcional: ajustar velocidad de transición
        .arg("--transition-step")
        .arg("5")
        .status()
        .expect("Error al invocar awww. ¿Seguro que awww-daemon está corriendo?");

    if !status.success() {
        eprintln!("awww ejecutó, pero devolvió un error. Revisa tus archivos.");
        exit(1);
    }
}
