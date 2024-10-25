///Adds one to the number given.
/// 
/// # Examples
/// 
///``` 
/// let arg=5;
/// let answer=my_crates::add_one(arg);
/// 
/// assert_eq!(6,answer);
/// ```
pub use art::utils::mix;
pub use art::kinds::PrimaryColors;
pub use art::kinds::SecondaryColors;
pub mod art{
    pub mod kinds{
        pub enum PrimaryColors{
            Red,
            Blue,
            Green,
        }
        pub enum SecondaryColors{
            Orange,
            Purple,
            Green, 
        }
    }
    pub mod utils{
        use crate::art::kinds::*;
        ///Takes two primary colors and returns a secondary color.
        pub fn mix(color1:PrimaryColors,color2:PrimaryColors)->SecondaryColors{
            SecondaryColors::Purple
        }
        
    }
    
}
