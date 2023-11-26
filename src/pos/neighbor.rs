//! 隣接ヘックスの相対距離参照支援
    
/// Y座標が奇数時の隣接ヘックスの相対距離
const NEIGH_Y_ODD: [super::HMapDist; 6] = [
    super::HMapDist([-1, -1]), 
    super::HMapDist([0, -1]), 
    super::HMapDist([-1, 0]), 
    super::HMapDist([1, 0]), 
    super::HMapDist([-1, 1]), 
    super::HMapDist([0, 1]), 
];

/// Y座標が偶数時の隣接ヘックスの相対距離
const NEIGH_Y_EVEN: [super::HMapDist; 6] = [    
    super::HMapDist([0, -1]), 
    super::HMapDist([1, -1]), 
    super::HMapDist([-1, 0]), 
    super::HMapDist([1, 0]), 
    super::HMapDist([0, 1]), 
    super::HMapDist([1, 1]), 
];

/// 隣接ヘックスの座標の計算
pub fn neigh_hex(
    center: &super::HMapPos, 
    size: &super::HMapSize, 
) -> [Option<super::HMapPos>; 6] {
    let arr = if *center.y() % 2 == 0 {
        NEIGH_Y_EVEN
    } else {
        NEIGH_Y_ODD
    };
    std::array::from_fn(
        |i| (*center + arr[i]).filter(size)
    )
}