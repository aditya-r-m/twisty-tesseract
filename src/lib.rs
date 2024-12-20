use wasm_bindgen::prelude::*;

const DIMENSIONS: usize = 4;
const LEN_FACE: usize = 2usize.pow(DIMENSIONS as u32 - 1);
const LEN: usize = LEN_FACE * 2 * DIMENSIONS as usize;

#[wasm_bindgen]
struct Tesseract {
    colors: [u8; LEN],
}

#[wasm_bindgen]
impl Tesseract {
    pub fn new() -> Tesseract {
        let mut colors = [0u8; LEN];
        for i in 0..LEN {
            colors[i] = (i / LEN_FACE) as u8;
        }
        Tesseract { colors }
    }

    pub fn rotate(&mut self) {
        for i in 0..LEN - 1 {
            self.colors[i] = self.colors[i + 1];
        }
    }
}
