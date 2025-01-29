use crate::utils::read_lines;

#[derive(Debug, Clone)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Robot {
        Robot { x, y, vx, vy }
    }
}

pub struct AocDay14 {
    robots: Vec<Robot>,
}

impl AocDay14 {
    pub fn new() -> AocDay14 {
        let input = read_lines("input/day14.txt");
        let robots = Self::get_robots(&input);
        AocDay14 { robots }
    }

    pub fn part1(&self) -> usize {
        let mut map = self.init_map();
        let mut robots = self.robots.clone();

        for _ in 0..100 {
            Self::teleport(&mut map, &mut robots);
        }

        Self::calc_quadrants(&map)
    }

    pub fn part2(&self) -> usize {
        let mut map = self.init_map();
        let mut robots = self.robots.clone();
        let mut seconds = 0;

        loop {
            seconds += 1;
            Self::teleport(&mut map, &mut robots);
            if Self::is_tree(&map) {
                break;
            }
        }

        seconds
    }

    fn init_map(&self) -> Vec<Vec<usize>> {
        let mut map = vec![vec![0; 101]; 103];
        for robot in self.robots.iter() {
            map[robot.y as usize][robot.x as usize] += 1;
        }
        map
    }

    fn teleport(map: &mut Vec<Vec<usize>>, robots: &mut Vec<Robot>) {
        let x_len = map[0].len() as i32;
        let y_len = map.len() as i32;

        for robot in robots.iter_mut() {
            map[robot.y as usize][robot.x as usize] -= 1;

            let new_x = (robot.x + robot.vx + x_len) % x_len;
            let new_y = (robot.y + robot.vy + y_len) % y_len;

            robot.x = new_x;
            robot.y = new_y;

            map[robot.y as usize][robot.x as usize] += 1;
        }
    }

    fn calc_quadrants(map: &Vec<Vec<usize>>) -> usize {
        let mid_x = map.len() / 2;
        let mid_y = map[0].len() / 2;

        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;

        for r in 0..mid_x {
            for c in 0..mid_y {
                top_left += map[r][c];
            }
            for c in mid_y + 1..map[0].len() {
                top_right += map[r][c];
            }
        }

        for r in mid_x + 1..map.len() {
            for c in 0..mid_y {
                bottom_left += map[r][c];
            }
            for c in mid_y + 1..map[0].len() {
                bottom_right += map[r][c];
            }
        }

        top_left * top_right * bottom_left * bottom_right
    }

    fn is_tree(map: &Vec<Vec<usize>>) -> bool {
        let width_list = vec![10, 8, 6, 4];

        for r in 4..map.len() {
            for c in 10..map[0].len() - 10 {
                if map[r][c] == 1 {
                    let mut found_all_widths = true;

                    for &width in &width_list {
                        let mut found = false;
                        for row in r - 4..=r {
                            let mut all_ones = true;
                            for idx in c - width..=c + width {
                                if map[row][idx] == 0 {
                                    all_ones = false;
                                    break;
                                }
                            }
                            if all_ones {
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            found_all_widths = false;
                            break;
                        }
                    }

                    if found_all_widths {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_robots(input: &Vec<String>) -> Vec<Robot> {
        let mut robots = Vec::new();

        for line in input {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let p = parts[0].split('=').collect::<Vec<_>>();
            let p = p[1].split(',').collect::<Vec<_>>();

            let x = p[0].parse::<i32>().unwrap();
            let y = p[1].parse::<i32>().unwrap();

            let v = parts[1].split('=').collect::<Vec<_>>();
            let v = v[1].split(',').collect::<Vec<_>>();

            let vx = v[0].parse::<i32>().unwrap();
            let vy = v[1].parse::<i32>().unwrap();

            robots.push(Robot::new(x, y, vx, vy));
        }

        robots
    }
}
