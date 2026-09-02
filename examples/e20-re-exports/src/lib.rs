//! # Art
//!
//! 一个用来建模艺术概念的代码库

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// RYB 颜色模型的三原色
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// RYB 模型的调和色
    #[derive(PartialEq, Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// 将两种等量的原色混合生成调合色
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        println!("{:?}, {:?}", c1, c2);
        SecondaryColor::Orange
    }
}