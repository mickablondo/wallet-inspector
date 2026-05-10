use std::env;

pub fn run() {
    // récupération des arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();

    // vérification qu'il ya au-moins deux arguments : chemin du binaire et adresse à inspecter
    if args.len() < 2 {
        eprintln!("Usage: wallet-inspector <address>");
        std::process::exit(1);
    }

    // récupération de l'adresse à inspecter
    let address = &args[1];
    println!("Inspecting address: {}", address);
}