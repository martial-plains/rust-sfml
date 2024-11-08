use derive_more::derive::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use sfml_sys::{sfVector2f, sfVector2i, sfVector2u};

pub type Vector2i = Vector2<i32>;
pub type Vector2u = Vector2<u32>;
pub type Vector2f = Vector2<f32>;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Sub,
    AddAssign,
    SubAssign,
    Add,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl From<sfVector2f> for Vector2f {
    fn from(sfVector2f { x, y }: sfVector2f) -> Self {
        Self { x, y }
    }
}

impl From<sfVector2i> for Vector2i {
    fn from(sfVector2i { x, y }: sfVector2i) -> Self {
        Self { x, y }
    }
}

impl From<sfVector2u> for Vector2u {
    fn from(sfVector2u { x, y }: sfVector2u) -> Self {
        Self { x, y }
    }
}
