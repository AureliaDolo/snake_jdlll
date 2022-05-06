// https://github.com/not-fl3/macroquad/blob/master/examples/snake.rs

#![doc = include_str!("../README.md")]
// Le prelude de macro quad contient tout ce qui est utile pour developper en utilisant macroquad
use macroquad::prelude::*;

// On peut aussi importer des éléments individuellement
// ici, un vecdeque <https://doc.rust-lang.org/std/collections/struct.VecDeque.html>
// un vecteur circulaire, on peut accéder facilement au début et à la fin
use std::collections::VecDeque;

/// taille de la grille
/// Note: les commenataire avec 3 / apparaissent dans la doc générée par `cargo doc`
/// (quand ilis sont placés aux bon endroits)
const SQUARES: i16 = 16;

/// Un alias de type bien pratique <https://doc.rust-lang.org/reference/items/type-aliases.htmlhttps://doc.rust-lang.org/std/collections/struct.VecDeque.html>
type Point = (i16, i16);

const UP: Point = (0, -1);
const DOWN: Point = (0, 1);
const RIGHT: Point = (1, 0);
const LEFT: Point = (-1, 0);

/// Le snake. <https://doc.rust-lang.org/book/ch05-01-defining-structs.html>
struct Snake {
    head: Point,
    body: VecDeque<Point>,
    dir: Point,
    rainbow: Vec<(Point, Color)>,
}

fn draw_square((x, y): Point, color: Color) {
    let game_size = screen_width().min(screen_height());
    let offset_x = (screen_width() - game_size) / 2. + 10.;
    let offset_y = (screen_height() - game_size) / 2. + 10.;
    let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;
    draw_rectangle(
        offset_x + x as f32 * sq_size,
        offset_y + y as f32 * sq_size,
        sq_size,
        sq_size,
        color,
    );
}

fn random_point() -> Point {
    (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES))
}

fn random_color() -> Color {
    [
        rand::gen_range::<u8>(0, u8::MAX),
        rand::gen_range::<u8>(0, u8::MAX),
        rand::gen_range::<u8>(0, u8::MAX),
        rand::gen_range::<u8>(0, u8::MAX),
    ]
    .into()
}

#[macroquad::main("Snake")]
async fn main() {
    // initialisation

    // mot clef let pour déclarer une variable
    // mot clef mut pour qu'elle soit mutable
    let mut snake = Snake {
        head: (0, 0),          // on commence en haut a gauche
        dir: RIGHT,            // en direction de la droite
        body: VecDeque::new(), // sans queue
        rainbow: Vec::new(),
    };
    // fruit généré aléatoirement
    let mut fruit: Point = random_point();

    // portals
    let mut orange: Point = random_point();
    let mut blue: Point = random_point();

    // note: pour score et speed les types sont inférés par le compilateur
    // et même si ce sont des nombres, ils ne peuvent pas spontanément interagir ensemble
    let mut score = 0;
    let mut speed = 0.3;
    // si la ligne suivante est décommenté, ça ne compile pas mais ça renvoit une erreur intéressante
    // score + speed;
    // en revanche

    let mut game_over = false;

    // on garde en mémoire la dernière fois qu'on a
    // mis à jour pour mettre à jour la logique du jeu
    // indépendamment du temps qu'une image met à s'afficher
    let mut last_update = get_time();

    // boucle infinie
    // on ne s'arrete jamais de jouer !
    loop {
        // ce if contient la logique du jeu
        if !game_over {
            snake.dir = if is_key_down(KeyCode::Right) && snake.dir != LEFT {
                RIGHT
            } else if is_key_down(KeyCode::Left) && snake.dir != RIGHT {
                LEFT
            } else if is_key_down(KeyCode::Up) && snake.dir != DOWN {
                UP
            } else if is_key_down(KeyCode::Down) && snake.dir != UP {
                DOWN
            } else {
                snake.dir
            };

            // si c'est le moment de mettre à jour
            if get_time() - last_update > speed {
                last_update = get_time();

                // l'ancien emplacement de la tete passe dans la queue
                snake.body.push_front(snake.head);
                // on met a jour la nouvelle tete en fonction de la queue
                snake.head = (
                    (snake.head.0 + snake.dir.0).rem_euclid(SQUARES),
                    (snake.head.1 + snake.dir.1).rem_euclid(SQUARES),
                );

                if snake.head == orange {
                    snake.head = blue;
                    orange = random_point();
                } else if snake.head == blue {
                    snake.head = orange;
                    blue = random_point();
                }
                // si la tete est sur un fruit
                if snake.head == fruit {
                    // un nouveau fruit
                    fruit = random_point();
                    score += 100;
                    speed += 0.01;
                } else {
                    // la ou etait le dernier block de la queue, il n'y a plus rien

                    snake
                        .rainbow
                        .push((snake.body.pop_back().unwrap(), random_color()));
                }

                // vérifier si on s'est mordu la queue
                for body in &snake.body {
                    if snake.head == *body {
                        game_over = true;
                    }
                }
            }
        }

        // ce if contient l'affichage
        if !game_over {
            clear_background(LIGHTGRAY);

            // quelques valeur pratiques pour l'affichage
            // pour pouvoir réajuster l'affichage en fonction de la fenêtre
            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            // la ou on joue
            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

            // le quadrillage
            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }

            for i in 1..SQUARES {
                draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }
            for (body, color) in &snake.rainbow {
                draw_square(*body, *color);
            }
            // dessiner la tête
            draw_square(snake.head, DARKGREEN);

            // dessiner le corps
            for body in &snake.body {
                draw_square(*body, LIME);
            }

            // dessiner le fruit
            draw_square(fruit, GOLD);

            draw_square(orange, ORANGE);

            draw_square(blue, BLUE);

            // écrire le score
            draw_text(
                format!("SCORE: {}", score).as_str(),
                10.,
                10.,
                20.,
                DARKGRAY,
            );
        } else {
            // si game over
            clear_background(WHITE);
            let text = "Game Over. Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            // massage de game over
            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            // reset
            if is_key_down(KeyCode::Enter) {
                snake = Snake {
                    head: (0, 0),
                    dir: RIGHT,
                    body: VecDeque::new(),
                    rainbow: Vec::new(),
                };
                fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                score = 0;
                speed = 0.3;
                last_update = get_time();
                game_over = false;
            }
        }
        next_frame().await
    }
}
