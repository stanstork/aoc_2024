#[derive(Debug, Clone)]
enum Op {
    Add,
    Multiply,
    Concat,
}

impl Op {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Multiply => a * b,
            Op::Concat => format!("{}{}", a, b).parse::<i64>().unwrap(),
        }
    }
}

pub struct AocDay7 {
    equations: Vec<(i64, Vec<i64>)>,
}

impl AocDay7 {
    pub fn new(input: Vec<String>) -> Self {
        let mut equations = Vec::new();

        for line in input {
            let equation = line.split(":").map(|x| x.trim()).collect::<Vec<_>>();
            let target = equation[0].parse::<i64>().unwrap();
            let nums = equation[1]
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            equations.push((target, nums));
        }

        AocDay7 { equations }
    }

    pub fn part1(&self) -> i64 {
        let ops = vec![Op::Add, Op::Multiply];
        self.calc(&ops)
    }

    pub fn part2(&self) -> i64 {
        let ops = vec![Op::Add, Op::Multiply, Op::Concat];
        self.calc(&ops)
    }

    fn calc(&self, ops: &[Op]) -> i64 {
        let mut sum = 0;

        for (target, nums) in &self.equations {
            let mut combos = Vec::new();
            Self::gen_ops(nums.len() - 1, ops, &mut combos, Vec::new());

            for operations in combos {
                let mut total = nums[0];
                for i in 0..operations.len() {
                    total = operations[i].apply(total, nums[i + 1]);
                }

                if total == *target {
                    sum += target;
                    break;
                }
            }
        }

        sum
    }

    fn gen_ops(n: usize, ops: &[Op], combos: &mut Vec<Vec<Op>>, current: Vec<Op>) {
        if n == 0 {
            combos.push(current.clone());
            return;
        }

        for op in ops {
            let mut next = current.clone();
            next.push(op.clone());
            Self::gen_ops(n - 1, ops, combos, next);
        }
    }
}
