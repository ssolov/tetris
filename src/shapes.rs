#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Position {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Position {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub(crate) type ShapeBody = [Position; 4];

#[derive(Debug, Clone)]
pub(crate) enum ShapeType {
    TForm,
    LForm,
    LMirrored,
    Line,
    SForm,
    SMirrored,
    Quadrat,
}

#[derive(Debug, Clone)]
pub(crate) struct Shape {
    pub(crate) body: ShapeBody,
    pub(crate) shape_type: ShapeType,
    direction: Direction,
}

impl Shape {
    pub(crate) fn new(t: ShapeType, d: Direction) -> Self {
        match t {
            ShapeType::Quadrat => quadrat(),
            ShapeType::Line => line(d),
            ShapeType::TForm => t_form(d),
            ShapeType::LForm => l_form(d),
            ShapeType::LMirrored => l_mirrored(d),
            ShapeType::SForm => s_form(d),
            ShapeType::SMirrored => s_mirrored(d),
        }
    }

    pub(crate) fn left(&self) -> Option<Shape> {
        let mut shape = self.clone();
        for p in shape.body.iter_mut() {
            p.x = p.x.checked_sub(1)?;
        }

        Some(shape)
    }

    pub(crate) fn right(&self) -> Option<Shape> {
        let mut shape = self.clone();
        for p in shape.body.iter_mut() {
            p.x = p.x.checked_add(1)?;
        }

        Some(shape)
    }

    pub(crate) fn down(&self) -> Option<Shape> {
        let mut shape = self.clone();
        for p in shape.body.iter_mut() {
            p.y = p.y.checked_add(1)?;
        }

        Some(shape)
    }

    pub(crate) fn turn_left(&self) -> Option<Shape> {
        let shape = self.clone();
        match self.shape_type {
            ShapeType::Line => line_turn_left(shape),
            ShapeType::TForm => t_form_turn_left(shape),
            ShapeType::LForm => l_form_turn_left(shape),
            ShapeType::LMirrored => l_mirrored_turn_left(shape),
            ShapeType::SForm => s_form_turn_left(shape),
            ShapeType::SMirrored => s_mirrored_turn_left(shape),
            ShapeType::Quadrat => Some(shape),
        }
    }
}

fn quadrat() -> Shape {
    let x = 4;
    let y = 0;
    Shape {
        shape_type: ShapeType::Quadrat,
        direction: Direction::Top,
        body: [
            Position::new(x, y),
            Position::new(x + 1, y),
            Position::new(x, y + 1),
            Position::new(x + 1, y + 1),
        ],
    }
}

fn line(direction: Direction) -> Shape {
    let x = if direction == Direction::Left || direction == Direction::Right {
        4
    } else {
        5
    };
    let y = 0;

    let body = match direction {
        Direction::Top | Direction::Bottom => [
            Position::new(x, y),
            Position::new(x, y + 1),
            Position::new(x, y + 2),
            Position::new(x, y + 3),
        ],
        Direction::Left | Direction::Right => [
            Position::new(x - 2, y),
            Position::new(x - 1, y),
            Position::new(x, y),
            Position::new(x + 1, y),
        ],
    };

    Shape {
        shape_type: ShapeType::Line,
        body,
        direction,
    }
}

fn line_turn_left(mut s: Shape) -> Option<Shape> {
    match s.direction {
        Direction::Top | Direction::Bottom => {
            s.body[0].x = s.body[0].x.checked_sub(2)?;
            s.body[0].y = s.body[0].y.checked_add(2)?;
            s.body[1].x = s.body[1].x.checked_sub(1)?;
            s.body[1].y = s.body[1].y.checked_add(1)?;
            s.body[3].x = s.body[3].x.checked_add(1)?;
            s.body[3].y = s.body[3].y.checked_sub(1)?;
            s.direction = Direction::Left;
        }
        Direction::Left | Direction::Right => {
            s.body[0].x = s.body[0].x.checked_add(2)?;
            s.body[0].y = s.body[0].y.checked_sub(2)?;
            s.body[1].x = s.body[1].x.checked_add(1)?;
            s.body[1].y = s.body[1].y.checked_sub(1)?;
            s.body[3].x = s.body[3].x.checked_sub(1)?;
            s.body[3].y = s.body[3].y.checked_add(1)?;
            s.direction = Direction::Top;
        }
    };

    Some(s)
}

fn t_form(direction: Direction) -> Shape {
    let x = 5;
    let y = 0;

    let body = match direction {
        Direction::Top => [
            Position::new(x, y),
            Position::new(x - 1, y + 1),
            Position::new(x, y + 1),
            Position::new(x + 1, y + 1),
        ],
        Direction::Bottom => [
            Position::new(x, y + 1),
            Position::new(x - 1, y),
            Position::new(x, y),
            Position::new(x + 1, y),
        ],
        Direction::Left => [
            Position::new(x, y),
            Position::new(x - 1, y + 1),
            Position::new(x, y + 1),
            Position::new(x, y + 2),
        ],
        Direction::Right => [
            Position::new(x, y),
            Position::new(x, y + 2),
            Position::new(x, y + 1),
            Position::new(x + 1, y + 1),
        ],
    };

    Shape {
        shape_type: ShapeType::TForm,
        body,
        direction,
    }
}

fn t_form_turn_left(mut s: Shape) -> Option<Shape> {
    match s.direction {
        Direction::Top => {
            s.body[3].x = s.body[3].x.checked_sub(1)?;
            s.body[3].y = s.body[3].y.checked_add(1)?;
            s.direction = Direction::Left;
        }
        Direction::Bottom => {
            s.body[0].y = s.body[0].y.checked_sub(2)?;
            s.body[1].x = s.body[1].x.checked_add(1)?;
            s.body[1].y = s.body[1].y.checked_add(1)?;
            s.direction = Direction::Right;
        }
        Direction::Left => {
            s.body[0].y = s.body[0].y.checked_add(2)?;
            s.body[3].x = s.body[3].x.checked_add(1)?;
            s.body[3].y = s.body[3].y.checked_sub(1)?;
            s.direction = Direction::Bottom;
        }
        Direction::Right => {
            s.body[1].x = s.body[1].x.checked_sub(1)?;
            s.body[1].y = s.body[1].y.checked_sub(1)?;
            s.direction = Direction::Top;
        }
    }

    Some(s)
}

fn l_form(direction: Direction) -> Shape {
    let x = 4;
    let y = 0;

    let body = match direction {
        Direction::Top => [
            Position::new(x, y),
            Position::new(x, y + 1),
            Position::new(x, y + 2),
            Position::new(x + 1, y + 2),
        ],
        Direction::Bottom => [
            Position::new(x, y + 2),
            Position::new(x, y + 1),
            Position::new(x, y),
            Position::new(x - 1, y),
        ],
        Direction::Left => [
            Position::new(x - 2, y + 1),
            Position::new(x - 1, y + 1),
            Position::new(x, y + 1),
            Position::new(x, y),
        ],
        Direction::Right => [
            Position::new(x + 2, y),
            Position::new(x + 1, y),
            Position::new(x, y),
            Position::new(x, y + 1),
        ],
    };

    Shape {
        shape_type: ShapeType::LForm,
        body,
        direction,
    }
}

fn l_form_turn_left(mut s: Shape) -> Option<Shape> {
    match s.direction {
        Direction::Top => {
            s.body[0].x = s.body[0].x.checked_sub(1)?;
            s.body[0].y = s.body[0].y.checked_add(1)?;
            s.body[2].x = s.body[2].x.checked_add(1)?;
            s.body[2].y = s.body[2].y.checked_sub(1)?;
            s.body[3].y = s.body[3].y.checked_sub(2)?;
            s.direction = Direction::Left;
        }
        Direction::Bottom => {
            s.body[0].x = s.body[0].x.checked_add(1)?;
            s.body[0].y = s.body[0].y.checked_sub(1)?;
            s.body[2].x = s.body[2].x.checked_sub(1)?;
            s.body[2].y = s.body[2].y.checked_add(1)?;
            s.body[3].y = s.body[3].y.checked_add(2)?;
            s.direction = Direction::Right;
        }
        Direction::Left => {
            s.body[0].x = s.body[0].x.checked_add(1)?;
            s.body[0].y = s.body[0].y.checked_add(1)?;
            s.body[2].x = s.body[2].x.checked_sub(1)?;
            s.body[2].y = s.body[2].y.checked_sub(1)?;
            s.body[3].x = s.body[3].x.checked_sub(2)?;
            s.direction = Direction::Bottom;
        }
        Direction::Right => {
            s.body[0].x = s.body[0].x.checked_sub(1)?;
            s.body[0].y = s.body[0].y.checked_sub(1)?;
            s.body[2].x = s.body[2].x.checked_add(1)?;
            s.body[2].y = s.body[2].y.checked_add(1)?;
            s.body[3].x = s.body[3].x.checked_add(2)?;
            s.direction = Direction::Top;
        }
    }

    Some(s)
}

fn l_mirrored(direction: Direction) -> Shape {
    let x = 5;
    let y = 0;

    let body = match direction {
        Direction::Top => [
            Position::new(x, y),
            Position::new(x, y + 1),
            Position::new(x, y + 2),
            Position::new(x - 1, y + 2),
        ],
        Direction::Bottom => [
            Position::new(x, y + 2),
            Position::new(x, y + 1),
            Position::new(x, y),
            Position::new(x + 1, y),
        ],
        Direction::Left => [
            Position::new(x - 2, y),
            Position::new(x - 1, y),
            Position::new(x, y),
            Position::new(x, y + 1),
        ],
        Direction::Right => [
            Position::new(x + 2, y + 1),
            Position::new(x + 1, y + 1),
            Position::new(x, y + 1),
            Position::new(x, y),
        ],
    };

    Shape {
        shape_type: ShapeType::LMirrored,
        body,
        direction,
    }
}

fn l_mirrored_turn_left(mut s: Shape) -> Option<Shape> {
    match s.direction {
        Direction::Top => {
            s.body[0].x = s.body[0].x.checked_sub(1)?;
            s.body[0].y = s.body[0].y.checked_add(1)?;
            s.body[2].x = s.body[2].x.checked_add(1)?;
            s.body[2].y = s.body[2].y.checked_sub(1)?;
            s.body[3].x = s.body[3].x.checked_add(2)?;
            s.direction = Direction::Left;
        }
        Direction::Bottom => {
            s.body[0].x = s.body[0].x.checked_add(1)?;
            s.body[0].y = s.body[0].y.checked_sub(1)?;
            s.body[2].x = s.body[2].x.checked_sub(1)?;
            s.body[2].y = s.body[2].y.checked_add(1)?;
            s.body[3].x = s.body[3].x.checked_sub(2)?;
            s.direction = Direction::Right;
        }
        Direction::Left => {
            s.body[0].x = s.body[0].x.checked_add(1)?;
            s.body[0].y = s.body[0].y.checked_add(1)?;
            s.body[2].x = s.body[2].x.checked_sub(1)?;
            s.body[2].y = s.body[2].y.checked_sub(1)?;
            s.body[3].y = s.body[3].y.checked_sub(2)?;
            s.direction = Direction::Bottom;
        }
        Direction::Right => {
            s.body[0].x = s.body[0].x.checked_sub(1)?;
            s.body[0].y = s.body[0].y.checked_sub(1)?;
            s.body[2].x = s.body[2].x.checked_add(1)?;
            s.body[2].y = s.body[2].y.checked_add(1)?;
            s.body[3].y = s.body[3].y.checked_add(2)?;
            s.direction = Direction::Top;
        }
    }

    Some(s)
}

fn s_form(direction: Direction) -> Shape {
    let x = 4;
    let y = 0;

    let body = match direction {
        Direction::Top | Direction::Bottom => [
            Position::new(x, y),
            Position::new(x, y + 1),
            Position::new(x + 1, y + 1),
            Position::new(x + 1, y + 2),
        ],
        Direction::Left | Direction::Right => [
            Position::new(x - 1, y + 1),
            Position::new(x, y + 1),
            Position::new(x, y),
            Position::new(x + 1, y),
        ],
    };

    Shape {
        shape_type: ShapeType::SForm,
        body,
        direction,
    }
}

fn s_form_turn_left(mut s: Shape) -> Option<Shape> {
    match s.direction {
        Direction::Top | Direction::Bottom => {
            s.body[0].y = s.body[0].y.checked_add(1)?;
            s.body[1].x = s.body[1].x.checked_add(1)?;
            s.body[2].y = s.body[2].y.checked_sub(1)?;
            s.body[3].x = s.body[3].x.checked_add(1)?;
            s.body[3].y = s.body[3].y.checked_sub(2)?;
            s.direction = Direction::Left;
        }
        Direction::Left | Direction::Right => {
            s.body[0].y = s.body[0].y.checked_sub(1)?;
            s.body[1].x = s.body[1].x.checked_sub(1)?;
            s.body[2].y = s.body[2].y.checked_add(1)?;
            s.body[3].x = s.body[3].x.checked_sub(1)?;
            s.body[3].y = s.body[3].y.checked_add(2)?;
            s.direction = Direction::Top;
        }
    }

    Some(s)
}

fn s_mirrored(direction: Direction) -> Shape {
    let x = 5;
    let y = 0;

    let body = match direction {
        Direction::Top | Direction::Bottom => [
            Position::new(x, y),
            Position::new(x, y + 1),
            Position::new(x - 1, y + 1),
            Position::new(x - 1, y + 2),
        ],
        Direction::Left | Direction::Right => [
            Position::new(x - 1, y),
            Position::new(x, y),
            Position::new(x, y + 1),
            Position::new(x + 1, y + 1),
        ],
    };

    Shape {
        shape_type: ShapeType::SMirrored,
        body,
        direction,
    }
}

fn s_mirrored_turn_left(mut s: Shape) -> Option<Shape> {
    match s.direction {
        Direction::Top | Direction::Bottom => {
            s.body[0].x = s.body[0].x.checked_sub(1)?;
            s.body[1].y = s.body[1].y.checked_sub(1)?;
            s.body[2].x = s.body[2].x.checked_add(1)?;
            s.body[3].x = s.body[3].x.checked_add(2)?;
            s.body[3].y = s.body[3].y.checked_sub(1)?;
            s.direction = Direction::Left;
        }
        Direction::Left | Direction::Right => {
            s.body[0].x = s.body[0].x.checked_add(1)?;
            s.body[1].y = s.body[1].y.checked_add(1)?;
            s.body[2].x = s.body[2].x.checked_sub(1)?;
            s.body[3].x = s.body[3].x.checked_sub(2)?;
            s.body[3].y = s.body[3].y.checked_add(1)?;
            s.direction = Direction::Top;
        }
    }

    Some(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quadrat_test() {
        let f = quadrat();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 0);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 1);

        let f = f.right().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 6);
        assert_eq!(f.body[1].y, 0);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 1);

        let f = f.left().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 0);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 1);

        let f = f.down().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 2);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 2);
    }

    #[test]
    fn line_test() {
        let f = line(Direction::Left);
        assert_eq!(f.body[0].x, 2);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 3);
        assert_eq!(f.body[1].y, 0);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 0);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 0);

        let f = line(Direction::Top);
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 3);

        let f = f.right().unwrap();
        assert_eq!(f.body[0].x, 6);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 6);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 6);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 3);

        let f = f.down().unwrap();
        assert_eq!(f.body[0].x, 6);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 6);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 6);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 4);

        let f = f.left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 4);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 3);
        assert_eq!(f.body[0].y, 3);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 3);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 3);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 4);
    }

    #[test]
    fn t_form_test() {
        let f = t_form(Direction::Left);
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 2);

        let f = t_form(Direction::Right);
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 1);

        let f = t_form(Direction::Bottom);
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 0);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 0);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 0);

        let f = t_form(Direction::Top);
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 1);

        let f = f.down().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 2);

        let f = f.right().unwrap();
        assert_eq!(f.body[0].x, 6);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 6);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 7);
        assert_eq!(f.body[3].y, 2);

        let f = f.left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 2);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 3);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 3);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 2);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 3);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 2);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 2);
    }

    #[test]
    fn l_form_test() {
        let f = l_form(Direction::Left);
        assert_eq!(f.body[0].x, 2);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 3);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 0);

        let f = l_form(Direction::Right);
        assert_eq!(f.body[0].x, 6);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 0);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 0);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 1);

        let f = l_form(Direction::Bottom);
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 2);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 0);

        assert_eq!(f.body[3].x, 3);
        assert_eq!(f.body[3].y, 0);

        let f = l_form(Direction::Top);
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 2);

        let f = f.down().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 3);

        let f = f.right().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 3);

        let f = f.left().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 3);

        // ___|
        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 3);
        assert_eq!(f.body[0].y, 2);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 1);

        // _
        //  |
        //  |
        //  |
        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 3);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 3);
        assert_eq!(f.body[3].y, 1);

        //  ___
        // |
        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 2);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 3);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 3);
        assert_eq!(f.body[3].y, 3);

        // |
        // |
        // |_
        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 3);
    }

    #[test]
    fn l_mirrored_test() {
        let f = l_mirrored(Direction::Left);
        assert_eq!(f.body[0].x, 3);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 0);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 0);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 1);

        let f = l_mirrored(Direction::Right);
        assert_eq!(f.body[0].x, 7);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 6);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 0);

        let f = l_mirrored(Direction::Bottom);
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 2);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 0);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 0);

        let f = l_mirrored(Direction::Top);
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 2);

        let f = f.down().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 3);

        let f = f.right().unwrap();
        assert_eq!(f.body[0].x, 6);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 6);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 6);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 3);

        let f = f.left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 3);

        // ___
        //   |
        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 2);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 6);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 3);

        //  _
        // |
        // |
        // |
        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 3);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 1);

        //  |
        //  –––
        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 6);
        assert_eq!(f.body[0].y, 2);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 1);

        //  |
        //  |
        // _|
        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 3);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 3);
    }

    #[test]
    fn s_form_test() {
        let f = s_form(Direction::Left);
        assert_eq!(f.body[0].x, 3);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 0);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 0);

        let f = s_form(Direction::Top);
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 2);

        let f = f.down().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 3);

        let f = f.right().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 6);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 3);

        let f = f.left().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 3);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 2);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 1);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 4);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 3);
    }

    #[test]
    fn s_mirrored_test() {
        let f = s_mirrored(Direction::Left);
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 0);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 1);

        let f = s_mirrored(Direction::Top);
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 0);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 1);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 2);

        let f = f.down().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 3);

        let f = f.right().unwrap();
        assert_eq!(f.body[0].x, 6);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 6);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 5);
        assert_eq!(f.body[3].y, 3);

        let f = f.left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 3);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 4);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 1);

        assert_eq!(f.body[2].x, 5);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 6);
        assert_eq!(f.body[3].y, 2);

        let f = f.turn_left().unwrap();
        assert_eq!(f.body[0].x, 5);
        assert_eq!(f.body[0].y, 1);

        assert_eq!(f.body[1].x, 5);
        assert_eq!(f.body[1].y, 2);

        assert_eq!(f.body[2].x, 4);
        assert_eq!(f.body[2].y, 2);

        assert_eq!(f.body[3].x, 4);
        assert_eq!(f.body[3].y, 3);
    }
}
