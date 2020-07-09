use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    iter::Peekable,
    str::Chars,
};

/// 操作対象のキューブの面を表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    /// F 面
    Front(usize),

    /// S 面
    Standing,

    /// B 面
    Back(usize),

    /// L 面
    Left(usize),

    /// M 面
    Middle,

    /// R 面
    Right(usize),

    /// U 面
    Up(usize),

    /// E 面
    Equational,

    /// D 面
    Down(usize),

    /// X 軸
    X,

    /// Y 軸
    Y,

    /// Z 軸
    Z,
}

/// 操作の回転方向を表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    /// 時計回り
    Clockwise,

    /// 反時計回り (プライム回転)
    Counterclockwise,

    /// 180 度 (2 回転)
    Turnover,
}

/// 1 つの操作を表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Movement {
    /// 回転する対象
    pub target: Face,

    /// 回転する方向
    pub direction: Rotation,
}

/// 回転記号のエラーを表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementParseError {
    /// 不正な面表記
    InvalidFace(char),
}

impl Display for MovementParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            MovementParseError::InvalidFace(face) => write!(f, "Invalid face notation: {}", face),
        }
    }
}

impl Error for MovementParseError {}

/// 回転記号をパースして `Movement` を生成するイテレーター。
#[derive(Debug)]
pub struct Movements<'a> {
    rest_notation: Peekable<Chars<'a>>,
}

impl<'a> Movements<'a> {
    pub fn new(source: &'a str) -> Movements<'a> {
        Movements {
            rest_notation: source.chars().peekable(),
        }
    }

    fn skip_whitespaces(&mut self) {
        while self.rest_notation.peek().map(|c| c.is_whitespace()) == Some(true) {
            self.rest_notation.next();
        }
    }
}

impl<'a> Iterator for Movements<'a> {
    type Item = Result<Movement, MovementParseError>;

    fn next(&mut self) -> Option<Result<Movement, MovementParseError>> {
        self.skip_whitespaces();
        let (face, layers) = match self.rest_notation.next() {
            None => return None,
            Some(face) => match face {
                'F' | 'S' | 'B' | 'L' | 'M' | 'R' | 'U' | 'E' | 'D' => (face, 1),
                'f' | 'b' | 'l' | 'r' | 'u' | 'd' => (face.to_ascii_uppercase(), 2),
                'x' | 'y' | 'z' => (face, 0),
                _ => return Some(Err(MovementParseError::InvalidFace(face))),
            },
        };

        // 日本と WCA では 2 層回転に w を用いる
        let layers = if layers == 1 {
            self.skip_whitespaces();
            match self.rest_notation.peek() {
                Some('w') => {
                    self.rest_notation.next();
                    2
                }
                _ => 1,
            }
        } else {
            layers
        };

        self.skip_whitespaces();
        let direction = match self.rest_notation.peek() {
            Some('2') => {
                self.rest_notation.next();
                Rotation::Turnover
            }
            Some('\'') => {
                self.rest_notation.next();
                Rotation::Counterclockwise
            }
            _ => Rotation::Clockwise,
        };

        let target = match face {
            'x' => Face::X,
            'y' => Face::Y,
            'z' => Face::Z,
            'F' => Face::Front(layers),
            'S' => Face::Standing,
            'B' => Face::Back(layers),
            'L' => Face::Left(layers),
            'M' => Face::Middle,
            'R' => Face::Right(layers),
            'U' => Face::Up(layers),
            'E' => Face::Equational,
            'D' => Face::Down(layers),
            _ => unreachable!("Unrecognized face"),
        };

        Some(Ok(Movement { target, direction }))
    }
}
