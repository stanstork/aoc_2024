use crate::{utils::split_lines_whitespace, AocDay};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Wire {
    name: String,
    value: Option<u16>,
}

impl Wire {
    fn new(name: &str, value: Option<u16>) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Gate {
    AND,
    OR,
    XOR,
}

impl Gate {
    fn apply(&self, a: u16, b: u16) -> u16 {
        match self {
            Self::AND => a & b,
            Self::OR => a | b,
            Self::XOR => a ^ b,
        }
    }
}

#[derive(Debug, Clone)]
struct Connection {
    input: [String; 2],
    gate: Gate,
    output: String,
}

impl Connection {
    fn new(input1: String, input2: String, gate: Gate, output: String) -> Self {
        Self {
            input: [input1, input2],
            gate,
            output,
        }
    }

    fn matches(&self, left: &str, right: &str, gate: &Gate) -> bool {
        ((self.input[0] == left && self.input[1] == right)
            || (self.input[0] == right && self.input[1] == left))
            && self.gate == *gate
    }

    fn equals(&self, other: &Connection) -> bool {
        self.input[0] == other.input[0]
            && self.input[1] == other.input[1]
            && self.gate == other.gate
            && self.output == other.output
    }
}

pub struct AocDay24 {
    wires: HashMap<String, Wire>,
    connections: Vec<Connection>,
}

impl AocDay24 {
    pub fn new() -> Self {
        let (wires, connections) = Self::parse_input();
        Self { wires, connections }
    }

    pub fn part1(&self) -> usize {
        let (mut wires, mut connections) = (self.wires.clone(), self.connections.clone());
        Self::simulate(&mut wires, &mut connections);

        let mut z_wires = wires
            .iter()
            .filter(|(k, _)| k.starts_with("z"))
            .collect::<Vec<_>>();
        z_wires.sort_by(|a, b| a.0.cmp(b.0));

        let binary = z_wires
            .iter()
            .map(|(_, w)| w.value.unwrap().to_string())
            .collect::<String>();

        usize::from_str_radix(binary.chars().rev().collect::<String>().as_str(), 2).unwrap()
    }

    pub fn part2(&self) -> String {
        let mut connections = self.connections.clone();
        let mut swaps = HashSet::new();

        Self::calc(&mut connections, &mut swaps);

        let mut flat: Vec<_> = swaps.iter().flat_map(|(a, b)| [a, b]).cloned().collect();

        flat.sort();
        flat.join(",")
    }

    fn simulate(wires: &mut HashMap<String, Wire>, connections: &mut Vec<Connection>) {
        let mut queue = VecDeque::new();

        fn enqueue(
            queue: &mut VecDeque<usize>,
            connections: &Vec<Connection>,
            wires: &HashMap<String, Wire>,
        ) {
            for (idx, connection) in connections.iter().enumerate() {
                if wires[connection.output.as_str()].value.is_none()
                    && connection.input.iter().all(|w| wires[w].value.is_some())
                {
                    queue.push_back(idx);
                }
            }
        }

        enqueue(&mut queue, connections, wires);

        while let Some(idx) = queue.pop_front() {
            let connection = &connections[idx];
            let [lhs, rhs] = [
                wires[&connection.input[0]].value.unwrap(),
                wires[&connection.input[1]].value.unwrap(),
            ];
            let result = connection.gate.apply(lhs, rhs);

            wires.get_mut(&connection.output).unwrap().value = Some(result);

            if queue.is_empty() {
                enqueue(&mut queue, connections, wires);
            }
        }
    }

    fn calc(connections: &mut Vec<Connection>, swaps: &mut HashSet<(String, String)>) {
        let mut carry = Self::find_connection(connections, "x00", "y00", Gate::AND).unwrap();

        for i in 1..45 {
            let x = format!("x{:02}", i);
            let y = format!("y{:02}", i);
            let z = format!("z{:02}", i);

            // z = x XOR y XOR carry
            // c_n+1 = (x AND y) OR ((x XOR y) AND c_n)

            let xor1 = Self::find_connection(connections, &x, &y, Gate::XOR).unwrap();
            let and1 = Self::find_connection(connections, &x, &y, Gate::AND).unwrap();

            let xor2 = Self::find_connection(connections, &xor1.output, &carry.output, Gate::XOR);
            let and2 = Self::find_connection(connections, &xor1.output, &carry.output, Gate::AND);

            if let Some(and2) = and2 {
                if xor2.as_ref().unwrap().output != z {
                    let z = connections.iter().find(|c| c.output == z).unwrap();
                    Self::swap_and_retry(connections, swaps, xor2.unwrap(), z.clone());
                    return;
                }

                if and1.output.starts_with("z") {
                    Self::swap_and_retry(connections, swaps, and1.clone(), xor2.unwrap());
                    return;
                }

                carry = Self::find_connection(connections, &and1.output, &and2.output, Gate::OR)
                    .expect("Expected OR gate connection");
            } else {
                Self::swap_and_retry(connections, swaps, xor1.clone(), and1);
                return;
            }
        }
    }

    fn swap_and_retry(
        connections: &mut Vec<Connection>,
        swaps: &mut HashSet<(String, String)>,
        c1: Connection,
        c2: Connection,
    ) {
        swaps.insert((c1.output.clone(), c2.output.clone()));
        Self::swap_output(connections, c1, c2);
        Self::calc(connections, swaps);
    }

    fn swap_output(connections: &mut Vec<Connection>, a: Connection, b: Connection) {
        let index1 = connections.iter().position(|c| c.equals(&a)).unwrap();
        let index2 = connections.iter().position(|c| c.equals(&b)).unwrap();

        let temp = connections[index1].output.clone();
        connections[index1].output = connections[index2].output.clone();
        connections[index2].output = temp;
    }

    fn find_connection(
        connections: &mut Vec<Connection>,
        left: &str,
        right: &str,
        gate: Gate,
    ) -> Option<Connection> {
        connections
            .iter()
            .find(|c| c.matches(left, right, &gate))
            .cloned()
    }

    fn parse_input() -> (HashMap<String, Wire>, Vec<Connection>) {
        let lines = split_lines_whitespace("input/day24.txt");
        let mut wires = HashMap::new();
        let mut connections = vec![];

        for line in &lines.0 {
            let (name, value) = line.split_once(':').unwrap();
            wires.insert(
                name.trim().to_string(),
                Wire::new(name.trim(), Some(value.trim().parse().unwrap())),
            );
        }

        for line in &lines.1 {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let (left, right, output) = (
                parts[0].to_string(),
                parts[2].to_string(),
                parts[4].to_string(),
            );
            let gate = match parts[1] {
                "AND" => Gate::AND,
                "OR" => Gate::OR,
                "XOR" => Gate::XOR,
                _ => panic!("Unknown gate"),
            };

            for name in [&left, &right, &output] {
                wires
                    .entry(name.clone())
                    .or_insert_with(|| Wire::new(name, None));
            }

            connections.push(Connection::new(left, right, gate, output));
        }

        (wires, connections)
    }
}

impl AocDay for AocDay24 {
    fn part1(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part1())
    }

    fn part2(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part2())
    }
}
