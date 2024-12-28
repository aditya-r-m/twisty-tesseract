use wasm_bindgen::prelude::*;

const DIMENSIONS: usize = 4;
const SIZE: usize = 3;
const LENF: usize = SIZE.pow(DIMENSIONS as u32 - 1);
const LEN: usize = LENF * 2 * DIMENSIONS;

const COLORS: [&str; 2 * DIMENSIONS] = [
    "RED", "GREEN", "BLUE", "CYAN", "MAGENTA", "PURPLE", "WHITE", "BLACK",
];

#[derive(Debug, Default, Clone, Copy)]
struct Point {
    color: usize,
    coordinates: [isize; DIMENSIONS],
}

#[wasm_bindgen]
pub struct Tesseract {
    points: [Point; LEN],
}

#[wasm_bindgen]
impl Tesseract {
    pub fn new() -> Tesseract {
        let mut points = [Point::default(); LEN];
        let mut color = 0usize;
        let mut p = 0usize;
        for a in 0..DIMENSIONS {
            for b in [-3isize, 3] {
                for i in [-1isize, 0, 1] {
                    for j in [-1isize, 0, 1] {
                        for k in [-1isize, 0, 1] {
                            points[p].color = color;
                            points[p].coordinates[a] = b;
                            points[p].coordinates[(a + 1) % DIMENSIONS] = i;
                            points[p].coordinates[(a + 2) % DIMENSIONS] = j;
                            points[p].coordinates[(a + 3) % DIMENSIONS] = k;
                            p += 1;
                        }
                    }
                }
                color += 1;
            }
        }
        Tesseract { points }
    }

    pub fn project(&self) -> String {
        self.points
            .iter()
            .filter(|point| point.coordinates[0] < 2)
            .map(|&Point { color, coordinates }| {
                let x = (200 * 100 * coordinates[1]) / (200 - 100 * coordinates[0]);
                let y = (200 * 100 * coordinates[2]) / (200 - 100 * coordinates[0]);
                let z = (200 * 100 * coordinates[3]) / (200 - 100 * coordinates[0]);
                format!("{x},{y},{z},{}", COLORS[color])
            })
            .collect::<Vec<String>>()
            .join("|")
    }
}
