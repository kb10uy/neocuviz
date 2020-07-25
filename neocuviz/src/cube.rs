use crate::notation::{Face as MovementFace, Movement, Rotation as MovementRotation};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// キューブ操作のエラーを表す。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CubeError {
    /// 非対応の回転
    UndefinedMovement(Movement),
}

impl Display for CubeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            CubeError::UndefinedMovement(m) => write!(f, "Undefined movement: {:?}", m),
        }
    }
}

impl Error for CubeError {}

/// キューブの面を表す。
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CubeFace {
    /// F 面
    Front,

    /// B 面
    Back,

    /// L 面
    Left,

    /// R 面
    Right,

    /// U 面
    Up,

    /// D 面
    Down,
}

/// 仮想的なキューブを表す。
pub struct Cube {
    /// 辺ごとの分割数
    divisions: usize,

    /// 面回転時のインデックス対応
    face_transform: Box<[usize]>,

    /// 現在の面の状態
    faces: HashMap<CubeFace, Box<[CubeFace]>>,
}

impl Cube {
    pub fn new(divisions: usize) -> Cube {
        let mut face_transform_source = Vec::with_capacity(divisions * divisions);
        for i in 0..(divisions * divisions) {
            let (x1, y1) = (i % divisions, i / divisions);
            let (x2, y2) = (y1, divisions - x1 - 1);
            let i_90 = y2 * divisions + x2;
            face_transform_source.push(i_90);
        }

        let make_face = |f| {
            (0..(divisions.pow(2)))
                .map(|_| f)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        };
        let mut faces = HashMap::with_capacity(6);
        faces.insert(CubeFace::Front, make_face(CubeFace::Front));
        faces.insert(CubeFace::Back, make_face(CubeFace::Back));
        faces.insert(CubeFace::Left, make_face(CubeFace::Left));
        faces.insert(CubeFace::Right, make_face(CubeFace::Right));
        faces.insert(CubeFace::Up, make_face(CubeFace::Up));
        faces.insert(CubeFace::Down, make_face(CubeFace::Down));

        Cube {
            face_transform: face_transform_source.into_boxed_slice(),
            faces,
            divisions,
        }
    }

    pub fn divisions(&self) -> usize {
        self.divisions
    }

    pub fn faces(&self) -> &HashMap<CubeFace, Box<[CubeFace]>> {
        &self.faces
    }

    /// 回転操作を適用する。
    pub fn apply(&mut self, movement: Movement) -> Result<(), CubeError> {
        let count = match movement.direction {
            MovementRotation::Clockwise => 1,
            MovementRotation::Turnover => 2,
            MovementRotation::Counterclockwise => 3,
        };

        match movement.target {
            // 通常回転
            MovementFace::Front(l) => {
                self.turn_face(CubeFace::Front, count);
                for i in 0..l {
                    self.turn_layer_z(self.divisions - 1 - i, count);
                }
            }
            MovementFace::Back(l) => {
                self.turn_face(CubeFace::Back, count);
                for i in 0..l {
                    self.turn_layer_z(i, 4 - count);
                }
            }
            MovementFace::Left(l) => {
                self.turn_face(CubeFace::Left, count);
                for i in 0..l {
                    self.turn_layer_x(i, 4 - count);
                }
            }
            MovementFace::Right(l) => {
                self.turn_face(CubeFace::Right, count);
                for i in 0..l {
                    self.turn_layer_x(self.divisions - 1 - i, count);
                }
            }
            MovementFace::Up(l) => {
                self.turn_face(CubeFace::Up, count);
                for i in 0..l {
                    self.turn_layer_y(i, count);
                }
            }
            MovementFace::Down(l) => {
                self.turn_face(CubeFace::Down, count);
                for i in 0..l {
                    self.turn_layer_y(self.divisions - 1 - i, 4 - count);
                }
            }

            // 中層回転
            // TODO これでいいの？
            MovementFace::Standing => {
                if self.divisions != 3 {
                    return Err(CubeError::UndefinedMovement(movement));
                }
                self.turn_layer_z(1, count);
            }
            MovementFace::Middle => {
                if self.divisions != 3 {
                    return Err(CubeError::UndefinedMovement(movement));
                }
                self.turn_layer_x(1, 4 - count);
            }
            MovementFace::Equational => {
                if self.divisions != 3 {
                    return Err(CubeError::UndefinedMovement(movement));
                }
                self.turn_layer_y(1, count);
            }

            // 全体回転
            MovementFace::X => {
                self.turn_face(CubeFace::Right, count);
                self.turn_face(CubeFace::Left, 4 - count);
                for i in 0..(self.divisions) {
                    self.turn_layer_x(i, count);
                }
            }
            MovementFace::Y => {
                self.turn_face(CubeFace::Up, count);
                self.turn_face(CubeFace::Down, 4 - count);
                for i in 0..(self.divisions) {
                    self.turn_layer_y(i, count);
                }
            }
            MovementFace::Z => {
                self.turn_face(CubeFace::Front, count);
                self.turn_face(CubeFace::Back, 4 - count);
                for i in 0..(self.divisions) {
                    self.turn_layer_z(i, count);
                }
            }
        }
        Ok(())
    }

    /// 面のみ回転する。
    ///
    /// * `face`: 対象の面
    /// * `count`: 時計回りに 90 度単位で回転する回数。
    fn turn_face(&mut self, face: CubeFace, count: usize) {
        let mut modified = self
            .faces
            .get(&face)
            .expect("Existing faces should be registered")
            .clone();
        for _ in 0..count {
            let next = self
                .face_transform
                .iter()
                .map(|&tr| modified[tr])
                .collect::<Vec<_>>()
                .into_boxed_slice();
            modified = next;
        }

        self.faces.insert(face, modified);
    }

    /// 層のみを X 軸にそって回転する。
    /// 回転方向は R 面時計回り。
    ///
    /// * `column_front`: F 面における列位置
    /// * `count`: 回転回数
    fn turn_layer_x(&mut self, column_front: usize, count: usize) {
        for _ in 0..count {
            let front_column = self.extract_column(CubeFace::Front, column_front);
            let up_column = self.swap_to_column(CubeFace::Up, column_front, &front_column, false);
            let back_column = self.swap_to_column(
                CubeFace::Back,
                self.divisions - 1 - column_front,
                &up_column,
                true,
            );
            let down_column = self.swap_to_column(CubeFace::Down, column_front, &back_column, true);
            self.swap_to_column(CubeFace::Front, column_front, &down_column, false);
        }
    }

    /// 層のみを Y 軸にそって回転する。
    /// 回転方向は U 面時計回り。
    ///
    /// * `row_right`: R 面における行位置
    /// * `count`: 回転回数
    fn turn_layer_y(&mut self, row_right: usize, count: usize) {
        for _ in 0..count {
            let right_row = self.extract_row(CubeFace::Right, row_right);
            let front_row = self.swap_to_row(CubeFace::Front, row_right, &right_row, false);
            let left_row = self.swap_to_row(CubeFace::Left, row_right, &front_row, false);
            let back_row = self.swap_to_row(CubeFace::Back, row_right, &left_row, false);
            self.swap_to_row(CubeFace::Right, row_right, &back_row, false);
        }
    }

    /// 層のみを Z 軸にそって回転する。
    /// 回転方向は F 面時計回り。
    ///
    /// * `column_front`: U 面における行位置
    /// * `count`: 回転回数
    fn turn_layer_z(&mut self, row_up: usize, count: usize) {
        for _ in 0..count {
            let up_row = self.extract_row(CubeFace::Up, row_up);
            let right_column =
                self.swap_to_column(CubeFace::Right, self.divisions - 1 - row_up, &up_row, false);
            let down_row = self.swap_to_row(
                CubeFace::Down,
                self.divisions - 1 - row_up,
                &right_column,
                true,
            );
            let left_row = self.swap_to_column(CubeFace::Left, row_up, &down_row, false);
            self.swap_to_row(CubeFace::Up, row_up, &left_row, true);
        }
    }

    /// 行を抽出する。
    fn extract_row(&self, face: CubeFace, row: usize) -> Box<[CubeFace]> {
        self.faces
            .get(&face)
            .expect("Existing faces should be registered")
            .iter()
            .skip(row * self.divisions)
            .take(self.divisions)
            .copied()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// 列を抽出する。
    fn extract_column(&self, face: CubeFace, column: usize) -> Box<[CubeFace]> {
        let divisions = self.divisions;
        self.faces
            .get(&face)
            .expect("Existing faces should be registered")
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| {
                if i % divisions == column {
                    Some(v)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// 特定の面の行と入れ替える。
    fn swap_to_row(
        &mut self,
        target_face: CubeFace,
        target_row: usize,
        values: &[CubeFace],
        invert: bool,
    ) -> Box<[CubeFace]> {
        assert_eq!(values.len(), self.divisions);
        let popped = self.extract_row(target_face, target_row);
        let face = self
            .faces
            .get_mut(&target_face)
            .expect("Existing faces should be registered");

        if invert {
            for (i, &v) in values.iter().rev().enumerate() {
                face[target_row * self.divisions + i] = v;
            }
        } else {
            for (i, &v) in values.iter().enumerate() {
                face[target_row * self.divisions + i] = v;
            }
        }

        popped
    }

    /// 特定の面の列と入れ替える。
    fn swap_to_column(
        &mut self,
        target_face: CubeFace,
        target_column: usize,
        values: &[CubeFace],
        invert: bool,
    ) -> Box<[CubeFace]> {
        assert_eq!(values.len(), self.divisions);
        let popped = self.extract_column(target_face, target_column);
        let face = self
            .faces
            .get_mut(&target_face)
            .expect("Existing faces should be registered");

        if invert {
            for (i, &v) in values.iter().rev().enumerate() {
                face[target_column + self.divisions * i] = v;
            }
        } else {
            for (i, &v) in values.iter().enumerate() {
                face[target_column + self.divisions * i] = v;
            }
        }

        popped
    }
}
