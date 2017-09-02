//! This module contains various algorithms for point and color transforms.

/// A coordinate transformation.
struct CoordinateTransformer<S, D> {
  pub min_src: S,
  pub max_src: S,
  pub min_dest: D,
  pub max_dest: D,
}

macro_rules! coordinate_impl {
  ($src_t: ty, $dest_t: ty) => {
    impl CoordinateTransformer<$src_t, $dest_t> {

      /// Construct a new CoordinateTransformer.
      pub fn new(min_src: $src_t, max_src: $src_t, min_dest: $dest_t,
                 max_dest: $dest_t) -> CoordinateTransformer<$src_t, $dest_t> {

        CoordinateTransformer {
          min_src: min_src,
          max_src: max_src,
          min_dest: min_dest,
          max_dest: max_dest,
        }
      }

      #[inline]
      pub fn scale_number(&self, num: $src_t) -> $dest_t {
        let numerator = num.saturating_sub(self.min_src);
        let denominator = self.max_src.saturating_sub(self.min_src);
        let normalized = numerator as f32 / denominator as f32;
        let scale = self.max_dest.saturating_sub(self.min_dest) as f32;

        let scaled = (normalized * scale) as $dest_t;
        scaled.saturating_add(self.min_dest)
      }
    }
  }
}

// NOTE: This is somewhat messy.
// There has to be a cleaner way of doing this.
// Only implemented for the widths used in laser projection.

coordinate_impl!(u16, u16);
coordinate_impl!(u16, u32);
coordinate_impl!(u16, i16);
coordinate_impl!(u16, i32);
coordinate_impl!(u32, u16);
coordinate_impl!(u32, u32);
coordinate_impl!(u32, i16);
coordinate_impl!(u32, i32);

coordinate_impl!(i16, u16);
coordinate_impl!(i16, u32);
coordinate_impl!(i16, i16);
coordinate_impl!(i16, i32);
coordinate_impl!(i32, u16);
coordinate_impl!(i32, u32);
coordinate_impl!(i32, i16);
coordinate_impl!(i32, i32);

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn spot_check() {
    let mut ct =
      CoordinateTransformer::<u16, u16>::new(0u16, 100u16, 0u16, 255u16);

    assert_eq!(0, ct.scale_number(0));
    assert_eq!(127, ct.scale_number(50));
    assert_eq!(255, ct.scale_number(100));

    let mut ct =
      CoordinateTransformer::<u16, i16>::new(0u16, 100u16, -100i16, 100i16);

    assert_eq!(-100, ct.scale_number(0));
    assert_eq!(0, ct.scale_number(50));
    assert_eq!(100, ct.scale_number(100));

    let mut ct =
      CoordinateTransformer::<i16, u16>::new(0i16, 10i16, 0u16, 1000u16);

    assert_eq!(0, ct.scale_number(0));
    assert_eq!(200, ct.scale_number(2));
    assert_eq!(500, ct.scale_number(5));
    assert_eq!(900, ct.scale_number(9));
    assert_eq!(1000, ct.scale_number(10));
  }
}
