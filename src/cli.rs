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

    if !is_valid_eth_address(address) {
        eprintln!("Error: '{}' n'est pas une adresse Ethereum valide", address);
        std::process::exit(1);
    }

    println!("Adresse: {}", address);
}

/**
 * Vérifie si une chaîne de caractères est une adresse Ethereum valide.
 * Une adresse Ethereum valide commence par "0x", a une longueur de 42 caractères, et les 40 caractères suivants sont des chiffres hexadécimaux (0-9, a-f, A-F).
 */
fn is_valid_eth_address(address: &str) -> bool {
    address.starts_with("0x")
        && address.len() == 42
        && address[2..].chars().all(|c| c.is_ascii_hexdigit())
}
