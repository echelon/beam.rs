//! Basic usage of beam.rs

extern crate beam;

use beam::Scaler;

fn main() {
  let scaler = Scaler::<u16, i16>::new(0, 100, -10_000, 10_000);
  for i in 0..101 {
    if i % 10 != 0 { continue }
    println!("{} scaled to target range = {}", i, scaler.scale(i));
  }
}
