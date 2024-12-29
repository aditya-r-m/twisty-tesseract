use std::collections::BTreeMap;

use wasm_bindgen::prelude::*;

const DIMENSIONS: [char; 4] = ['w', 'x', 'y', 'z'];
const EDGES: [isize; 4] = [-45, -15, 15, 45];
const SPAN: isize = 200;
const LEN_POINTS: usize = EDGES.len().pow(DIMENSIONS.len() as u32 - 1) * 2 * DIMENSIONS.len();
const RADIUS: isize = 7;

const COLORS: [&str; 2 * DIMENSIONS.len()] = [
    "RED", "GREEN", "BLUE", "CYAN", "MAGENTA", "YELLOW", "WHITE", "PURPLE",
];

#[derive(Debug, Default, Clone, Copy)]
struct Point {
    color: usize,
    coordinates: [isize; DIMENSIONS.len()],
}

struct Action {
    permutation: [usize; LEN_POINTS],
}

#[wasm_bindgen]
pub struct Tesseract {
    points: [Point; LEN_POINTS],
    actions: BTreeMap<(bool, isize, isize), Action>,
}

#[wasm_bindgen]
impl Tesseract {
    pub fn new() -> Tesseract {
        let mut points = [Point::default(); LEN_POINTS];
        let mut color = 0usize;
        let mut p = 0usize;
        for a in 0..DIMENSIONS.len() {
            for b in [-SPAN, SPAN] {
                for i in EDGES {
                    for j in EDGES {
                        for k in EDGES {
                            points[p].color = color;
                            points[p].coordinates[a] = b;
                            points[p].coordinates[(a + 1) % DIMENSIONS.len()] = i;
                            points[p].coordinates[(a + 2) % DIMENSIONS.len()] = j;
                            points[p].coordinates[(a + 3) % DIMENSIONS.len()] = k;
                            p += 1;
                        }
                    }
                }
                color += 1;
            }
        }
        let mut actions: BTreeMap<(bool, isize, isize), Action> = BTreeMap::new();
        Tesseract { points, actions }
    }

    pub fn project(&self) -> String {
        let mut projection = self
            .points
            .iter()
            .filter(|point| point.coordinates[0] < SPAN)
            .map(|&Point { color, coordinates }| {
                let x = (SPAN * coordinates[1]) / (SPAN - coordinates[0]);
                let y = (SPAN * coordinates[2]) / (SPAN - coordinates[0]);
                let z = (SPAN * coordinates[3]) / (SPAN - coordinates[0]);
                let xr0 = (4 * x - 3 * z) / 5;
                let zr0 = (3 * x + 4 * z) / 5;
                let zr1 = (4 * zr0 - 3 * y) / 5;
                let yr1 = (3 * zr0 + 4 * y) / 5;
                (zr1, yr1, xr0, color)
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
