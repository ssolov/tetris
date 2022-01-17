use crossterm::{
    cursor,
    event::{read, Event, EventStream, KeyCode, KeyEvent},
    style,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand, QueueableCommand, Result,
};
use futures::{executor, select, FutureExt, StreamExt};
use futures_timer::Delay;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::{
    io::{stdout, Write},
    time::Duration,
};

mod shapes;

use shapes::{Direction, Position, Shape, ShapeType};

fn validate(board: &[[u8; 10]], shape_pos: &[Position; 4]) -> bool {
    for p in shape_pos {
        if p.x >= 10 || p.y >= board.len() || board[p.y][p.x] != 0 {
            return false;
        }
    }

    true
}

fn change(board: &mut [[u8; 10]], pos: &[Position; 4], occupied: u8) {
    pos.iter().for_each(|p| board[p.y][p.x] = occupied)
}

fn print_board(board: &[[u8; 10]], score: u32, speed: u64) {
    let mut row = 0;
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(0, row)).unwrap();
    stdout
        .queue(style::Print(&format!("\u{250C}{:\u{2500}>20}", "\u{2510}")))
        .unwrap();
    for y in 2..board.len() {
        row += 1;
        stdout.queue(cursor::MoveTo(0, row)).unwrap();
        stdout
            .queue(style::Print(&format!(
                "\u{2502}{}\u{2502}",
                board[y]
                    .iter()
                    .map(|r| format!("{}", if r == &0 { ' ' } else { '\u{2587}' }))
                    .join(" ")
            )))
            .unwrap();
    }

    row += 1;
    stdout.queue(cursor::MoveTo(0, row)).unwrap();
    stdout
        .queue(style::Print(&format!("\u{2514}{:\u{2500}>20}", "\u{2518}")))
        .unwrap();

    row += 2;
    stdout.queue(cursor::MoveTo(0, row)).unwrap();
    stdout
        .queue(style::Print(&format!(
            "Score: {} Speed: {}",
            score,
            1000 - speed
        )))
        .unwrap();
    stdout.flush().unwrap();
}

fn remove_completed_lines(board: &mut [[u8; 10]]) -> u32 {
    let mut score = 0_u32;
    for y in 0..board.len() {
        if board[y].iter().find(|&n| n == &0).is_none() {
            let mut prev = y;
            for b in (0..prev).rev() {
                board[prev] = board[b];
                prev = b;
            }
            board[0] = [0; 10];
            score += 10;
        }
    }

    score
}

fn move_shape_down(shape: Shape, board: &[[u8; 10]], steps: Option<usize>) -> Option<Shape> {
    let mut steps = steps.unwrap_or(board.len());
    let mut shape = Some(shape);

    while steps > 0 {
        steps -= 1;
        let next = shape.as_ref().map(|s| s.down()).flatten();

        if next.as_ref().filter(|s| validate(board, &s.body)).is_some() {
            shape = next;
        } else {
            break;
        }
    }

    shape
}

fn random_shape() -> Shape {
    let nr = thread_rng().gen_range(0..=16);
    match nr {
        0 => Shape::new(ShapeType::SForm, Direction::Top),
        1 => Shape::new(ShapeType::SForm, Direction::Left),
        2 => Shape::new(ShapeType::LForm, Direction::Top),
        3 => Shape::new(ShapeType::LForm, Direction::Left),
        4 => Shape::new(ShapeType::LForm, Direction::Bottom),
        5 => Shape::new(ShapeType::LForm, Direction::Right),
        6 => Shape::new(ShapeType::LMirrored, Direction::Top),
        7 => Shape::new(ShapeType::LMirrored, Direction::Left),
        8 => Shape::new(ShapeType::LMirrored, Direction::Bottom),
        9 => Shape::new(ShapeType::LMirrored, Direction::Right),
        10 => Shape::new(ShapeType::TForm, Direction::Top),
        11 => Shape::new(ShapeType::TForm, Direction::Left),
        12 => Shape::new(ShapeType::TForm, Direction::Bottom),
        13 => Shape::new(ShapeType::TForm, Direction::Right),
        14 => Shape::new(ShapeType::Line, Direction::Top),
        15 => Shape::new(ShapeType::Line, Direction::Left),
        _ => Shape::new(ShapeType::Quadrat, Direction::Top),
    }
}

async fn run_game() -> Result<()> {
    let mut event_stream = EventStream::new();
    let mut board = [[0_u8; 10]; 22];
    let mut shape = random_shape();
    let mut down_delay = 1000;
    let speed_up_delay = 30;
    let mut score = 0;

    let mut speed_up = Delay::new(Duration::from_secs(speed_up_delay)).fuse();
    let mut down = Delay::new(Duration::from_millis(down_delay)).fuse();

    loop {
        let mut next_event = event_stream.next().fuse();

        change(&mut board, &shape.body, 1);
        print_board(&board, score, down_delay);
        change(&mut board, &shape.body, 0);

        select! {
            _ = speed_up => {
                down_delay -= 100;
                speed_up = Delay::new(Duration::from_secs(speed_up_delay)).fuse();
            },
            _ = down => {
                down = Delay::new(Duration::from_millis(down_delay)).fuse();

                if let Some(next_shape) = shape.down().filter(|s| validate(&board, &s.body)) {
                    shape = next_shape;
                } else {
                    change(&mut board, &shape.body, 1);

                    shape = random_shape();
                    if !validate(&board, &shape.body) {
                        change(&mut board, &shape.body, 1);
                        break;
                    }
                }
            },
            event = next_event => {
                if let Some(next_shape) = match event {
                    Some(Ok(Event::Key(KeyEvent { code: KeyCode::Left, ..}))) => shape.left(),
                    Some(Ok(Event::Key(KeyEvent { code: KeyCode::Right, ..}))) => shape.right(),
                    Some(Ok(Event::Key(KeyEvent { code: KeyCode::Up, ..}))) => shape.turn_left(),
                    Some(Ok(Event::Key(KeyEvent { code: KeyCode::Down, ..}))) => move_shape_down(shape.clone(), &board, Some(3)),
                    Some(Ok(Event::Key(KeyEvent { code: KeyCode::Char(' '), ..}))) => move_shape_down(shape.clone(), &board, None),
                    Some(Ok(Event::Key(KeyEvent { code: KeyCode::Esc, ..}))) => break,
                    _ => None,

                }.filter(|s| validate(&board, &s.body)) {
                    shape = next_shape;
                }
            },
        };

        let new_score = remove_completed_lines(&mut board);
        if new_score > 0 {
            score += new_score;
            print_board(&board, score, down_delay);
        }
    }

    Ok(())
}

fn print_help() {
    let mut stdout = stdout();
    stdout.queue(cursor::Hide).unwrap();
    stdout.queue(cursor::MoveTo(0, 0)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();

    stdout.queue(style::Print("Key bindings:")).unwrap();
    stdout.queue(cursor::MoveTo(0, 1)).unwrap();
    stdout
        .queue(style::Print("\u{2190} - Move to the left"))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 2)).unwrap();
    stdout
        .queue(style::Print("\u{2192} - Move to the right"))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 3)).unwrap();
    stdout
        .queue(style::Print("\u{2191} - Rotate 90°"))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 4)).unwrap();
    stdout
        .queue(style::Print("\u{2193} - Move down 3 lines"))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 5)).unwrap();
    stdout.queue(style::Print("SPACE - drop down")).unwrap();
    stdout.queue(cursor::MoveTo(0, 6)).unwrap();
    stdout.queue(style::Print("ESC - quit the game")).unwrap();
    stdout.queue(cursor::MoveTo(0, 8)).unwrap();
    stdout
        .queue(style::Print("Press any key to starg the game"))
        .unwrap();

    stdout.flush().unwrap();

    read().unwrap();

    stdout.execute(Clear(ClearType::All)).unwrap();
}

fn main() {
    enable_raw_mode().unwrap();
    print_help();

    let _ = executor::block_on(run_game());

    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(0, 25)).unwrap();
    stdout.queue(style::Print("GAME OVER\n\n")).unwrap();
    stdout.queue(cursor::Show).unwrap();
    stdout.flush().unwrap();
    disable_raw_mode().unwrap();
}
