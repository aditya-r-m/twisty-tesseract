use std::collections::{BTreeMap, VecDeque};

use wasm_bindgen::prelude::*;

const LEN_DIMENSIONS: usize = 4;
const LEN_FACE: usize = EDGES.len().pow(LEN_DIMENSIONS as u32 - 1);
const LEN: usize = LEN_FACE * 2 * LEN_DIMENSIONS;
const SPAN: isize = 200;
const OFFSET_SMALL: isize = 15;
const OFFSET_LARGE: isize = 45;
const EDGES: [isize; 4] = [-OFFSET_LARGE, -OFFSET_SMALL, OFFSET_SMALL, OFFSET_LARGE];
const RADIUS: isize = 7;
const ANIMATION_FRAMES: isize = 30;

const COLORS: [&str; 2 * LEN_DIMENSIONS] = [
    "RED", "GREEN", "BLUE", "CYAN", "MAGENTA", "YELLOW", "WHITE", "PURPLE",
];

#[wasm_bindgen]
pub struct Tesseract {
    points: [[isize; LEN_DIMENSIONS]; LEN],
    state: [usize; LEN],
    actions: BTreeMap<[usize; 4], [usize; LEN]>,
    pending_actions: VecDeque<[usize; 4]>,
    animation_step: isize,
}

#[wasm_bindgen]
impl Tesseract {
    pub fn new() -> Tesseract {
        let mut points = [[0isize; LEN_DIMENSIONS]; LEN];
        let mut p = 0usize;
        for a in 0..LEN_DIMENSIONS {
            for b in [-SPAN, SPAN] {
                for i in EDGES {
                    for j in EDGES {
                        for k in EDGES {
                            points[p][a] = b;
                            points[p][(a + 1) % LEN_DIMENSIONS] = i;
                            points[p][(a + 2) % LEN_DIMENSIONS] = j;
                            points[p][(a + 3) % LEN_DIMENSIONS] = k;
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
            for mut dimensions in 0..LEN_DIMENSIONS.pow(3) {
                let a = dimensions % LEN_DIMENSIONS;
                dimensions /= LEN_DIMENSIONS;
                let b = dimensions % LEN_DIMENSIONS;
                dimensions /= LEN_DIMENSIONS;
                let c = dimensions % LEN_DIMENSIONS;
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
            pending_actions: VecDeque::new(),
            animation_step: 0,
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
        let chars = s.chars().collect::<Vec<char>>();
        self.pending_actions.push_back([
            chars[0] as usize - '0' as usize,
            chars[1] as usize - 'w' as usize,
            chars[2] as usize - 'w' as usize,
            chars[3] as usize - 'w' as usize,
        ]);
    }

    pub fn tick(&mut self) {
        if self.pending_actions.is_empty() {
            return;
        }
        self.animation_step += 1;
        self.animation_step %= ANIMATION_FRAMES;
        if self.animation_step != 0 {
            return;
        }
        if let Some(action) = self.pending_actions.pop_front() {
            self.apply(action);
        }
    }

    pub fn project(&self, b: usize, s: isize) -> String {
        let mut projection = self
            .points
            .iter()
            .cloned()
            .enumerate()
            .map(|(p, mut point)| {
                if let Some(action_key) = self.pending_actions.get(0) {
                    if let Some(action) = self.actions.get(action_key) {
                        for i in 0..4 {
                            point[i] = ((ANIMATION_FRAMES - self.animation_step) * point[i]
                                + self.animation_step * self.points[action[p]][i])
                                / ANIMATION_FRAMES;
                        }
                    }
                }
                point
            })
            .zip(self.state)
            .filter(|(point, _)| point[b] * s < SPAN)
            .map(|(point, c)| {
                let x = (SPAN * point[(b + 1) % LEN_DIMENSIONS]) / (SPAN - s * point[b]);
                let y = (SPAN * point[(b + 2) % LEN_DIMENSIONS]) / (SPAN - s * point[b]);
                let z = (SPAN * point[(b + 3) % LEN_DIMENSIONS]) / (SPAN - s * point[b]);
                let xr0 = (4 * x - 3 * z) / 5;
                let zr0 = (3 * x + 4 * z) / 5;
                let zr1 = (4 * zr0 - 3 * y) / 5;
                let yr1 = (3 * zr0 + 4 * y) / 5;
                (zr1, yr1, xr0, c / LEN_FACE)
            })
            .filter(|&(z, _, _, _)| z < 8 * SPAN)
            .collect::<Vec<(isize, isize, isize, usize)>>();
        projection.sort();
        projection
            .into_iter()
            .map(|(z, y, x, color)| {
                format!(
                    "{x},{y},{},{}",
                    (8 * SPAN * RADIUS) / (8 * SPAN - z).abs(),
                    COLORS[color]
                )
            })
            .collect::<Vec<String>>()
            .join("|")
    }

    pub fn solve(&self) -> String {
        format!("WIP")
    }
}
