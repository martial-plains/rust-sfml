use derive_more::derive::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use sfml_sys::sfVector3f;

pub type Vextor3i = Vector3<isize>;
pub type Vector3f = Vector3<f32>;

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
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Vector3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}

impl From<sfVector3f> for Vector3f {
    fn from(sfVector3f { x, y, z }: sfVector3f) -> Self {
        Self { x, y, z }
    }
}
