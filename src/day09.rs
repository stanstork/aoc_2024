use std::iter::repeat;

pub struct AocDay9 {
    disk_map: Vec<i32>,
}

impl AocDay9 {
    pub fn new(input: Vec<String>) -> AocDay9 {
        let mut disk_map = Vec::new();
        let mut id = 0;

        for (i, c) in input[0].chars().enumerate() {
            let size = c.to_digit(10).unwrap() as usize;
            let free_space = i % 2 != 0;

            disk_map.extend(repeat(if free_space { -1 } else { id }).take(size));

            if !free_space {
                id += 1;
            }
        }

        AocDay9 { disk_map }
    }

    pub fn part1(&self) -> i64 {
        let mut disk_map = self.disk_map.clone();
        let mut left = 0;
        let mut right = disk_map.len() - 1;

        while left < right {
            if disk_map[left] == -1 {
                disk_map.swap(left, right);
                right -= 1;
            } else {
                left += 1;
            }
        }

        Self::checksum(&disk_map)
    }

    pub fn part2(&self) -> i64 {
        let mut disk_map = self.disk_map.clone();
        let mut right = disk_map.len() - 1;

        while right > 0 {
            if disk_map[right] != -1 {
                let size = Self::get_block_size(&disk_map, right);
                let start = Self::get_free_block_start(&disk_map, size, right);

                if let Some(start) = start {
                    let block_end = right - size + 1;
                    for i in 0..size {
                        disk_map.swap(start + i, block_end + i);
                    }
                }

                right -= size;
            } else {
                right -= 1;
            }
        }

        Self::checksum(&disk_map)
    }

    fn checksum(disk_map: &[i32]) -> i64 {
        disk_map
            .iter()
            .enumerate()
            .filter(|(_, &value)| value != -1)
            .map(|(i, &value)| i as i64 * value as i64)
            .sum()
    }

    fn get_block_size(disk_map: &[i32], mut right: usize) -> usize {
        let current = disk_map[right];
        let mut size = 0;

        while right > 0 && disk_map[right] == current {
            size += 1;
            right -= 1;
        }

        size
    }

    fn get_free_block_start(disk_map: &[i32], size: usize, block_start: usize) -> Option<usize> {
        let mut current_size = 0;
        for (i, &value) in disk_map.iter().enumerate() {
            if value == -1 {
                current_size += 1;
                if current_size == size && i < block_start {
                    return Some(i - size + 1);
                }
            } else {
                current_size = 0;
            }
        }

        None
    }
}
