use crossterm::{
    cursor,
    event::{Event, EventStream, KeyCode, KeyEvent},
    queue, style,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand, Result,
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

fn print_board(board: &[[u8; 10]]) {
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

        //if y < board.len() - 1 {
        //    row += 1;
        //    stdout.queue(cursor::MoveTo(0, row)).unwrap();
        //    stdout
        //        .queue(style::Print(&format!("\u{2502}{: >20}", "\u{2502}")))
        //        .unwrap();
        //}
    }

    row += 1;
    stdout.queue(cursor::MoveTo(0, row)).unwrap();
    stdout
        .queue(style::Print(&format!("\u{2514}{:\u{2500}>20}", "\u{2518}")))
        .unwrap();

    stdout.flush().unwrap();
}

fn remove_completed_lines(board: &mut [[u8; 10]]) {
    for y in 0..board.len() {
        if board[y].iter().find(|&n| n == &0).is_none() {
            let mut prev = y;
            for b in (0..prev).rev() {
                board[prev] = board[b];
                prev = b;
            }
            board[0] = [0; 10];
            print_board(&board);
        }
    }
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

    loop {
        let mut delay = Delay::new(Duration::from_millis(500)).fuse();
        let mut next_event = event_stream.next().fuse();

        change(&mut board, &shape.body, 1);
        print_board(&board);
        change(&mut board, &shape.body, 0);

        select! {
            _ = delay => {
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
                    Some(Ok(Event::Key(KeyEvent { code: KeyCode::Char(' '), ..}))) => {
                        let mut shape = Some(shape.clone());
                        loop {
                            let next = shape.as_ref().map(|s| s.down()).flatten();

                            if next
                                .as_ref()
                                .filter(|s| validate(&board, &s.body))
                                .is_some()
                            {
                                shape = next;
                            } else {
                                break;
                            }
                        }

                        shape
                    },
                    Some(Ok(Event::Key(KeyEvent { code: KeyCode::Esc, ..}))) => break,
                    _ => None,

                }.filter(|s| validate(&board, &s.body)) {
                    shape = next_shape;
                }
            },
        };

        remove_completed_lines(&mut board);
    }

    Ok(())
}

fn main() {
    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    queue!(stdout, cursor::Hide).unwrap();
    queue!(stdout, cursor::MoveTo(0, 0)).unwrap();
    queue!(stdout, Clear(ClearType::All)).unwrap();
    stdout.flush().unwrap();

    let _ = executor::block_on(run_game());

    queue!(stdout, cursor::Show).unwrap();
    stdout.flush().unwrap();
    disable_raw_mode().unwrap();
}
