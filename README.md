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
   Compiling wallet-inspector v0.1.0 (.../wallet-inspector)
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
   Compiling cairo v0.1.0 (.../wallet-inspector/cairo/Scarb.toml)
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

#### Rust

- Fichier main.rs : point d'entrée qui appelle la méthode run() du module cli.
- Fichier cli.rs : récupère les arguments, vérifie que l'adresse passée en paramètre est bien au format Ethereum.
- Fichier rpc.rs : appel réseau pour récupérer le solde ETH d'une adresse et ses 5 dernières transactions.
- Fichier starknet.rs : permet de contacter le smart contract Cairo.
- Fichier .env : fichier à créer à la racine du répertoire du projet, et doit contenir :
  - ALCHEMY_API_KEY=<votre_api_key_alchemy>
  - ETHERSCAN_API_KEY=<votre_api_key_etherscan>
  - STARKNET_RPC_URL=https://starknet-sepolia.public.blastapi.io/rpc/v0_7

/!\ Besoin de créer une clé API Alchemy (gratuite) sur https://alchemy.com pour récupérer la balance de l'adresse.  
/!\ Besoin de créer une clé API Etherscan (gratuite) sur https://etherscan.io pour récupérer l'historique des transactions.

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
Balance: 229.5997 ETH

Dernières transactions :
  2026-05-11 12:20 | 0xc691d347640b88d3 | 0.00000555 ETH | 0xe02b584b -> 0xd8da6bf2 | ✓
  2026-05-11 06:38 | 0xcdf409f88aa6f157 | 0.00010000 ETH | 0xb6035198 -> 0xd8da6bf2 | ✓
  2026-05-11 06:38 | 0x060840190ce3fba1 | 0.00000000 ETH | 0xb6035198 -> 0xd8da6bf2 | ✓
  2026-05-11 05:55 | 0x2fb588b728ed2a7a | 0.00000500 ETH | 0xe02b584b -> 0xd8da6bf2 | ✓
  2026-05-11 04:46 | 0x50e2e0026c2a09e5 | 0.00000808 ETH | 0xe02b584b -> 0xd8da6bf2 | ✓
```

#### Cairo

Développement du smart contract dans wallet_reader.cairo.

Ce smart contract est un registre d'adresses favorites stocké sur la blockchain StarkNet.

Les fonctions accessibles :

- add_watched_address : ajoute une adresse à la liste ;
- get_watched_count : retourne le nombre d'adresses stockées ;
- get_watched_address : retourne l'adresse à un index donné ;

## Déploiement du contrat Cairo sur StarkNet Sepolia

Pour déployer le contrat Cairo, il faut signer la transaction de déploiement. Cela nécessite de créer trois choses :

**1. Un keystore** (`account.json`)  
Fichier qui contient la clé privée chiffrée stockée localement.

**2. Un fichier de compte** (`account-descriptor.json`)  
Décrit le compte StarkNet. Starkli en a besoin pour savoir comment construire et signer les transactions.

**3. Un compte on-chain**  
Sur StarkNet, les comptes sont eux-mêmes des smart contracts — c'est ce qu'on appelle l'_account abstraction_. Un compte doit donc être déployé sur la blockchain avant de pouvoir envoyer des transactions. ArgentX le fait automatiquement à la création du wallet ; avec starkli il faut le faire manuellement.

### Pré requis

Pour déployer le smart contract Cairo sur une blockchain de test, il faut d'abord installer starkli, l'outil CLI pour interagir avec StarkNet :

```bash
$ curl https://get.starkli.sh | sh
$ source /home/<user>/.starkli/env
$ starkliup
$ starkli --version
```

Installer l'extension [ReadyX](https://www.ready.co/ready-x) et créer un wallet sur le réseau **Sepolia** (testnet StarkNet).  
ReadyX sert uniquement à récupérer des STRK de test via le faucet.

Se rendre sur [Faucet StarkNet](https://faucet.starknet.io/) pour récupérer des STRK de test.

Créer un dossier pour stocker les clés starkli :

```bash
$ mkdir -p ~/.starkli-wallets
```

Création d'un keystore starkli à partir de la seed phrase ArgentX :

```bash
$ starkli signer keystore from-key ~/.starkli-wallets/account.json
```

La clé privée est accessible dans l'extension ReadyX : <b>Paramètres du compte > Exporter la clé privée</b>.

Création du fichier de compte StarkNet associé :

```bash
$ starkli account oz init ~/.starkli-wallets/account-descriptor.json --keystore ~/.starkli-wallets/account.json
Enter keystore password:
Created new account config file: /home/<user>/.starkli-wallets/account-descriptor.json

Once deployed, this account will be available at:
    0x...............................

Deploy this account by running:
    starkli account deploy /home/<user>/.starkli-wallets/account-descriptor.json
```

Déposer 5 STRX via l'extension Ready X (réseau Sépolia !) sur l'adresse indiquée en sortie de la précédente commande.

### Déploiement

--- WORK IN PROGESS ....

```bash
$ starkli account deploy ~/.starkli-wallets/account-descriptor.json --keystore ~/.starkli-wallets/account.json --rpc https://starknet-sepolia.drpc.org
```
