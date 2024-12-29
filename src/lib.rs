use std::collections::BTreeMap;

use wasm_bindgen::prelude::*;

const DIMENSIONS: [char; 4] = ['w', 'x', 'y', 'z'];
const SPAN: isize = 200;
const OFFSET_SMALL: isize = 15;
const OFFSET_LARGE: isize = 45;
const EDGES: [isize; 4] = [-OFFSET_LARGE, -OFFSET_SMALL, OFFSET_SMALL, OFFSET_LARGE];
const LEN_FACE: usize = EDGES.len().pow(DIMENSIONS.len() as u32 - 1);
const LEN: usize = LEN_FACE * 2 * DIMENSIONS.len();
const RADIUS: isize = 7;

const COLORS: [&str; 2 * DIMENSIONS.len()] = [
    "RED", "GREEN", "BLUE", "CYAN", "MAGENTA", "YELLOW", "WHITE", "PURPLE",
];

#[wasm_bindgen]
pub struct Tesseract {
    points: [[isize; DIMENSIONS.len()]; LEN],
    state: [usize; LEN],
    actions: BTreeMap<(bool, isize, isize), [usize; LEN]>,
}

#[wasm_bindgen]
impl Tesseract {
    pub fn new() -> Tesseract {
        let mut points = [[0isize; DIMENSIONS.len()]; LEN];
        let mut p = 0usize;
        for a in 0..DIMENSIONS.len() {
            for b in [-SPAN, SPAN] {
                for i in EDGES {
                    for j in EDGES {
                        for k in EDGES {
                            points[p][a] = b;
                            points[p][(a + 1) % DIMENSIONS.len()] = i;
                            points[p][(a + 2) % DIMENSIONS.len()] = j;
                            points[p][(a + 3) % DIMENSIONS.len()] = k;
                            p += 1;
                        }
                    }
                }
            }
        }
        let mut state = [0usize; LEN];
        for i in 0..LEN {
            state[i] = i;
        }
        let mut actions: BTreeMap<(bool, isize, isize), [usize; LEN]> = BTreeMap::new();
        for d in [-1isize, 1isize] {
            for a in 0..DIMENSIONS.len() as isize {
                for sign_a in [-1isize, 1isize] {
                    for b in 0..DIMENSIONS.len() as isize {
                        for sign_b in [-1isize, 1isize] {
                            let mut action = [0; LEN];
                            for i in 0..LEN {
                                action[i] = i;
                            }
                            actions.insert((d < 0, sign_a * a, sign_b * b), action);
                        }
                    }
                }
            }
        }
        Tesseract {
            points,
            state,
            actions,
        }
    }

    pub fn project(&self) -> String {
        let mut projection = self
            .points
            .iter()
            .zip(self.state)
            .filter(|(point, _)| point[0] < SPAN)
            .map(|(point, c)| {
                let x = (SPAN * point[1]) / (SPAN - point[0]);
                let y = (SPAN * point[2]) / (SPAN - point[0]);
                let z = (SPAN * point[3]) / (SPAN - point[0]);
                let xr0 = (4 * x - 3 * z) / 5;
                let zr0 = (3 * x + 4 * z) / 5;
                let zr1 = (4 * zr0 - 3 * y) / 5;
                let yr1 = (3 * zr0 + 4 * y) / 5;
                (zr1, yr1, xr0, c / LEN_FACE)
            })
            .collect::<Vec<(isize, isize, isize, usize)>>();
        projection.sort();
        projection
            .into_iter()
            .map(|(z, y, x, color)| {
                format!(
                    "{x},{y},{},{}",
                    (8 * SPAN * RADIUS) / (8 * SPAN - z),
                    COLORS[color]
                )
            })
            .collect::<Vec<String>>()
            .join("|")
    }
}
