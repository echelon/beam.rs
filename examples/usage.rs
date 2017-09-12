//! Basic usage of beam.rs

extern crate beam;

use beam::PipelinePoint;
use beam::rotate::x_axis_rotate;
use beam::scale::Scaler;

fn main() {
  scale();
  rotate();
}

fn scale() {
  let scaler = Scaler::<u16, i16>::new(0, 100, -10_000, 10_000);
  for i in 0..101 {
    if i % 10 != 0 { continue }
    println!("{} scaled to target range = {}", i, scaler.scale(i));
  }
}

fn rotate() {
  let mut point = PipelinePoint::xy_binary(1.0, 1.0, true);
  x_axis_rotate(&mut point, 0.75);

  println!("Point: {}", point);
}
