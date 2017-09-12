//! This module contains various functions to help rotate points.

use point::PipelinePoint;

/// Rotate a 2D point about the x-axis.
/// Theta can be any real-numbered value, but the period of rotation is 0..2pi.
pub fn x_axis_rotate(point: &mut PipelinePoint, theta: f32) {
  let y = point.y * theta.cos();
  point.y = y;
}

/// Rotate a 2D point about the y-axis.
/// Theta can be any real-numbered value, but the period of rotation is 0..2pi.
pub fn y_axis_rotate(point: &mut PipelinePoint, theta: f32) {
  let x = point.x * theta.cos();
  point.x = x;
}

/// Rotate a 2D point about the z-axis.
/// Theta can be any real-numbered value, but the period of rotation is 0..2pi.
pub fn z_axis_rotate(point: &mut PipelinePoint, theta: f32) {
  let x = point.x * theta.cos() - point.y * theta.sin();
  let y = point.y * theta.cos() + point.x * theta.sin();
  point.x = x;
  point.y = y;
}

#[cfg(test)]
mod test {
  use super::*;
  use expectest::prelude::*;
  use point::PipelinePoint;
  use std::f32::consts::PI;

  #[test]
  fn test_x_axis_rotate() {
    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    x_axis_rotate(&mut pt, 0.0);

    expect!(pt.x).to(be_close_to(1.0));
    expect!(pt.y).to(be_close_to(1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    x_axis_rotate(&mut pt, PI * 2.0);

    expect!(pt.x).to(be_close_to(1.0));
    expect!(pt.y).to(be_close_to(1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    x_axis_rotate(&mut pt, PI/2.0);

    expect!(pt.x).to(be_close_to(1.0));
    expect!(pt.y).to(be_close_to(0.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    x_axis_rotate(&mut pt, PI);

    expect!(pt.x).to(be_close_to(1.0));
    expect!(pt.y).to(be_close_to(-1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    x_axis_rotate(&mut pt, PI * 3.0 / 2.0);

    expect!(pt.x).to(be_close_to(1.0));
    expect!(pt.y).to(be_close_to(0.0));
  }

  #[test]
  fn test_y_axis_rotate() {
    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    y_axis_rotate(&mut pt, 0.0);

    expect!(pt.x).to(be_close_to(1.0));
    expect!(pt.y).to(be_close_to(1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    y_axis_rotate(&mut pt, PI * 2.0);

    expect!(pt.x).to(be_close_to(1.0));
    expect!(pt.y).to(be_close_to(1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    y_axis_rotate(&mut pt, PI/2.0);

    expect!(pt.x).to(be_close_to(0.0));
    expect!(pt.y).to(be_close_to(1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    y_axis_rotate(&mut pt, PI);

    expect!(pt.x).to(be_close_to(-1.0));
    expect!(pt.y).to(be_close_to(1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    y_axis_rotate(&mut pt, PI * 3.0 / 2.0);

    expect!(pt.x).to(be_close_to(0.0));
    expect!(pt.y).to(be_close_to(1.0));
  }

  #[test]
  fn test_z_axis_rotate() {
    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    z_axis_rotate(&mut pt, 0.0);

    expect!(pt.x).to(be_close_to(1.0));
    expect!(pt.y).to(be_close_to(1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    z_axis_rotate(&mut pt, PI * 2.0);

    expect!(pt.x).to(be_close_to(1.0));
    expect!(pt.y).to(be_close_to(1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    z_axis_rotate(&mut pt, PI/2.0);

    expect!(pt.x).to(be_close_to(-1.0));
    expect!(pt.y).to(be_close_to(1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    z_axis_rotate(&mut pt, PI);

    expect!(pt.x).to(be_close_to(-1.0));
    expect!(pt.y).to(be_close_to(-1.0));

    let mut pt = PipelinePoint::xy_binary(1.0, 1.0, true);
    z_axis_rotate(&mut pt, PI * 3.0 / 2.0);

    expect!(pt.x).to(be_close_to(1.0));
    expect!(pt.y).to(be_close_to(-1.0));
  }
}
