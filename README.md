# Wallet Inspector

Un outil en ligne de commande (CLI) écrit en Rust pour inspecter l'activité des portefeuilles Ethereum et StarkNet. Le projet va inclure également un contrat Cairo qui servira de carnet d'adresses on-chain.

## Stack technique

- Rust : le langage de programmation pour le CLI ;
- Cargo : l'outil de build et gestionnaire de dépendances de Rust ;
- Cairo : le langage de programmation pour écrire les smart contracts sur StarkNet ;
- Scarb : le gestionnaire de dépendances et compilateur de Cairo ;

## Préparation du projet

### Installation

Vérifier que les composants suivants ne sont pas déjà installés :

```bash
$ rustc --version
$ cargo --version
$ scarb --version
```

Installation de rust sous WSL :

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>1
```

Installation du linker cc :

```bash
$ sudo apt update && sudo apt install -y build-essential
```

Recharger le PATH :

```bash
$ source "$HOME/.cargo/env"
```

Vérifications :

```bash
$ rustc --version
rustc 1.95.0 (59807616e 2026-04-14)
$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
```

Installation de Scarb :

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh

$ source ~/.bashrc

$ scarb --version
scarb 2.18.0 (e6144df0f 2026-04-21)
cairo: 2.18.0 (https://crates.io/crates/cairo-lang-compiler/2.18.0)
sierra: 1.8.0
arch: x86_64-unknown-linux-gnu
```

### Initialisation

Init du projet Rust et vérification de la compilation :

```bash
$ cargo init
    Creating binary (application) package
$ cargo run
   Compiling wallet-inspector v0.1.0 (/mnt/d/Developpement/github/wallet-inspector)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/wallet-inspector`
Hello, world!
```

Initialisation de la partie Cairo et vérification :

```bash
$ scarb new cairo
✔ Which test runner do you want to set up? · None (default)
Created `cairo` package.

$ cd cairo && scarb build
   Compiling cairo v0.1.0 (/mnt/d/Developpement/github/wallet-inspector/cairo/Scarb.toml)
    Finished `dev` profile target(s) in 1 second
```

Ajout des dépendances dans Cargo.toml puis build :

```bash
$ cargo build
```

/!\ J'ai dû ajouter deux dépendances pour que le build fonctionne :

```bash
$ sudo apt install -y pkg-config libssl-dev
```

### Développement

- Fichier main.rs : point d'entrée qui appelle la méthode run() du module cli.  
- Fichier cli.rs : récupère les arguments, vérifie que l'adresse passée en paramètre est bien au format Ethereum.  
- Fichier rpc.rs : appel réseau pour récupérer le solde ETH d'une adresse.  
- Fichier .env : fichier à créer à la racine du répertoire du projet, et doit contenir : ALCHEMY_API_KEY=<votre_api_key_alchemy>

/!\ Besoin de créer une clé API Alchemy (gratuite) sur https://alchemy.com.

Tests sur l'adresse Ethereum :

```bash
$ cargo run -- 0x1234abcd
   Compiling wallet-inspector v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.79s
     Running `target/debug/wallet-inspector 0x1234abcd`
Error: '0x1234abcd' n est pas une adresse Ethereum valide

$ cargo run -- saucisse
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/wallet-inspector saucisse`
Error: 'saucisse' n est pas une adresse Ethereum valide
```

Test de récupération de la balance d'une adresse Ethereum :

```bash
$ cargo run -- 0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045
   Compiling wallet-inspector v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.41s
     Running `target/debug/wallet-inspector 0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045`
Balance: 229.5975 ETH
```
