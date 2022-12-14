use glam::{ivec2, IVec2};

use itertools::Itertools;

pub fn cartesian_inclusive(x: IVec2, y: IVec2) -> impl Iterator<Item = IVec2> {
    let min = x.min(y);
    let max = x.max(y);
    (min.x..=max.x)
        .cartesian_product(min.y..=max.y)
        .map(|(x, y)| ivec2(x, y))
}

pub fn cartesian_exclusive(x: IVec2, y: IVec2) -> impl Iterator<Item = IVec2> {
    let min = x.min(y);
    let max = x.max(y);
    (min.x..max.x)
        .cartesian_product(min.y..max.y)
        .map(|(x, y)| ivec2(x, y))
}
