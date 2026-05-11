use std::env;
use colored::Colorize;
use chrono::{DateTime, Utc};

pub async fn run() {
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

    // ----------- ALCHEMY -----------
    // vérification de la bonne récupération de la clé API d'Alchemy depuis les variables d'environnement
    let api_key = env::var("ALCHEMY_API_KEY").unwrap_or_else(|_| {
        eprintln!("Error: ALCHEMY_API_KEY non trouvé dans .env");
        std::process::exit(1);
    });

    // appel de la fonction get_balance pour récupérer le solde de l'adresse et affichage du résultat
    match crate::rpc::get_balance(address, &api_key).await {
        Ok(balance) => println!("Balance: {:.4} ETH", balance),
        Err(e) => eprintln!("Error: {}", e),
    }

    // ----------- ETHERSCAN -----------
    // vérification de la bonne récupération de la clé API d'Etherscan depuis les variables d'environnement
    let etherscan_key = env::var("ETHERSCAN_API_KEY").unwrap_or_else(|_| {
        eprintln!("Error: ETHERSCAN_API_KEY non trouvé dans .env");
        std::process::exit(1);
    });

    // appel de la fonction get_transactions pour récupérer les 5 dernières transactions de l'adresse et affichage du résultat
    match crate::rpc::get_transactions(address, &etherscan_key).await {
        Ok(txs) => {
            println!("\nDernières transactions :");
            for tx in txs {
                let wei: u128 = tx.value.parse().unwrap_or(0);
                let eth = wei as f64 / 1e18;

                let timestamp: i64 = tx.time_stamp.parse().unwrap_or(0);
                let date = DateTime::from_timestamp(timestamp, 0)
                    .unwrap_or_default()
                    .format("%Y-%m-%d %H:%M")
                    .to_string();

                let status = if tx.is_error == "0" {
                    "✓".green()
                } else {
                    "✗".red()
                };

                println!("  {} | {} | {:.8} ETH | {} -> {} | {}",
                    date.dimmed(),
                    &tx.hash[..18].yellow(),
                    eth,
                    &tx.from[..10].cyan(),
                    &tx.to[..10].cyan(),
                    status
                );
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
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
