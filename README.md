# Un snake

Pour l'atelier  Atelier developpement de jeu en Rust aux jdll le 3 avril 2022 à 14h.

On va utiliser [macroquad](https://macroquad.rs/) et s'appuyer notamment sur [cet exemple](https://github.com/not-fl3/macroquad/blob/master/examples/snake.rs)

## Setup

[cargo](https://doc.rust-lang.org/cargo/) est LE gestionnaire de paquet en rust. Il permet de gérer les dépendances de façon simple et fluide. 

`cargo run` pour exécuter votre premier Hello world sans avoir écrit une ligne de code. (Cette commande a générer un repertoire `target` qui contient les binaires)

Nous nous intéresserons à deux fichiers
- `snake/Cargo.toml` qui contient les informations sur le package et notamment les dépendances
- `snake/src/main.rs` qui contient le code source

 `macroquad = "0.3"` est dans les  dépendances du `Cargo.toml`. `cargo build` dans le terminal pour voir que cette dépendance est bien téléchargée.

> Bonus: `cargo doc` permet de générer la documentation d'un projet y compris pour ses dépendances. Voir `snake/target/doc/`



## Idée générale
Ce jeu a été repris d'un exemple fourni par macroquad.

Deux possibilités: 
* réimplémenter le snake à votre façon de zéro
* améliorer la qualité du code
* ajouter des fonctionnalités

### Fonctionnement de base
* une grille de n * n carrés
* un carré pour la tête du snake
* un fruit qui apparait
* si le snake mange le fruit, le fruit disparait, le snake grandi, il accélère et un nouveau fruit apparait.
* déplacement à intervalle régulier
* game over si il se mange lui même ou si il se cogne dans un mur


### Fonctionnalités supplémentaire
* monde circulaire: si snake arrive au bord, il réapparait de l'autre côté
* portal
* tracer un arc en ciel

