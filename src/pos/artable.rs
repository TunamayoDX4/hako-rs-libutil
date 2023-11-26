//! 円環形の指定距離参照支援

use std::ops::Range;
    
/// 円環相対距離テーブル
pub struct AroundDistTbl {
    table: Vec<super::HMapDist>, 
    center_dist: u32, 
}
impl AroundDistTbl {
    /// 円環相対距離テーブルの初期化
    pub fn new() -> Self { Self {
        table: vec![[0, 0].into()], 
        center_dist: 0, 
    } }

    /// 円環相対距離テーブルの拡張
    pub fn extend(
        &mut self, 
        center_dist: u32, 
    ) -> bool {
        if center_dist <= self.center_dist { return false }
        for r in (self.center_dist + 1)..=center_dist {
            let r = r as i64;
            for y in -r..=r {
                let shift_x = y.abs() / 2;
                if y.abs() == r { for x in 0..=r { self.table.push([
                    shift_x + x - r, y
                ].into())}} else {
                    let space_x = r * 2 - y.abs();
                    self.table.push([shift_x - r, y].into());
                    self.table.push([shift_x + space_x - r, y].into());
                }
            }
        }
        self.center_dist = center_dist;
        true
    }

    /// 指定距離の座標を算出するためのイテレータの取得
    pub fn iter<'a>(
        &'a mut self, 
        range: impl std::ops::RangeBounds<u32>, 
        center: super::HMapPos, 
        size: super::HMapSize, 
    ) -> (u64, ADTWrap<'a>) {
        let range = match range.start_bound() {
            std::ops::Bound::Included(&s) => s, 
            std::ops::Bound::Excluded(&s) => s + 1, 
            std::ops::Bound::Unbounded => 0, 
        }..match range.end_bound() {
            std::ops::Bound::Included(&e) => e, 
            std::ops::Bound::Excluded(&e) => e.checked_sub(1)
                .unwrap_or(0), 
            std::ops::Bound::Unbounded => self.center_dist, 
        };
        if self.center_dist < range.end {
            if !self.extend(range.end) {
                unreachable!()
            }
        }
        let op_range = {
            let start = if range.start == 0 { 0 } else {
                1 + ((range.start as u64 - 1) * 6) / 2 * range.start as u64
            };
            let end = 1 + (range.end as u64 * 6) / 2 * (range.end as u64 + 1);
            end - start
        };
        (op_range, ADTWrap {
            adt: &self.table, 
            range, 
            center, 
            size, 
        })
    }
}

/// 円環相対距離テーブルの指定距離範囲内でのイテレータ用ラッパ
pub struct ADTWrap<'a> {
    adt: &'a [super::HMapDist], 
    range: Range<u32>, 
    center: super::HMapPos, 
    size: super::HMapSize, 
}
impl<'a> ADTWrap<'a> {
    /// イテレータの取得
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = (
        u32, super::HMapPos, 
    )> + 'b {
        self.range.clone()
            .into_iter()
            .map(|r| r)
            .map(|r| {
                let base = if r == 0 { 0 }
                else { 1 + ((r - 1) * 6) / 2 * r };
                let add = if r == 0 { 1 } else { r * 6 };
                (
                    r, 
                    (
                        base, 
                        base + add, 
                    )
                )
            })
            .map(move |(
                rad, 
                range, 
            )| {
                let iter = self.adt[range.0 as usize..range.1 as usize]
                    .iter()
                    .map(move |dist| if *self.center.y() % 2 == 1 { *dist }
                    else {
                        let mut dist = *dist;
                        if dist.y().abs() % 2 == 1 { *dist.x_mut() += 1; }
                        dist
                    })
                    .map(move |dist| (self.center + dist)
                        .filter(&self.size)
                    )
                    .flatten();
                (rad as u32, iter)
            })
            .flat_map(move |(
                rad, iter, 
            )| iter.map(move |pos| (rad, pos)))
    }
}