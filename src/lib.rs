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
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    // 不再写 getter/setter，直接用字段即可
    #[wasm_bindgen]
    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }

    #[wasm_bindgen]
    pub fn scale(&self, k: f64) -> Vec2 {
        Vec2 { x: self.x * k, y: self.y * k }
    }
}