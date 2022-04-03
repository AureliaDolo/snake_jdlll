# Un snake

On va utiliser [macroquad](https://macroquad.rs/)

## Setup

[cargo](https://doc.rust-lang.org/cargo/) est LE gestionnaire de paquet en rust. Il permet de gérer les dépendances de façon simple et fluide. 

`cargo new snake` dans un terminal pour générer un nouveau package rust.
`cargo run` pour exécuter votre premier Hello world sans avoir écrit une ligne de code. (Cette commande a générer un repertoire `target` qui contient les binaires)

Nous éditerons deux fichiers
- `snake/Cargo.toml` qui contient les inforamtions sur le package et notamment les dépendances
- `snake/src/main.rs` qui contient le code source

Ajouter `macroquad = "0.3"` aux dépendances dans le `Cargo.toml`. `cargo build` dans le terminal pour voir que cette dépendance est bien téléchargée.

> Bonus: `cargo doc` permet de générer la documentation d'un projet y compris pour ses dépendances. Voir `snake/target/doc/macroquad/index.html`


```rust
use macroquad::prelude::*; // importe les trucs utiles

#[macroquad::main("Snake")]
async fn main() {
    // votre code
    loop {}
}
```
Une fenêtre apparait , le debut du succès !! 🐍

> pour la fermer, ctrl+C dans le terminal

## Idée générale
* une grille de n * n carrés



Pour une valeur constante accessible depuis tout le code
```rust
const MA_VALEUR: u8 = 42;
```