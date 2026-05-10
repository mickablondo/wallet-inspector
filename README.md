# Wallet Inspector

A CLI tool written in Rust to inspect Ethereum and StarkNet wallet activity. Includes a minimal Cairo contract as a learning exercise.

## Stach technique

- Rust : le langage de programmation pour le CLI ;
- Cargo : l'outil de build et gestionnaire de dépendances de Rust ;
- Cairo : le langage de programmation pour écrire les smart contracts sur StarkNet ;
- Scarb : le gestionnaire de dépendances et compilateur de Cairo ;

## Préparation du projet

### Installation

Vérifier que les composnants suivants ne sont pas déjà installés :

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

### Développement

Fichier main.rs : point d'entrée qui appelle la méthode run() du module cli.  
Fichier cli.rs :

Compilation et exécution :

```bash
$ cargo run -- 0x1234abcd
   Compiling wallet-inspector v0.1.0 (/mnt/d/Developpement/github/wallet-inspector)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/wallet-inspector 0x1234abcd`
Inspecting address: 0x1234abcd
```
