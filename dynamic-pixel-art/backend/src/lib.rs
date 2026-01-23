mod math;

// enum TransformKind {
//     HeatWave,
// }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HeatHazeEngine {
    width: usize,
    height: usize,
    src_buffer: Vec<u8>,
    dst_buffer: Vec<u8>,
}

#[wasm_bindgen]
impl HeatHazeEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, input_pixels: &[u8]) -> Self {
        Self {
            width,
            height,
            src_buffer: input_pixels.to_vec(),
            dst_buffer: vec![0u8; input_pixels.len()],
        }
    }

    pub fn apply_effect(&mut self, time: f64, strength: f32) {
        math::apply_heat_haze_inplace(
            &self.src_buffer,
            &mut self.dst_buffer,
            self.width,
            self.height,
            time,
            strength,
        );
    }

    pub fn get_buffer_ptr(&self) -> *const u8 {
        self.dst_buffer.as_ptr()
    }
}
