use std::{thread, time::{Duration, Instant}, io::{stdout, stdin, Read, Write}, sync::{Mutex, Arc}};
use crossterm::*;
use owo_colors::{OwoColorize, AnsiColors, DynColors};
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
// use noise::*;
use bracket_noise::prelude::*;

#[derive(FromPrimitive, Debug)]
enum Keys {
    Enter = 13,
    Exit = 3,
    Up = 119,
    Down = 115,
    Right = 100,
    Left = 97,
    Unknown = 0
}

const BRAILLE_TABLE: [&str; 13] = [" ", " ", "⠐", "⠐", "⠌", "⠌", "⠇", "⠇", "⠳", "⠳", "⠷", "⠷", "⠿"];
const COLOR_TABLE: [AnsiColors; 10] = [
    AnsiColors::Blue, 
    AnsiColors::BrightBlue, 
    AnsiColors::Green, 
    AnsiColors::BrightGreen, 
    AnsiColors::Yellow, 
    AnsiColors::BrightYellow, 
    AnsiColors::Red,
    AnsiColors::BrightRed,
    AnsiColors::Magenta,
    AnsiColors::White];

const MOVE_SPEED: f32 = 1.0;
const PRINT_RANGE: bool = false;

const SIZE_MUL: f32 = 1.0;
const INTENSITY: f32 = 1.0;
const UPPER_BOUND: f32 = 1.0;
const LOWER_BOUND: f32 = -1.0;
const NEW_UP: f32 = 12.0;
const NEW_LOW: f32 = 0.0;

fn main() {
    let (w,h) = terminal::size().unwrap();
    terminal::enable_raw_mode().unwrap();
    execute!(
        stdout(), 
        terminal::EnterAlternateScreen, 
        cursor::MoveTo(0, 0));

    let noise_source = FastNoise::new();
    let mut buffer: Vec<AnsiColors> = vec![AnsiColors::Black; ((w+1)*(h+1)) as usize];

    let keys_pressed: Vec<Keys> = Vec::new();
    let keys_pressed_mutex = Mutex::new(keys_pressed);
    let keys_pressed_arc = Arc::new(keys_pressed_mutex);

    let mut offset = (0.0, 0.0);

    {
        let arc = keys_pressed_arc.clone();
        // Input loop
        thread::spawn(move || {
            loop {
                let mut input = [1];
                stdin().read(&mut input).unwrap();

                if let Some(key) = FromPrimitive::from_u8(input[0]) {
                    let mut handle = arc.lock().unwrap();
                    handle.push(key);
                    // match key {
                    //     Keys::Enter | Keys::Exit => handle.push(Keys::Enter),
                    //     // Keys::Up =>    execute!(stdout(), cursor::MoveUp(1)).unwrap(),
                    //     // Keys::Down =>  execute!(stdout(), cursor::MoveDown(1)).unwrap(),
                    //     // Keys::Left =>  execute!(stdout(), cursor::MoveLeft(1)).unwrap(),
                    //     // Keys::Right => execute!(stdout(), cursor::MoveRight(1)).unwrap(),
                    //     Keys::Up =>    OFFSET.0 += 1.0 * MOVE_SPEED,
                    //     Keys::Down =>  OFFSET.0 -= 1.0 * MOVE_SPEED,
                    //     Keys::Left =>  OFFSET.1 += 1.0 * MOVE_SPEED,
                    //     Keys::Right => OFFSET.1 -= 1.0 * MOVE_SPEED,
                    //     _ => ()
                    // }
                }
            }
        });
    }

    'main: loop {
        let handle = keys_pressed_arc.lock().unwrap();
        for key in &*handle {
            match key {
                Keys::Enter | Keys::Exit => {
                    exit((w,h));
                    break 'main;
                },
                Keys::Up =>    offset.0 += 1.0 * MOVE_SPEED,
                Keys::Down =>  offset.0 -= 1.0 * MOVE_SPEED,
                Keys::Left =>  offset.1 += 1.0 * MOVE_SPEED,
                Keys::Right => offset.1 -= 1.0 * MOVE_SPEED,
                _ => ()
            }
        }
        
        let timer_start = Instant::now();

        let mut range_of_unmapped = (9999.0, -9999.0);
        let mut range_of_mapped = (9999.0, -9999.0);

        for y in 0..h {
            for x in 0..w {
                // let mut noise_value = noise_source.get([(t + 2.0*x as f64) / w as f64 * SIZE_MUL, y as f64 / h as f64 * SIZE_MUL + t]);
                let mut noise_value = noise_source.get_noise(
                    (x as f32) * SIZE_MUL + offset.0, 
                    (y as f32) * SIZE_MUL + offset.1);

                if noise_value > range_of_unmapped.1 { range_of_unmapped.1 = noise_value; }
                if noise_value < range_of_unmapped.0 { range_of_unmapped.0 = noise_value; }

                map(&mut noise_value);

                if noise_value > range_of_mapped.1 { range_of_mapped.1 = noise_value; }
                if noise_value < range_of_mapped.0 { range_of_mapped.0 = noise_value; }

                let braille = match (noise_value * INTENSITY) as u8 {
                    i if i < BRAILLE_TABLE.len() as u8 => BRAILLE_TABLE[i as usize],
                    _ => "⠿"
                };
                
                let col = match (noise_value * INTENSITY) as u8 {
                    i if i < COLOR_TABLE.len() as u8 => COLOR_TABLE[i as usize],
                    _ => AnsiColors::BrightWhite
                };

                buffer[((x+1)*(y+1)) as usize] = col;
                // println!("{}", (x+1)*(y+1));
                // assert_eq!(buffer[(x*y) as usize], col);
                assert_eq!(offset.0, 0.0);
            }
        }
        
        let beforePrint = timer_start.elapsed().as_millis();

        let mut end_string = String::new();
        for i in (1 as usize)..buffer.len() {
            end_string.push_str(&format!("{}", " ".on_color(DynColors::Ansi(buffer[i]))));
            // print!("{}", " ".on_color(DynColors::Ansi(*color)));
        }

        print!("{}", end_string);
        // println!("{}", timer_start.elapsed().as_millis() - beforePrint);

        execute!(
            stdout(), 
            cursor::MoveTo(0,0));

        if PRINT_RANGE == true {
            println!("{:?}", range_of_unmapped);
            println!("{:?}", range_of_mapped);
        }
        
        sleep(2000);
        execute!(
            stdout(),
            // terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        );
    }

    execute!(
        stdout(),
        cursor::SetCursorShape(cursor::CursorShape::Block),
        cursor::MoveTo(0, 0)
    );
}

fn exit(orig_size: (u16, u16)) {
    terminal::disable_raw_mode().unwrap();
        execute!(
            stdout(), 
            terminal::SetSize(orig_size.0, orig_size.1),
            terminal::LeaveAlternateScreen,
            terminal::Clear(terminal::ClearType::All));
}

fn sleep(millis: u64) {
    stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(millis));
}

// fn map(x: &mut f64) {
fn map(x: &mut f32) {
    *x = (*x-LOWER_BOUND)*((NEW_UP-NEW_LOW)/(UPPER_BOUND-LOWER_BOUND))+NEW_LOW;
}
