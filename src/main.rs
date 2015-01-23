extern crate tcod;
use tcod::{Console, BackgroundFlag, KeyCode, Key};

static WIDTH: i32 = 80;
static HEIGHT: i32 = 50;

struct Character {
    x: i32,
    y: i32
}

fn render(console: &mut Console, character: &Character) {
    console.clear();
    console.put_char(character.x, character.y, '@', BackgroundFlag::Set);
    Console::flush();
}

fn main() {
    let mut con = Console::init_root(WIDTH, HEIGHT, "Roguelike", false);
    let mut exit = false;

    let mut character = Character{x:40, y:25};

    render(&mut con, &character);
    while !(Console::window_closed() || exit) {
        let keypress = Console::wait_for_keypress(true);
        match keypress.key {
            Key::Special(KeyCode::Escape) => exit = true,
            Key::Special(KeyCode::Up) => {
                if character.y > 0 {
                    character.y -= 1;
                }
            },
            Key::Special(KeyCode::Down) => {
                if character.y < HEIGHT {
                    character.y += 1;
                }
            },
            Key::Special(KeyCode::Right) => {
                if character.x < WIDTH {
                    character.x += 1;
                }
            },
            Key::Special(KeyCode::Left) => {
                if character.x > 0 {
                    character.x -= 1;
                }
            },
            _ => {}
        }

        render(&mut con, &character);
    }
}
