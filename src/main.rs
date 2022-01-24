use crossterm::{
    cursor,
    event::{read, Event, EventStream, KeyCode, KeyEvent},
    style::{self, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand, QueueableCommand, Result,
};
use futures::{executor, select, FutureExt, StreamExt};
use futures_timer::Delay;
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

fn change(board: &mut [[u8; 10]], shape: &Shape, occupied: bool) {
    shape.body.iter().for_each(|p| {
        board[p.y][p.x] = if occupied {
            match shape.shape_type {
                ShapeType::TForm => 1,
                ShapeType::LForm => 2,
                ShapeType::LMirrored => 3,
                ShapeType::Line => 4,
                ShapeType::SForm => 5,
                ShapeType::SMirrored => 6,
                ShapeType::Quadrat => 7,
            }
        } else {
            0
        }
    })
}

fn print_board(board: &[[u8; 10]], score: u32, speed: u64) {
    let mut row = 0;
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(0, row)).unwrap();
    stdout
        .queue(style::PrintStyledContent(
            format!("\u{250C}{:\u{2500}>20}", "\u{2510}").yellow(),
        ))
        .unwrap();
    for line in board.iter().skip(2) {
        row += 1;
        stdout.queue(cursor::MoveTo(0, row)).unwrap();
        stdout
            .queue(style::PrintStyledContent("\u{2502}".yellow()))
            .unwrap();
        for (i, cell) in line.iter().enumerate() {
            match cell {
                1 => stdout.queue(style::PrintStyledContent("\u{2587}".dark_cyan())),
                2 => stdout.queue(style::PrintStyledContent("\u{2587}".dark_green())),
                3 => stdout.queue(style::PrintStyledContent("\u{2587}".dark_red())),
                4 => stdout.queue(style::PrintStyledContent("\u{2587}".dark_blue())),
                5 => stdout.queue(style::PrintStyledContent("\u{2587}".dark_magenta())),
                6 => stdout.queue(style::PrintStyledContent("\u{2587}".red())),
                7 => stdout.queue(style::PrintStyledContent("\u{2587}".dark_yellow())),
                _ => stdout.queue(style::Print(" ")),
            }
            .unwrap();

            if i < line.len() - 1 {
                stdout.queue(style::Print(" ")).unwrap();
            }
        }

        stdout
            .queue(style::PrintStyledContent("\u{2502}".yellow()))
            .unwrap();
    }

    row += 1;
    stdout.queue(cursor::MoveTo(0, row)).unwrap();
    stdout
        .queue(style::PrintStyledContent(
            format!("\u{2514}{:\u{2500}>20}", "\u{2518}").yellow(),
        ))
        .unwrap();

    row += 2;
    stdout.queue(cursor::MoveTo(0, row)).unwrap();
    stdout
        .queue(style::PrintStyledContent("Score: ".dark_green()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent(format!("{}", score).dark_red()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent(" Speed: ".dark_green()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent(
            format!("{}", 1100 - speed).dark_red(),
        ))
        .unwrap();
    stdout.flush().unwrap();
}

fn remove_completed_lines(board: &mut [[u8; 10]]) -> u32 {
    let mut score = 0_u32;
    let mut add = 10;
    for y in 0..board.len() {
        if !board[y].iter().any(|n| n == &0) {
            let mut prev = y;
            for b in (0..prev).rev() {
                board[prev] = board[b];
                prev = b;
            }
            board[0] = [0; 10];
            score += add;
            add *= 2;
        }
    }

    score
}

fn move_shape_down(shape: Shape, board: &[[u8; 10]], steps: Option<usize>) -> Option<Shape> {
    let mut steps = steps.unwrap_or(board.len());
    let mut shape = Some(shape);

    while steps > 0 {
        steps -= 1;
        let next = shape.as_ref().and_then(|s| s.down());

        if next.as_ref().filter(|s| validate(board, &s.body)).is_some() {
            shape = next;
        } else {
            break;
        }
    }

    shape
}

fn random_shape() -> Shape {
    let nr = thread_rng().gen_range(0..=27);
    match nr {
        0 => Shape::new(ShapeType::SForm, Direction::Top),
        1 => Shape::new(ShapeType::Quadrat, Direction::Top),
        2 => Shape::new(ShapeType::LForm, Direction::Top),
        3 => Shape::new(ShapeType::Line, Direction::Top),
        4 => Shape::new(ShapeType::TForm, Direction::Top),
        5 => Shape::new(ShapeType::SMirrored, Direction::Top),
        6 => Shape::new(ShapeType::LMirrored, Direction::Top),
        7 => Shape::new(ShapeType::SForm, Direction::Left),
        8 => Shape::new(ShapeType::Quadrat, Direction::Left),
        9 => Shape::new(ShapeType::LForm, Direction::Left),
        10 => Shape::new(ShapeType::Line, Direction::Left),
        11 => Shape::new(ShapeType::TForm, Direction::Left),
        12 => Shape::new(ShapeType::SMirrored, Direction::Left),
        13 => Shape::new(ShapeType::LMirrored, Direction::Left),
        14 => Shape::new(ShapeType::SForm, Direction::Bottom),
        15 => Shape::new(ShapeType::Quadrat, Direction::Bottom),
        16 => Shape::new(ShapeType::LForm, Direction::Bottom),
        17 => Shape::new(ShapeType::Line, Direction::Bottom),
        18 => Shape::new(ShapeType::TForm, Direction::Bottom),
        19 => Shape::new(ShapeType::SMirrored, Direction::Bottom),
        20 => Shape::new(ShapeType::LMirrored, Direction::Bottom),
        21 => Shape::new(ShapeType::SForm, Direction::Right),
        22 => Shape::new(ShapeType::Quadrat, Direction::Right),
        23 => Shape::new(ShapeType::LForm, Direction::Right),
        24 => Shape::new(ShapeType::Line, Direction::Right),
        25 => Shape::new(ShapeType::TForm, Direction::Right),
        26 => Shape::new(ShapeType::SMirrored, Direction::Right),
        _ => Shape::new(ShapeType::LMirrored, Direction::Right),
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

        change(&mut board, &shape, true);
        print_board(&board, score, down_delay);
        change(&mut board, &shape, false);

        select! {
            _ = speed_up => if down_delay > 100 {
                down_delay -= 100;
                speed_up = Delay::new(Duration::from_secs(speed_up_delay)).fuse();
            },
            _ = down => {
                down = Delay::new(Duration::from_millis(down_delay)).fuse();

                if let Some(next_shape) = shape.down().filter(|s| validate(&board, &s.body)) {
                    shape = next_shape;
                } else {
                    change(&mut board, &shape, true);

                    shape = random_shape();
                    if !validate(&board, &shape.body) {
                        change(&mut board, &shape, true);
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
    stdout.queue(cursor::MoveTo(0, 0)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout
        .queue(style::SetAttribute(style::Attribute::Bold))
        .unwrap();

    stdout
        .queue(style::PrintStyledContent("Key bindings:".dark_blue()))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 1)).unwrap();
    stdout
        .queue(style::PrintStyledContent("\u{2190}".dark_red()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent(" - ".dark_yellow()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent("Move to the left".dark_green()))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 2)).unwrap();
    stdout
        .queue(style::PrintStyledContent("\u{2192}".dark_red()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent(" - ".dark_yellow()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent("Move to the right".dark_green()))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 3)).unwrap();
    stdout
        .queue(style::PrintStyledContent("\u{2191}".dark_red()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent(" - ".dark_yellow()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent("Rotate 90°".dark_green()))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 4)).unwrap();
    stdout
        .queue(style::PrintStyledContent("\u{2193}".dark_red()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent(" - ".dark_yellow()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent("Move down 3 lines".dark_green()))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 5)).unwrap();
    stdout
        .queue(style::PrintStyledContent("SPACE".dark_red()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent(" - ".dark_yellow()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent("drop down".dark_green()))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 6)).unwrap();
    stdout
        .queue(style::PrintStyledContent("ESC".dark_red()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent(" - ".dark_yellow()))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent("quit the game".dark_green()))
        .unwrap();
    stdout.queue(cursor::MoveTo(0, 8)).unwrap();
    stdout
        .queue(style::PrintStyledContent(
            "Press any key to starg the game".dark_blue(),
        ))
        .unwrap();

    stdout
        .queue(style::SetAttribute(style::Attribute::Reset))
        .unwrap();

    stdout.flush().unwrap();

    read().unwrap();

    stdout.execute(Clear(ClearType::All)).unwrap();
}

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = stdout();
    stdout.queue(cursor::Hide).unwrap();
    print_help();

    let _ = executor::block_on(run_game());

    stdout.queue(cursor::MoveTo(4, 25)).unwrap();
    stdout
        .queue(style::SetAttribute(style::Attribute::Bold))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent("GAME OVER\n\n".dark_red()))
        .unwrap();
    stdout
        .queue(style::SetAttribute(style::Attribute::Reset))
        .unwrap();
    stdout.queue(cursor::Show).unwrap();
    stdout.flush().unwrap();
    disable_raw_mode().unwrap();
}
