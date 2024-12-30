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
    actions: BTreeMap<[usize; 4], [usize; LEN]>,
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
        let mut actions: BTreeMap<[usize; 4], [usize; LEN]> = BTreeMap::new();
        for flags in 0..4 {
            let shell = flags & 1 == 0;
            let negative = flags & 2 == 0;
            for mut dimensions in 0..DIMENSIONS.len().pow(3) {
                let a = dimensions % DIMENSIONS.len();
                dimensions /= DIMENSIONS.len();
                let b = dimensions % DIMENSIONS.len();
                dimensions /= DIMENSIONS.len();
                let c = dimensions % DIMENSIONS.len();
                if a == b || b == c || c == a {
                    continue;
                }
                let mut action = [0; LEN];
                for i in 0..LEN {
                    action[i] = i;
                    if points[i][a as usize] * if negative { -1 } else { 1 } > 0
                        && (shell
                            != (points[i][a as usize] * if negative { -1 } else { 1 }
                                > OFFSET_SMALL))
                    {
                        let mut point_j = points[i];
                        point_j[c] = points[i][b];
                        point_j[b] = -points[i][c];
                        action[i] = points.iter().position(|&point| point == point_j).unwrap();
                    }
                }
                actions.insert(
                    [
                        if negative { 0 } else { 2 } + if shell == negative { 1 } else { 0 },
                        a,
                        b,
                        c,
                    ],
                    action,
                );
            }
        }
        Tesseract {
            points,
            state,
            actions,
        }
    }

    fn apply(&mut self, key: [usize; 4]) {
        if let Some(action) = self.actions.get(&key) {
            let mut state = self.state.clone();
            for i in 0..LEN {
                state[action[i]] = self.state[i];
            }
            self.state = state;
        }
    }

    pub fn input(&mut self, s: String) {
        if s.len() != 4 {
            return;
        }
        let chars = s.chars().collect::<Vec<char>>();
        let layer = chars[0] as usize - '0' as usize;
        if let Some(a) = DIMENSIONS
            .iter()
            .position(|&d| d == chars[1].to_ascii_lowercase())
        {
            if let Some(b) = DIMENSIONS
                .iter()
                .position(|&d| d == chars[2].to_ascii_lowercase())
            {
                if let Some(c) = DIMENSIONS
                    .iter()
                    .position(|&d| d == chars[3].to_ascii_lowercase())
                {
                    self.apply([layer, a, b, c]);
                }
            }
        }
    }

    pub fn project(&self, b: usize, s: isize) -> String {
        let mut projection = self
            .points
            .iter()
            .zip(self.state)
            .filter(|(point, _)| point[b] * s < SPAN)
            .map(|(point, c)| {
                let x = (SPAN * point[(b + 1) % DIMENSIONS.len()]) / (SPAN - s * point[b]);
                let y = (SPAN * point[(b + 2) % DIMENSIONS.len()]) / (SPAN - s * point[b]);
                let z = (SPAN * point[(b + 3) % DIMENSIONS.len()]) / (SPAN - s * point[b]);
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
