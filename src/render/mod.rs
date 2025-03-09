use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::{color::Color, math::Vector2};

pub fn rect(
    ctx: &CanvasRenderingContext2d,
    color: Color,
    rect_data: (f64, f64, f64, f64),
    width: Option<f64>,
) -> () {
    ctx.set_fill_style_str(&color.to_string());
    ctx.fill_rect(rect_data.0, rect_data.1, rect_data.2, rect_data.3);
}

pub fn circle(
    ctx: &CanvasRenderingContext2d,
    color: Color,
    circle_data: (f64, f64, f64),
    width: Option<f64>,
) -> () {
}

pub fn draw_image(ctx: &CanvasRenderingContext2d, position: Vector2) -> () {}

pub fn draw_image_complex(ctx: &CanvasRenderingContext2d) -> () {}
