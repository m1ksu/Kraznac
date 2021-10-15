use std::{thread, time::{Duration, Instant}, io::{stdout, stdin, Read, Write}};
use crossterm::*;
use owo_colors::{OwoColorize, AnsiColors, DynColors};
// use rand::prelude::random;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use noise::*;


#[derive(FromPrimitive)]
enum Keys {
    Enter = 13,
    Exit = 3,
    Up = 119,
    Down = 115,
    Right = 100,
    Left = 97,
    Unknown = 0
}

fn main() {
    let orig_size = terminal::size().unwrap();
    terminal::enable_raw_mode().unwrap();
    execute!(
        stdout(), 
        terminal::EnterAlternateScreen, 
        cursor::MoveTo(0, 0));

    let mut t = 0.0;
    loop {
    if let Ok((w, h)) = terminal::size() {
        let tim_genS = Instant::now();
        let perlin = Billow::new();

        const SIZE_MUL: f64 = 5.0;
        const INTENSITY: f64 = 11.0;
        const LOWER_BOUND: f64 = -0.86;
        const UPPER_BOUND: f64 = 0.85;
        const NEW_UP: f64 = 1.0;
        const NEW_LOW: f64 = 0.05;
        
        let mut ext = (9999.0, -9999.0);
        let mut extN = (9999.0, -9999.0);
        let mut buffer: Vec<AnsiColors> = Vec::new();

        for y in 0..h {
            for x in 0..w {
                fn map(x: f64) -> f64 {
                    return (x-LOWER_BOUND)*((NEW_UP-NEW_LOW)/(UPPER_BOUND-LOWER_BOUND))+NEW_LOW;
                }
                // let num = random::<u8>()/(255/6);
                let uscp = perlin.get([(t + 2.0*x as f64) / w as f64 * SIZE_MUL, y as f64 / h as f64 * SIZE_MUL + t]);
                if uscp > ext.1 { ext.1 = uscp; }
                if uscp < ext.0 { ext.0 = uscp; }
                // println!("{}", uscp);
                // let perl = (uscp-(-1.0))/(1.0-(-1.0)) * ((1.0)-(0.0));
                // let perl = uscp-(LOWER_BOUND)/(UPPER_BOUND-(LOWER_BOUND)) * (NEW_UP - NEW_LOW);
                let perl = map(uscp);
                if perl > extN.1 { extN.1 = perl; }
                if perl < extN.0 { extN.0 = perl; }
                let braille = match (perl * INTENSITY) as u8 {
                    0 => " ",
                    1 => " ",
                    2 => "⠐",
                    3 => "⠐",
                    4 => "⠌",
                    5 => "⠌",
                    6 => "⠇",
                    7 => "⠇",
                    8 => "⠳",
                    9 => "⠳",
                    10 => "⠷",
                    11 => "⠷",
                    12 => "⠿",
                    _ => " "
                };
                /*
                    // println!("{}", perlin.get([(x as f64)/(w as f64), (y as f64)/(h as f64)]));
                    // let num = perlin.get((x as f64)*(y as f64));
                    // print!(" {} {}", x,y);
                    // let color = match (perl * INTENSITY) as u8 {
                    //     0 => DynColors::Ansi(AnsiColors::Black),
                    //     1 => DynColors::Ansi(AnsiColors::Blue),
                    //     2 => DynColors::Ansi(AnsiColors::BrightBlue),
                    //     3 => DynColors::Ansi(AnsiColors::Green),
                    //     4 => DynColors::Ansi(AnsiColors::BrightGreen),
                    //     5 => DynColors::Ansi(AnsiColors::Yellow),
                    //     6 => DynColors::Ansi(AnsiColors::BrightYellow),
                    //     7 => DynColors::Ansi(AnsiColors::Red),
                    //     8 => DynColors::Ansi(AnsiColors::BrightRed),
                    //     9 => DynColors::Ansi(AnsiColors::Magenta),
                    //     10 => DynColors::Ansi(AnsiColors::White),
                    //     _ => DynColors::Ansi(AnsiColors::BrightWhite)
                    // };
                    // buffer[(w*h) as usize] = match (perl * INTENSITY) as u8 {
                */

                let col = match (perl * INTENSITY) as u8 {
                    0 => AnsiColors::Black,
                    1 => AnsiColors::Blue,
                    2 => AnsiColors::BrightBlue,
                    3 => AnsiColors::Green,
                    4 => AnsiColors::BrightGreen,
                    5 => AnsiColors::Yellow,
                    6 => AnsiColors::BrightYellow,
                    7 => AnsiColors::Red,
                    8 => AnsiColors::BrightRed,
                    9 => AnsiColors::Magenta,
                    10 => AnsiColors::White,
                    _ => AnsiColors::BrightWhite
                };
                buffer.push(col);
                // print!("{}", " ".on_color(color));
            }
        }
		
		let beforePrint = tim_genS.elapsed().as_millis();

        let mut end_string = String::new();
        for color in buffer {
            end_string.push_str(&format!("{}", " ".on_color(DynColors::Ansi(color))));
        }

		execute!(
            stdout(), 
            cursor::MoveTo(0,0));

		print!("{}", end_string);
        sleep(200);
		t += 1.0;
     }
    }

    execute!(
        stdout(),
        cursor::SetCursorShape(cursor::CursorShape::Block),
        cursor::MoveTo(0, 0)
    );

    // Input loop
    loop {
        let mut input = [1];
        stdin().read(&mut input).unwrap();
        if let Some(key) = FromPrimitive::from_u8(input[0]) {
            match key {
                Keys::Enter | Keys::Exit => break,
                Keys::Up =>    execute!(stdout(), cursor::MoveUp(1)).unwrap(),
                Keys::Down =>  execute!(stdout(), cursor::MoveDown(1)).unwrap(),
                Keys::Left =>  execute!(stdout(), cursor::MoveLeft(1)).unwrap(),
                Keys::Right => execute!(stdout(), cursor::MoveRight(1)).unwrap(),
                _ => ()
            }
        }
    }
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
