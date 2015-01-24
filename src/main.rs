extern crate tcod;
use tcod::{Console, BackgroundFlag, KeyCode, Key};

mod entities;

static WIDTH: i32 = 80;
static HEIGHT: i32 = 50;

fn render(console: &mut Console, character: &entities::Character) {
    console.clear();
    console.put_char(character.position.x, character.position.y, '@', BackgroundFlag::Set);
    Console::flush();
}

fn main() {
    let mut con = Console::init_root(WIDTH, HEIGHT, "Roguelike", false);
    let mut exit = false;

    let mut character = entities::Character{position: entities::Coordinates{x:40, y:25}, c: '@'};

    render(&mut con, &character);
    while !(Console::window_closed() || exit) {
        let keypress = Console::wait_for_keypress(true);
        match keypress.key {
            Key::Special(KeyCode::Escape) => exit = true,
            Key::Special(KeyCode::Up) => {
                if character.position.y > 0 {
                    character.position.y -= 1;
                }
            },
            Key::Special(KeyCode::Down) => {
                if character.position.y < HEIGHT {
                    character.position.y += 1;
                }
            },
            Key::Special(KeyCode::Right) => {
                if character.position.x < WIDTH {
                    character.position.x += 1;
                }
            },
            Key::Special(KeyCode::Left) => {
                if character.position.x > 0 {
                    character.position.x -= 1;
                }
            },
            _ => {}
        }

        render(&mut con, &character);
    }
}
