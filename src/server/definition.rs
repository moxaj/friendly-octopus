#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3<T>(pub T, pub T, pub T);

pub type Size = Vec3<u16>;

pub type Position = Vec3<i32>;

#[derive(Debug, Clone, Copy)]
pub struct WorldDefinition {
    pub size: Size,
    pub seed: [u8; 32],
}
