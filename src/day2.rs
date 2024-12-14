pub struct AocDay2 {
    reports: Vec<Vec<i32>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Order {
    Inc,
    Dec,
}

const MIN_DIFF: i32 = 1;
const MAX_DIFF: i32 = 3;

impl AocDay2 {
    pub fn new(input: Vec<String>) -> AocDay2 {
        let reports = Self::parse(&input);
        AocDay2 { reports }
    }

    pub fn part1(&self) -> i32 {
        self.reports
            .iter()
            .filter(|report| Self::is_safe_report(&report))
            .count() as i32
    }

    pub fn part2(&self) -> i32 {
        self.reports
            .iter()
            .filter(|report| {
                if Self::is_safe_report(&report) {
                    return true;
                }

                report.iter().enumerate().any(|(i, _)| {
                    let new_report = report
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, &x)| x)
                        .collect::<Vec<_>>();
                    Self::is_safe_report(&new_report)
                })
            })
            .count() as i32
    }

    fn is_safe_report(report: &Vec<i32>) -> bool {
        let order = Self::get_order(&report, 0, 1);
        report
            .iter()
            .skip(1)
            .enumerate()
            .all(|(i, _)| Self::is_safe(report, &order, i, i + 1))
    }

    fn get_order(report: &Vec<i32>, l: usize, r: usize) -> Order {
        if report[l] > report[r] {
            Order::Dec
        } else {
            Order::Inc
        }
    }

    fn is_safe(report: &Vec<i32>, order: &Order, left: usize, right: usize) -> bool {
        let new_order = Self::get_order(&report, left, right);
        if new_order != *order {
            return false;
        }
        let diff = (report[left] - report[right]).abs();
        diff >= MIN_DIFF && diff <= MAX_DIFF
    }

    fn parse(input: &Vec<String>) -> Vec<Vec<i32>> {
        input
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect()
    }
}
