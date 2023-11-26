//! 箱庭諸島に使うユーティリティ的なのの実装

pub mod pos;

pub mod prelude {
    pub use crate::pos::{
        HMapSize, 
        HMapPos, 
        HMapDist, 
        neighbor::neigh_hex, 
        artable::AroundDistTbl, 
    };
}