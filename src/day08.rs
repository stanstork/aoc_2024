use crate::{utils::read_matrix, AocDay};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Antenna {
    positions: Vec<(usize, usize)>,
}

pub struct AocDay8 {
    map: Vec<Vec<char>>,
    antennas: HashMap<char, Antenna>,
}

impl AocDay8 {
    pub fn new() -> Self {
        let matrix = read_matrix("input/day8.txt");
        let antennas = Self::get_antennas(&matrix);
        AocDay8 {
            map: matrix,
            antennas,
        }
    }

    pub fn part1(&self) -> usize {
        let mut map = self.map.clone();

        for antenna in self.antennas.values() {
            for &start in &antenna.positions {
                for &end in &antenna.positions {
                    if start == end {
                        continue;
                    }

                    let dx = start.0 as i32 - end.0 as i32;
                    let dy = start.1 as i32 - end.1 as i32;

                    let new_x1 = start.0 as i32 + dx;
                    let new_y1 = start.1 as i32 + dy;

                    let new_x2 = end.0 as i32 - dx;
                    let new_y2 = end.1 as i32 - dy;

                    Self::place_single_antidote(&mut map, new_x1, new_y1);
                    Self::place_single_antidote(&mut map, new_x2, new_y2);
                }
            }
        }

        map.iter().flatten().filter(|&&c| c == '#').count()
    }

    pub fn part2(&self) -> usize {
        let mut map = self.map.clone();

        for antenna in self.antennas.values() {
            for &start in &antenna.positions {
                for &end in &antenna.positions {
                    if start == end {
                        continue;
                    }

                    let dx = start.0 as i32 - end.0 as i32;
                    let dy = start.1 as i32 - end.1 as i32;

                    self.place_antidote(&mut map, start.0 as i32, start.1 as i32, dx, dy);
                    self.place_antidote(&mut map, end.0 as i32, end.1 as i32, -dx, -dy);
                }
            }
        }

        let antennas_count = self
            .antennas
            .values()
            .flat_map(|a| a.positions.clone())
            .count();

        map.iter().flatten().filter(|&&c| c == '#').count() + antennas_count
    }

    fn get_antennas(map: &[Vec<char>]) -> HashMap<char, Antenna> {
        let mut antennas: HashMap<char, Antenna> = HashMap::new();

        for (r, row) in map.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell != '.' {
                    antennas
                        .entry(cell)
                        .or_insert_with(|| Antenna {
                            positions: Vec::new(),
                        })
                        .positions
                        .push((c, r));
                }
            }
        }
        antennas
    }

    fn place_single_antidote(map: &mut [Vec<char>], x: i32, y: i32) {
        if x >= 0 && y >= 0 && y < map.len() as i32 && x < map[0].len() as i32 {
            map[y as usize][x as usize] = '#';
        }
    }

    fn place_antidote(&self, map: &mut [Vec<char>], x: i32, y: i32, dx: i32, dy: i32) {
        let nx = x + dx;
        let ny = y + dy;

        if nx >= 0 && ny >= 0 && ny < map.len() as i32 && nx < map[0].len() as i32 {
            let current = map[ny as usize][nx as usize];

            if !self.antennas.contains_key(&current) {
                map[ny as usize][nx as usize] = '#';
            }
            self.place_antidote(map, nx, ny, dx, dy);
        }
    }
}

impl AocDay for AocDay8 {
    fn part1(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part1())
    }

    fn part2(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part2())
    }
}
