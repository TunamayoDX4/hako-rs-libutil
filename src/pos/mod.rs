use std::num::NonZeroU32;
use std::ops::{
    Add, 
    Sub, 
    Mul, 
    Div, 
    AddAssign, 
    SubAssign, 
    MulAssign, 
    DivAssign, 
};

use serde::{Serialize, Deserialize};

pub mod neighbor;
pub mod artable;

/// 箱庭諸島面積型
/// 
/// 箱庭諸島のマップ面積を表現するためのドメイン固有型。
/// NonZeroU32整数で表現されたベクトル型です。
#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct HMapSize([NonZeroU32; 2]);
impl HMapSize {
    pub fn x(&self) -> &NonZeroU32 { &self.0[0] }
    pub fn x_mut(&mut self) -> &mut NonZeroU32 { &mut self.0[0] }
    pub fn y(&self) -> &NonZeroU32 { &self.0[1] }
    pub fn y_mut(&mut self) -> &mut NonZeroU32 { &mut self.0[1] }
    pub fn serial(&self) -> u64 { self.x().get() as u64 * self.y().get() as u64 }
}
impl From<[NonZeroU32; 2]> for HMapSize {
    fn from(value: [NonZeroU32; 2]) -> Self { Self(value) }
}
impl From<HMapSize> for [NonZeroU32; 2] {
    fn from(value: HMapSize) -> Self { [
        *value.x(), 
        *value.y(), 
    ]}
}
impl From<HMapSize> for [u32; 2] {
    fn from(value: HMapSize) -> Self { [
        value.x().get(), 
        value.y().get(), 
    ]}
}

/// 箱庭諸島座標型
/// 
/// 箱庭諸島の座標を表現するためのドメイン固有型。
/// u32整数で表現された座標型です。
#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct HMapPos([u32; 2]);
impl HMapPos {
    pub fn x(&self) -> &u32 { &self.0[0] }
    pub fn x_mut(&mut self) -> &mut u32 { &mut self.0[0] }
    pub fn y(&self) -> &u32 { &self.0[1] }
    pub fn y_mut(&mut self) -> &mut u32 { &mut self.0[1] }
    pub fn serial(
        &self, 
        size: &HMapSize, 
    ) -> u64 {
        *self.x() as u64 + *self.y() as u64 * size.x().get() as u64
    }
    pub fn check(
        &self, 
        size: &HMapSize, 
    ) -> bool {
        *self.x() < size.x().get() && *self.y() < size.y().get()
    }
    pub fn filter(
        self, 
        size: &HMapSize, 
    ) -> Option<Self> { if self.check(size) {
        Some(self)
    } else {
        None
    }}
}
impl From<[u32; 2]> for HMapPos {
    fn from(value: [u32; 2]) -> Self { Self(value) }
}
impl Sub<Self> for HMapPos {
    type Output = HMapDist;

    fn sub(self, rhs: Self) -> Self::Output { HMapDist([
        self.0[0] as i64 - rhs.0[0] as i64, 
        self.0[1] as i64 - rhs.0[1] as i64, 
    ]) }
}
impl Add<HMapDist> for HMapPos {
    type Output = Self;

    fn add(self, rhs: HMapDist) -> Self::Output { Self([
        (self.0[0] as i64 + rhs.0[0]) as u32, 
        (self.0[1] as i64 + rhs.0[1]) as u32,  
    ]) }
}
impl AddAssign<HMapDist> for HMapPos {
    fn add_assign(&mut self, rhs: HMapDist) {
        *self = *self + rhs
    }
}

/// 箱庭座標距離型
/// 
/// 箱庭諸島の座標間の距離を表現するためのドメイン固有型。
/// i64整数で表現されたベクトル型です。
#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct HMapDist([i64; 2]);
impl HMapDist {
    pub fn x(&self) -> &i64 { &self.0[0] }
    pub fn x_mut(&mut self) -> &mut i64 { &mut self.0[0] }
    pub fn y(&self) -> &i64 { &self.0[1] }
    pub fn y_mut(&mut self) -> &mut i64 { &mut self.0[1] }
}
impl From<[i64; 2]> for HMapDist {
    fn from(value: [i64; 2]) -> Self { Self(value) }
}
impl Add<Self> for HMapDist {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { Self([
        self.0[0] + rhs.0[0], 
        self.0[1] + rhs.0[1], 
    ])}
}
impl AddAssign<Self> for HMapDist {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl Sub<Self> for HMapDist {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { Self([
        self.0[0] - rhs.0[0], 
        self.0[1] - rhs.0[1], 
    ])}
}
impl SubAssign<Self> for HMapDist {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
impl Mul<i64> for HMapDist {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output { Self([
        self.0[0] * rhs, 
        self.0[1] * rhs, 
    ])}
}
impl MulAssign<i64> for HMapDist {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs
    }
}
impl Div<i64> for HMapDist {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output { Self([
        self.0[0] / rhs, 
        self.0[1] / rhs, 
    ])}
}
impl DivAssign<i64> for HMapDist {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs
    }
}