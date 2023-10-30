extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn man() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,"",termion::clear::All,termion::cursor::Goto(1, 1),termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 1),
               termion::clear::CurrentLine)
                .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let device_state = DeviceState::new();
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            println!("Pressed key: {:?}", key);
        }
    }
}