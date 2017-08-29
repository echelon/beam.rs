//! This module contains various algorithms for point and color transforms.

/// Scaler performs a coordinate transformation from a source range to a target
/// range.
#[allow(dead_code)]
pub struct Scaler<S, D> {
  min_src: S,
  max_src: S,
  min_dest: D,
  max_dest: D,
}

macro_rules! scaler_impl {
  ($src_t: ty, $dest_t: ty) => {
    impl Scaler<$src_t, $dest_t> {

      /// Construct a new Scaler.
      /// * `min_src` - minimum of the source range
      /// * `max_src` - maximum of the source range
      /// * `min_dest` - minimum of the output range
      /// * `max_dest` - maximum of the output range
      #[allow(dead_code)]
      pub fn new(min_src: $src_t, max_src: $src_t, min_dest: $dest_t,
                 max_dest: $dest_t) -> Scaler<$src_t, $dest_t> {
        Scaler {
          min_src: min_src,
          max_src: max_src,
          min_dest: min_dest,
          max_dest: max_dest,
        }
      }

      /// Scale a number from the source range to the target output dimensions.
      /// * `num` - the number to be scaled.
      #[inline]
      #[allow(dead_code)]
      pub fn scale(&self, num: $src_t) -> $dest_t {
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

scaler_impl!(u16, u16);
scaler_impl!(u16, u32);
scaler_impl!(u16, i16);
scaler_impl!(u16, i32);
scaler_impl!(u32, u16);
scaler_impl!(u32, u32);
scaler_impl!(u32, i16);
scaler_impl!(u32, i32);

scaler_impl!(i16, u16);
scaler_impl!(i16, u32);
scaler_impl!(i16, i16);
scaler_impl!(i16, i32);
scaler_impl!(i32, u16);
scaler_impl!(i32, u32);
scaler_impl!(i32, i16);
scaler_impl!(i32, i32);

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn spot_check() {
    let sc = Scaler::<u16, u16>::new(0u16, 100u16, 0u16, 255u16);

    assert_eq!(0, sc.scale(0));
    assert_eq!(127, sc.scale(50));
    assert_eq!(255, sc.scale(100));

    let sc = Scaler::<u16, i16>::new(0u16, 100u16, -100i16, 100i16);

    assert_eq!(-100, sc.scale(0));
    assert_eq!(0, sc.scale(50));
    assert_eq!(100, sc.scale(100));

    let sc = Scaler::<i16, u16>::new(0i16, 10i16, 0u16, 1000u16);

    assert_eq!(0, sc.scale(0));
    assert_eq!(200, sc.scale(2));
    assert_eq!(500, sc.scale(5));
    assert_eq!(900, sc.scale(9));
    assert_eq!(1000, sc.scale(10));
  }
}
