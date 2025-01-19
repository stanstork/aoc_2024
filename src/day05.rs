use std::collections::{HashMap, HashSet};

trait RuleMapTrait {
    fn insert_rule(&mut self, key: i32, val: i32);
}

impl RuleMapTrait for HashMap<i32, Rule> {
    fn insert_rule(&mut self, key: i32, val: i32) {
        if let Some(rule) = self.get_mut(&key) {
            rule.after.insert(val);
            if let Some(other_rule) = self.get_mut(&val) {
                other_rule.before.insert(key);
            } else {
                let mut rule = Rule::new();
                rule.before.insert(key);
            }
        } else {
            let mut rule = Rule::new();
            rule.after.insert(val);
            self.insert(key, rule);
        }
    }
}

struct Rule {
    before: HashSet<i32>,
    after: HashSet<i32>,
}

impl Rule {
    fn new() -> Rule {
        Rule {
            before: HashSet::new(),
            after: HashSet::new(),
        }
    }
}

pub struct AocDay5 {
    rules: HashMap<i32, Rule>,
    pages: Vec<Vec<i32>>,
}

impl AocDay5 {
    pub fn new(input: Vec<String>) -> AocDay5 {
        let rules = Self::get_rules(&input);
        let pages = Self::get_pages(&input);

        AocDay5 { rules, pages }
    }

    pub fn part1(&self) -> i32 {
        let (valid_pages, _) = self.categorize_pages();
        valid_pages
            .iter()
            .fold(0, |acc, page| acc + page[page.len() / 2])
    }

    pub fn part2(&self) -> i32 {
        let (_, invalid_pages) = self.categorize_pages();
        let mut total_sum = 0;

        for page in invalid_pages.iter() {
            let mut valid_page = page.clone();

            while !self.is_page_valid(&valid_page) {
                for i in 0..valid_page.len() {
                    if let Some(rule) = self.rules.get(&valid_page[i]) {
                        for j in 0..i {
                            if rule.after.contains(&valid_page[j]) {
                                valid_page.swap(i, j);
                            }
                        }
                        for j in i + 1..valid_page.len() {
                            if rule.before.contains(&valid_page[j]) {
                                valid_page.swap(i, j);
                            }
                        }
                    }
                }
            }

            total_sum += valid_page[valid_page.len() / 2];
        }

        total_sum
    }

    fn get_rules(lines: &Vec<String>) -> HashMap<i32, Rule> {
        let mut rules_map: HashMap<i32, Rule> = HashMap::new();
        let rules = lines
            .iter()
            .take_while(|x| !x.is_empty())
            .collect::<Vec<_>>();

        for rule in rules {
            let mut parts = rule.split("|").map(str::parse::<i32>);
            let (key, val) = (
                parts.next().unwrap().unwrap(),
                parts.next().unwrap().unwrap(),
            );

            rules_map.insert_rule(key, val);
        }

        rules_map
    }

    fn get_pages(lines: &Vec<String>) -> Vec<Vec<i32>> {
        lines
            .iter()
            .skip_while(|x| !x.is_empty())
            .skip(1)
            .map(|line| {
                line.split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn categorize_pages(&self) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
        let mut valid_pages = Vec::new();
        let mut invalid_pages = Vec::new();

        for page in self.pages.iter() {
            if self.is_page_valid(page) {
                valid_pages.push(page.clone());
            } else {
                invalid_pages.push(page.clone());
            }
        }

        (valid_pages, invalid_pages)
    }

    fn is_page_valid(&self, page: &Vec<i32>) -> bool {
        for (i, number) in page.iter().enumerate() {
            if let Some(rule) = self.rules.get(number) {
                for before in page.iter().take(i) {
                    if rule.after.contains(before) {
                        return false;
                    }
                }
                for after in page.iter().skip(i + 1) {
                    if rule.before.contains(after) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
