use std::io::{stdout, Stdout};
use std::io;
use std::io::Read;

use crossterm::{self, terminal};
use crossterm::{execute,cursor};
use crossterm::terminal::{Clear,ClearType};
use crossterm::cursor::Hide;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;

use rand::Rng;

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, Hide).unwrap();
    

    let mut map: [[char; 13]; 13] = [
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.'],
        ['.','.','.','.','.','.','.','.','.','.','.','.','.']
    ];

    //snake
    let mut position: (isize,isize) = (2, 6);
    let mut body: Vec<(isize, isize)> = vec![(0,6),(1,6)];
    map[position.1 as usize][position.0 as usize] = '#';
    for limb in &body{
        map[limb.1 as usize][limb.0 as usize] = '#';
    }
    let mut length: usize = 2;
    let mut direction: (isize,isize) = (1,0);

    //apple
    let mut apple_position: (isize, isize) = (rand::thread_rng().gen_range(0..=12),rand::thread_rng().gen_range(0..=12));
    map[apple_position.1 as usize][apple_position.0 as usize] = '@';

    //time
    let mut move_cooldown: u32 = 0;

    let input_channel = spawn_input_channel();

    loop {
        if move_cooldown > 0 {
            move_cooldown -= 1;
        }

        if move_cooldown <= 0{
            map[body[0].1 as usize][body[0].0 as usize] = '.';
            body.push(position);
            if body.len() > length {
                body.remove(0 as usize);
            }

            position.0 += direction.0;
            position.1 += direction.1;

            if position.0 < 0 || position.0 > 12 || position.1 < 0 || position.1 > 12 {
                lose(length - 2, &stdout, map);
            }
            else if body.contains(&position){
                lose(length - 2, &stdout, map);
            }

            map[position.1 as usize][position.0 as usize] = '#';

            move_cooldown = 3;
        }

        if position == apple_position{
            length += 1;
            
            while body.contains(&apple_position) ||  position == apple_position{
                apple_position = (rand::thread_rng().gen_range(0..=12),rand::thread_rng().gen_range(0..=12));
                map[apple_position.1 as usize][apple_position.0 as usize] = '@';
            }
        }
        
        update_screen(&stdout, map);

        std::thread::sleep(std::time::Duration::from_millis(33));
    
        match input_channel.try_recv() {
            Ok(key) => match key{
                [b'w'] => {
                    if direction != (0,1){
                        direction = (0,-1);
                    }
                },
                [b's'] => {
                    if direction != (0,-1){
                        direction = (0,1);
                    }
                },
                [b'a'] => {
                    if direction != (1,0){
                        direction = (-1,0);
                    }
                },
                [b'd'] => {
                    if direction != (-1,0){
                        direction = (1,0);
                    }
                },
                [b'q'] => break,
                _ => ()
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
    }
}

fn display(map : [[char; 13]; 13]){
    for slice in map {
        for tile in slice {
            print!("{tile}");
        }
        println!();
    }
}

fn update_screen (mut stdout : &Stdout, map: [[char; 13]; 13]){
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout, Clear(ClearType::FromCursorDown), cursor::MoveTo(0, 0)).unwrap();

    display(map);
}

fn spawn_input_channel() -> Receiver<[u8; 1]>{
    let (tx, rx) = mpsc::channel::<[u8; 1]>();
    std::thread::spawn(move || loop {
        let mut buffer = [0; 1];
        io::stdin().read(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn lose(score: usize, stdout : &Stdout, map: [[char; 13]; 13]) {
    update_screen(&stdout, map);

    println!("You died! Your score was: {score}");
    std::process::exit(0);
}