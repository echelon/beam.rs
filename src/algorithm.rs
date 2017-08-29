//! This module contains various algorithms for point and color transforms.

use std::ops::Add;

/// A coordinate transformation.
struct CoordinateTransform <S, D> where S: Add, D: Add{
  pub min_src_x: S,
  pub max_src_x: S,
  pub min_src_y: S,
  pub max_src_y: S,
  pub min_dest_x: D,
  pub max_dest_x: D,
  pub min_dest_y: D,
  pub max_dest_y: D,
}

/// A trait for numbers that can be scaled.
trait Scalable : Sized + Add<Self, Output=Self> {

  /// Scale a number to a percentage of a range.
  /// Return values are from 0.0 to 1.0, inclusive.
  fn scale_to_range(&self, minimum: Self, maximum: Self) -> f32;
}

macro_rules! scalable_impl {
  ($trait_name: ident, $t: ty) => {
    impl $trait_name for $t {

      #[inline]
      fn scale_to_range(&self, minimum: $t, maximum: $t) -> f32 {
        let numerator = self.saturating_sub(minimum);
        let range = maximum.saturating_sub(minimum);
        numerator as f32 / range as f32
      }
    }
  }
}

scalable_impl!(Scalable, u8);
scalable_impl!(Scalable, u16);
scalable_impl!(Scalable, u32);
scalable_impl!(Scalable, u64);
scalable_impl!(Scalable, usize);

scalable_impl!(Scalable, i8);
scalable_impl!(Scalable, i16);
scalable_impl!(Scalable, i32);
scalable_impl!(Scalable, i64);
scalable_impl!(Scalable, isize);




/*fn map_x_point(image_position: u32, image_scale: u32) -> i16 {
  let num = image_position as f64;
  let denom = image_scale as f64;
  let ratio = num / denom;
  let scale = x_max.saturating_sub(x_min) as f64;
  let result = ratio * scale * -1.0;
  let result = result as i16;

  result.saturating_add(self.x_offset)
}*/


/*impl <S,D> CoordinateTransform<S, D> where S: Add, D: Add<Output=D> {
  pub fn transform(self, x: S) -> D {
    x + self.min_dest_x
  }
}*/

