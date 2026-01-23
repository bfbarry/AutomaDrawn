use ndarray::{ArrayBase, ArrayView3, Dim, ViewRepr};
use noise::{NoiseFn, Perlin};
use std::sync::LazyLock;
use wasm_bindgen::prelude::*;

static NOISE_GEN: LazyLock<Perlin> = LazyLock::new(|| Perlin::new(42));

#[wasm_bindgen]
pub fn apply_heat_haze_inplace(
    source: &[u8],
    output: &mut [u8], // Write directly into the engine's dst_buffer
    width: usize,
    height: usize,
    time: f64,
    strength: f32,
) {
    // We assume 4 channels (RGBA)
    let img_view = ArrayView3::from_shape((height, width, 4), source)
        .expect("Buffer size does not match dimensions");

    // let mut output = vec![0u8; source.len()];

    for y in 0..height {
        let vertical_factor = y as f32 / height as f32;
        for x in 0..width {
            let nx = NOISE_GEN.get([x as f64 * 0.05, y as f64 * 0.05, time]) as f32;
            let ny = NOISE_GEN.get([x as f64 * 0.05 + 100.0, y as f64 * 0.05, time]) as f32;

            let src_x = x as f32 + (nx * strength * vertical_factor);
            let src_y = y as f32 + (ny * strength * vertical_factor);

            let offset = (y * width + x) * 4;
            let sampled = sample_bilinear(&img_view, src_x, src_y);
            output[offset..offset + 4].copy_from_slice(&sampled);
        }
    }
}

/// Grabs a pixel value at fractional coordinates using bilinear interpolation
fn sample_bilinear(img: &ArrayBase<ViewRepr<&u8>, Dim<[usize; 3]>, u8>, x: f32, y: f32) -> Vec<u8> {
    let (h, w, c) = img.dim();

    // Clamp coordinates to image boundaries
    let x0 = x.floor().clamp(0.0, (w - 2) as f32) as usize;
    let x1 = x0 + 1;
    let y0 = y.floor().clamp(0.0, (h - 2) as f32) as usize;
    let y1 = y0 + 1;

    let dx = x - x0 as f32;
    let dy = y - y0 as f32;

    (0..c)
        .map(|channel| {
            let p00 = img[[y0, x0, channel]] as f32;
            let p10 = img[[y0, x1, channel]] as f32;
            let p01 = img[[y1, x0, channel]] as f32;
            let p11 = img[[y1, x1, channel]] as f32;

            // Interpolate horizontally, then vertically
            let top = p00 + dx * (p10 - p00);
            let bottom = p01 + dx * (p11 - p01);
            (top + dy * (bottom - top)) as u8
        })
        .collect()
}
