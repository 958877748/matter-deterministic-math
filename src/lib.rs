use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 { self.x }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 { self.y }

    #[wasm_bindgen]
    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }

    #[wasm_bindgen]
    pub fn scale(&self, k: f64) -> Vec2 {
        Vec2 { x: self.x * k, y: self.y * k }
    }

    // 可以继续加 dot/cross/magnitude/normalize ...
}