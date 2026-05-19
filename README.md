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
$ curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh -s -- -v 2.8.5

$ source ~/.bashrc

$ scarb --version
scarb 2.8.5 (3967bd4a6 2024-11-18)
cairo: 2.8.5 (https://crates.io/crates/cairo-lang-compiler/2.8.5)
sierra: 1.6.0
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
  - ALCHEMY_API_KEY=<votre_api_key_alchemy_ethereum>
  - ETHERSCAN_API_KEY=<votre_api_key_etherscan>
  - ALCHEMY_STARKNET_API_KEY=<votre_api_key_alchemy_starknet>
  - STARKNET_RPC_URL=<URL_RPC_BLOCKHAIN_STARKNET>

> ⚠️ Créer une clé API Alchemy (gratuite) sur https://alchemy.com — réseau **Ethereum Mainnet** — pour récupérer la balance et les transactions.  
> ⚠️ Créer une clé API Alchemy (gratuite) sur https://alchemy.com — réseau **StarkNet Sepolia** — pour interagir avec le smart contract Cairo.  
> ⚠️ Créer une clé API Etherscan (gratuite) sur https://etherscan.io — pour récupérer l'historique des transactions.

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

Déposer 5 STRK via l'extension Ready X (réseau Sépolia !) sur l'adresse indiquée en sortie de la précédente commande.

### Déploiement sur une blockchain de test

#### Déploiement du compte StarkNet :

```bash
$ starkli account deploy ~/.starkli-wallets/account-descriptor.json --keystore ~/.starkli-wallets/account.json --rpc https://starknet-sepolia.g.alchemy.com/starknet/version/rpc/v0_8/<CLE_API_ALCHEMY_STARKNET>
Enter keystore password:
The estimated account deployment fee is 0.019376205515911808 STRK. However, to avoid failure, fund at least:
    0.043596462410801568 STRK
to the following address:
    0x.....................................
Press [ENTER] once you've funded the address.
Account deployment transaction: 0x......................
Waiting for transaction 0x.......................... to confirm. If this process is interrupted, you will need to run `starkli account fetch` to update the account file.
Transaction not confirmed yet...
Transaction not confirmed yet...
Transaction 0x............ confirmed
```

#### Déploiement du smart contract Cairo

On compile le smart contract :

```bash
$ cd cairo && scarb build
```

On déclare le smart contract sur StarkNet :

```bash
$ starkli declare target/dev/cairo_WalletReader.contract_class.json --casm-file target/dev/cairo_WalletReader.compiled_contract_class.json --keystore ~/.starkli-wallets/account.json --account ~/.starkli-wallets/account-descriptor.json --rpc https://starknet-sepolia.g.alchemy.com/starknet/version/rpc/v0_8/<CLE_API_ALCHEMY_STARKNET>
```

> ⚠️ **Blocage connu (mai 2026)** : Le déploiement est actuellement impossible avec starkli 0.4.2
> car StarkNet Sepolia tourne en v0.14.2 qui utilise un nouveau hash Blake pour les compiled_class_hash, non encore supporté par starkli.
> En attente d'une mise à jour de l'outillage.
> Suivre : https://github.com/xJonathanLEI/starkli/issues

### Déploiement sur une blockchain locale

Pour éviter d'utiliser la blockchain Sepolia, on va installer la blockchain locale starketnet-devnet et sncast.

Dans le cas où vous aviez installé scarb précédemment, supprimez le :

```bash
$ rm -rf ~/.local/share/scarb-install
$ rm ~/.local/bin/scarb
```

Puis installer la blockchain locale et tous les outils compatibles (Scarb, Starknet Foundry avec sncast, starknet-dev ...), tapez les commandes suivantes :

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.starkup.sh | sh
$ source ~/.bashrc
$ scarb --version
$ sncast --version
$ starknet-devnet --version
```

Lancement de la blockchain locale (option seed 0 pour générer des comptes de test au démarrage) :

```bash
$ starknet-devnet --seed 0
```

Sur un autre terminal, testez que le devnet répond bien :

```bash
$ curl -X POST http://127.0.0.1:5050/rpc -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"starknet_chainId","params":[],"id":1}'
{"jsonrpc":"2.0","id":1,"result":"0x......"
```

Créez un compte sncast à partir des comptes pré-déployés de devnet :

```bash
$ sncast account import --url http://127.0.0.1:5050/rpc --name devnet-account --address <ACCOUNT_ADDRESS_TEST> --private-key <PRIVATE_KEY_TEST> --type oz
✔ Do you want to make this account default? · Yes, global default (~/.config/starknet-foundry/snfoundry.toml)
Success: Account imported successfully

Account Name: devnet-account
```

Déclaration du smart contract Cairo :

```bash
$ cd cairo
$ sncast declare --url http://127.0.0.1:5050/rpc --contract-name WalletReader
```

Déploiement du smart contract :

```bash
$ sncast deploy --url http://127.0.0.1:5050/rpc --class-hash <ClassHash_delaprecedentecommande> --arguments '<ACCOUNT_ADDRESS_TEST>'
```

#### Interaction avec le smart contract

Dans le .env :

```bash
STARKNET_RPC_URL=http://127.0.0.1:5050/rpc
```

Lancer la commande suivante avec l'adresse ETH de Vitalik Buterin :

```bash
$ cargo run -- 0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045
   Compiling wallet-inspector v0.1.0 (.../wallet-inspector)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.54s
     Running `.../wallet-inspector/target/debug/wallet-inspector 0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045`
Balance: 5.6767 ETH

Dernières transactions :
  2026-05-16 13:24 | 0x7147eb6ecfef6e07 | 0 ETH | 0xce21a8b9 -> 0xd8da6bf2 | ✓
  2026-05-16 10:33 | 0x4dc69741a5845029 | 0.000005 ETH | 0xdd2326a8 -> 0xd8da6bf2 | ✓
  2026-05-16 10:33 | 0x95ec4f6fdf3f0278 | 0.000008 ETH | 0xdd2326a8 -> 0xd8da6bf2 | ✓
  2026-05-16 03:39 | 0xec85e587663aeb5c | 0 ETH | 0xd8da6bf2 -> 0xf20784fb | ✓
  2026-05-14 16:33 | 0x40a628bd7be1e07b | 0.000005 ETH | 0xdd2326a8 -> 0xd8da6bf2 | ✓

StarkNet contract - compteur : 0
```
