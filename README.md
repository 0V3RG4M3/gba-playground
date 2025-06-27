# GBA Playground

## Overview

Ce dépôt contient plusieurs projets d'exploration pour le développement Game Boy
Advance en Rust. La compilation cible l'architecture `thumbv4t-none-eabi` et
`mgba` est utilisé comme émulateur par défaut (voir `.cargo/config.toml`). Les
exécutables principaux se trouvent dans `src/bin/` et incluent différents tests
ou prototypes (`egj2024`, `egj2025`, `platformer`, etc.). Le code commun est
exposé via la bibliothèque située dans `src/`.

## Préparation

Certaines ressources sont générées à l'aide de scripts Python disponibles dans
le dossier `scripts`. Ces scripts nécessitent un environnement virtuel créé avec
[Poetry](https://python-poetry.org/).

Depuis `scripts/` :

```bash
poetry install --no-root
```

Générez ensuite les données musicales utilisées par la bibliothèque :

```bash
poetry run main_generate_tune_rs.py
```

Des utilitaires supplémentaires, comme `main_convert_sprites.py`, permettent de
convertir les graphismes en palettes et tuiles compatibles GBA.

## Compilation

Le projet fonctionne avec Rust stable. Avant la première compilation, installez les
composants pour la cible GBA :

```bash
rustup component add rust-src
rustup target add thumbv4t-none-eabi
```

Vous pouvez ensuite construire la version finale :

```bash
cargo build --release
```

## Exécution

Installez l'émulateur [mGBA](https://mgba.io/downloads.html). Grâce à la
configuration dans `.cargo/config.toml`, la commande suivante lance
l'exécutable par défaut (`egj2025`) directement dans l'émulateur :

```bash
cargo run
```

## Tests

Des tests unitaires existent pour la partie bibliothèque. Ils s'exécutent sur la
cible `x86_64-unknown-linux-gnu` :

```bash
cargo test --lib --target=x86_64-unknown-linux-gnu
```

## Journalisation

Le module `log4gba` offre des macros simples pour afficher des messages dans la
console de l'émulateur :

```rust
log4gba::debug("Hello world!")
```

Les logs apparaissent dans la fenêtre *Logs* de mGBA (`Tools > View Logs…`).
