#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3<T>(pub T, pub T, pub T);

pub type Vec3i = Vec3<i32>;

fn quot_rem_i(n: i32, m: i32) -> (i32, i32) {
    let quot = n / m;
    let rem = n % m;
    if rem >= 0 {
        (quot, rem)
    } else {
        (quot - 1, rem + m)
    }
}

pub fn quot_rem_vec3i(n: &Vec3i, m: &Vec3i) -> (Vec3i, Vec3i) {
    let (quot1, rem1) = quot_rem_i(n.0, m.0);
    let (quot2, rem2) = quot_rem_i(n.1, m.1);
    let (quot3, rem3) = quot_rem_i(n.2, m.2);
    (Vec3(quot1, quot2, quot3), Vec3(rem1, rem2, rem3))
}

pub fn linear_index(block_position: &Vec3i, chunk_size: &Vec3i) -> usize {
    (block_position.0
        + block_position.1 * chunk_size.0
        + block_position.2 * (chunk_size.0 + chunk_size.1)) as usize
}