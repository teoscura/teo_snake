extern crate rand;
extern crate device_query;

use std::{vec, thread::sleep, time::Duration};
use rand::*;
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main(){
    let device_state = DeviceState::new();
    let mut rows = [['⬛'; 10]; 10];
    let mut score: i32 = 0;
    let mut direction: u8 = 0; // = Up, 1 = Left, 2 = Down, 3= Right
    let mut positions:Vec<(usize, usize)>  = vec![(5,5)]; //every square taken by our slithery friend
    let mut close_game: bool = false;
    rows[5][5] = '🟩';
    rows[3][5] = '🍎';
    while !close_game {
        print!("\x1B[2J\x1B[1;1H");
        direction = input_direction(direction, &device_state);
        close_game = game_tick(&mut rows, &mut positions, direction, &mut score);
        draw_game(&rows, &score);
        sleep(Duration::from_millis(250));
    }
    println!("Game over! Final Score: {}", score);
} 

fn draw_game(a: &[[char;10];10], score: &i32){
    println!("");
    println!("  Score:{:02} teo.snake", &score);
    for i in 1..10 {
        print!("  ");
        for j in 1..10 {
            print!("{}", a[i][j]);
        }
        println!("");
    }
}

fn game_tick(a: &mut[[char;10];10], vec: &mut Vec<(usize, usize)>, d: u8, s: &mut i32)-> bool {
    let mut game_over: bool = false;
    let newpos: (usize, usize)= newposition(d, vec[0], &mut game_over);
    if game_over == true {
        return game_over
    }

    for i in 0..vec.len() {
        if newpos == vec[i] {
            game_over = true;
            return game_over
        }
    }
    vec.insert(0, newpos);

    let mut next_fruitx:usize;
    let mut next_fruity:usize;
    let mut checkpass: bool;

    'looptillvalid: loop {
        checkpass = true;
        next_fruitx = rand::thread_rng().gen_range(1..10);
        next_fruity = rand::thread_rng().gen_range(1..10);
        for i in 0..vec.len() {
            if (next_fruitx, next_fruity) == vec[i] {
                checkpass = false;
                break;
            }
        }
        if checkpass == true {    
            break 'looptillvalid;
        }
    }
    let lastpos: (usize, usize) = vec.pop().unwrap();
    a[lastpos.0][lastpos.1] = '⬛';

    match a[newpos.0][newpos.1] {
        '🍎' => { vec.push(lastpos);
            a[newpos.0][newpos.1] = '🟩'; 
            a[lastpos.0][lastpos.1] = '🟩';
            *s += 1; 
            game_over = false;
            a[next_fruitx][next_fruity] = '🍎';
        }
         _  => { a[newpos.0][newpos.1] = '🟩';
            game_over = false}
    }

    game_over
}

fn newposition(d: u8, first: (usize, usize), go: &mut bool) -> (usize, usize){
    let mut ret: (usize, usize) = (0,0);
    match d {
        0 => { ret.0 = first.0 - 1; ret.1 = first.1}
        1 => { ret.0 = first.0; ret.1 = first.1 - 1}
        2 => { ret.0 = first.0 + 1; ret.1 = first.1}
        3 => { ret.0 = first.0; ret.1 = first.1 + 1}
        _ => {println!("Invalid Direction")}
    }

    if ret.0 < 1 || ret.0 > 9 || ret.1 < 1 || ret.1 > 9 {
        *go = true;
    }

    ret
}

fn input_direction(d: u8, d_state: &DeviceState) -> u8{

    let mut dir: u8 = d;
    
    let keys: Vec<Keycode> = d_state.get_keys(); 
        if keys.len() != 0 {
            match keys[0] {
                
                Keycode::Up => {dir = 0}
                Keycode::Left => {dir = 1}
                Keycode::Down => {dir = 2}
                Keycode::Right => {dir = 3}
                _ => ()
            }
        }
    dir
}

