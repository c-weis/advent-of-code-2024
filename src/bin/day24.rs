use itertools::Itertools;
use rusty_advent_2024::utils::file_io;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    str::FromStr,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum GateType {
    XOR,
    AND,
    OR,
}

impl GateType {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            GateType::XOR => a ^ b,
            GateType::AND => a & b,
            GateType::OR => a | b,
        }
    }
}

impl Display for GateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GateType::XOR => "XOR", //"^",
                GateType::AND => "AND", //"&",
                GateType::OR => "OR",   //"|",
            }
        )
    }
}

#[derive(Debug)]
struct InvalidGateString(String);
impl FromStr for GateType {
    type Err = InvalidGateString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::AND),
            "XOR" => Ok(Self::XOR),
            "OR" => Ok(Self::OR),
            _ => Err(InvalidGateString(String::from(s))),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Gate {
    // a op b -> c
    a: String,
    b: String,
    op: GateType,
}

impl Gate {
    fn mirror(self) -> Self {
        Gate {
            a: self.b,
            b: self.a,
            op: self.op,
        }
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.a, self.op, self.b)
    }
}

struct Device {
    known_values: HashMap<String, bool>,
    gate_map: HashMap<String, Gate>,
    input_bits: usize,
}

#[derive(Debug)]
enum SpecialParseBoolError {
    WrongChar(char),
    WrongLength(usize),
}

fn special_bool_parse(slice: &str) -> Result<bool, SpecialParseBoolError> {
    match slice.chars().exactly_one() {
        Ok('0') => Ok(false),
        Ok('1') => Ok(true),
        Ok(c) => Err(SpecialParseBoolError::WrongChar(c)),
        _ => Err(SpecialParseBoolError::WrongLength(slice.len())),
    }
}

#[derive(Debug)]
enum DeviceError {
    CircularGateError,
    IncompleteDeviceError,
}

#[derive(Clone, Debug)]
struct Adder {
    x_in: String,
    y_in: String,
    c_in: String,
    bit_xor: String,
    bit_and: String,
    pre_c_out: String,
    c_out: String,
    s_out: String,
}

impl Device {
    fn compute(&mut self, name: &String) -> Result<bool, DeviceError> {
        self._compute(name, &mut HashSet::new())
    }

    fn _compute(
        &mut self,
        name: &String,
        indeterminates: &mut HashSet<String>,
    ) -> Result<bool, DeviceError> {
        if indeterminates.contains(name) {
            return Err(DeviceError::CircularGateError);
        }
        if let Some(value) = self.known_values.get(name) {
            return Ok(*value);
        } else {
            let gate = self
                .gate_map
                .get(name)
                .ok_or(DeviceError::IncompleteDeviceError)?
                .clone();

            indeterminates.insert(name.clone());
            let a = self._compute(&gate.a, &mut indeterminates.clone())?;
            let b = self._compute(&gate.b, &mut indeterminates.clone())?;
            let value = gate.op.apply(a, b);

            self.known_values.insert(name.clone(), value);
            Ok(value)
        }
    }

    fn _assemble(&self, c: char) -> u64 {
        let mut num: u64 = 0;
        let mut i = 00;
        while let Some(&b) = self.known_values.get(&format!("{c}{i:02}")) {
            if b {
                num += 1 << i;
            }
            i += 1;
        }
        num
    }

    fn set_x_y(&mut self, x: u64, y: u64) {
        self.known_values.clear();

        // (x >> i & 1) == 1 determines if bit i is set
        for i in 0..self.input_bits {
            self.known_values
                .insert(format!("x{i:02}"), (x >> i & 1) == 1);
            self.known_values
                .insert(format!("y{i:02}"), (y >> i & 1) == 1);
        }
    }

    fn x(&self) -> u64 {
        self._assemble('x')
    }

    fn y(&self) -> u64 {
        self._assemble('y')
    }

    fn z(&mut self) -> Result<u64, DeviceError> {
        let z_digits: Vec<String> = self
            .gate_map
            .keys()
            .filter(|key| key.as_str().starts_with("z"))
            .cloned()
            .collect();

        for z_digit in z_digits {
            self.compute(&z_digit)?;
        }

        Ok(self._assemble('z'))
    }

    fn is_valid(&mut self) -> bool {
        !self.z().is_err()
    }

    fn swap_gates(&mut self, name1: &String, name2: &String) {
        let gate1 = self
            .gate_map
            .get(name1)
            .cloned()
            .expect("No gate for {name1} found!");

        let gate2 = self
            .gate_map
            .get(name2)
            .cloned()
            .expect("No gate for {name2} found!");

        self.gate_map.insert(name1.to_string(), gate2);
        self.gate_map.insert(name2.to_string(), gate1);
        self.known_values.clear();
    }

    fn from_file(path: &str) -> Self {
        let mut lines = file_io::strings_from_file(path);

        let known_values: HashMap<String, bool> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| -> (String, bool) {
                line.split_once(": ")
                    .and_then(|(s, v)| -> Option<(String, bool)> {
                        Some((
                            String::from(s),
                            special_bool_parse(v).expect("Bool could not be parsed."),
                        ))
                    })
                    .expect("Known values should be declared as 'xyz: 0/1'.")
            })
            .collect();

        let gate_map: HashMap<String, Gate> = lines
            .map(|line| -> (String, Gate) {
                match line.split_whitespace().collect_tuple() {
                    Some((a, op, b, _, c)) => (
                        c.into(),
                        Gate {
                            a: a.into(),
                            op: op.parse().expect("Operation could not be parsed."),
                            b: b.into(),
                        },
                    ),
                    _ => panic!("Line {line} could not be parsed."),
                }
            })
            .collect();

        Device {
            input_bits: known_values
                .keys()
                .filter(|name| name.starts_with("x"))
                .count(),
            known_values,
            gate_map,
        }
    }

    const MISSING_NODE: &str = " _";

    fn gate_name(gate: &Gate, inverted_gate_map: &HashMap<Gate, String>) -> String {
        inverted_gate_map
            .get(gate)
            .cloned()
            .unwrap_or(Self::MISSING_NODE.into())
    }

    fn x_str(bit: usize) -> String {
        format!("x{bit:02}")
    }

    fn y_str(bit: usize) -> String {
        format!("y{bit:02}")
    }

    fn z_str(bit: usize) -> String {
        format!("z{bit:02}")
    }

    fn decompose_into_adders(&self) -> Vec<Adder> {
        let output_bits = self.input_bits + 1;
        let mut inverted_gate_map: HashMap<Gate, String> = HashMap::new();
        for (name, gate) in &self.gate_map {
            if let Some(old_name) = inverted_gate_map.insert(gate.clone(), name.clone()) {
                panic!("Gate {name} was inserted as {old_name} before.");
            }
            if let Some(old_name) = inverted_gate_map.insert(gate.clone().mirror(), name.clone()) {
                panic!("Gate {name} was inserted with {old_name} before.");
            }
        }

        // Reconstruct adding by hand, check where device deviates
        // Half-adders
        let mut bit_xor_gates: Vec<String> = vec![];
        let mut bit_and_gates: Vec<String> = vec![];
        for bit in 0..self.input_bits {
            bit_xor_gates.push(Self::gate_name(
                &Gate {
                    a: Self::x_str(bit),
                    b: Self::y_str(bit),
                    op: GateType::XOR,
                },
                &inverted_gate_map,
            ));
            bit_and_gates.push(Self::gate_name(
                &Gate {
                    a: Self::x_str(bit),
                    b: Self::y_str(bit),
                    op: GateType::AND,
                },
                &inverted_gate_map,
            ));
        }

        // Full adders
        // C_{i+1} = (x_i & y_i) | (C_i & (x_i ^ y_i))
        // pre_carry_{i+1} := C_i & (x_i ^ y_i)
        // carry_{i+1} := (x_i & y_i) | pre_carry_{i+1}
        let mut pre_carry_gates: Vec<String> =
            vec![Self::MISSING_NODE.into(), Self::MISSING_NODE.into()];
        let mut carry_gates: Vec<String> =
            vec![Self::MISSING_NODE.into(), bit_and_gates[0].clone()];
        for bit in 2..output_bits {
            pre_carry_gates.push(Self::gate_name(
                &Gate {
                    a: carry_gates[bit - 1].clone(),
                    b: bit_xor_gates[bit - 1].clone(),
                    op: GateType::AND,
                },
                &inverted_gate_map,
            ));
            carry_gates.push(Self::gate_name(
                &Gate {
                    a: bit_and_gates[bit - 1].clone(),
                    b: pre_carry_gates[bit].clone(),
                    op: GateType::OR,
                },
                &inverted_gate_map,
            ));
        }

        // outputs:
        let mut out_gates: Vec<String> = vec![bit_xor_gates[0].clone()];
        for bit in 1..self.input_bits {
            out_gates.push(Self::gate_name(
                &Gate {
                    a: bit_xor_gates[bit].clone(),
                    b: carry_gates[bit].clone(),
                    op: GateType::XOR,
                },
                &inverted_gate_map,
            ));
        }
        out_gates.push(carry_gates[output_bits - 1].clone());

        let mut adders: Vec<Adder> = vec![];
        for bit in 0..self.input_bits {
            adders.push(Adder {
                x_in: Self::x_str(bit),
                y_in: Self::y_str(bit),
                c_in: carry_gates[bit].clone(),
                bit_xor: bit_xor_gates[bit].clone(),
                bit_and: bit_and_gates[bit].clone(),
                pre_c_out: pre_carry_gates[bit + 1].clone(),
                c_out: carry_gates[bit + 1].clone(),
                s_out: out_gates[bit].clone(),
            })
        }

        adders
    }
}

fn part1(path: &str) -> u64 {
    let mut device = Device::from_file(path);
    device.z().expect("Device should be self-consistent.")
}

fn part2(path: &str) -> String {
    let mut device = Device::from_file(path);

    println!("{}", mermaid_diagram(&device));

    // This first pair is not detected by the loop below.
    // I found it by inspection of the mermaid diagram I print above
    let gate1: String = "NOT".into();
    let gate2: String = "TRU".into();
    device.swap_gates(&gate1, &gate2);

    let mut swapped_gates: Vec<String> = vec![gate1, gate2]
        .into_iter()
        .map(|s| s.into())
        .collect_vec();

    for _ in 0..4 {
        let adders = device.decompose_into_adders();
        for (bit, adder) in adders.iter().enumerate() {
            if adder.s_out != Device::z_str(bit) {
                swapped_gates.push(adder.s_out.clone());
                swapped_gates.push(Device::z_str(bit));
                device.swap_gates(&adder.s_out, &Device::z_str(bit));
                break;
            }
        }
    }
    swapped_gates.sort();
    swapped_gates.join(",")
}

fn mermaid_diagram(device: &Device) -> String {
    let adders = device.decompose_into_adders();
    let mermaid_adder_subgraphs: String = adders
        .iter()
        .by_ref()
        .enumerate()
        .map(|(idx, adder)| {
            format!(
                concat!(
                    "    subgraph adder{:02}\n",
                    "        {}[X]\n",
                    "        {}[Y]\n",
                    "        {}[XOR]\n",
                    "        {}[AND]\n",
                    "        {}[AND]\n",
                    "        {}[C]\n",
                    "        {}_[S]\n",
                    "    end"
                ),
                idx,
                adder.x_in.clone(),
                adder.y_in.clone(),
                adder.bit_xor.clone(),
                adder.bit_and.clone(),
                adder.pre_c_out.clone(),
                adder.c_out.clone(),
                adder.s_out.clone(),
            )
        })
        .join("\n");

    /*let mermaid_connectors: String = adders
        .iter()
        .flat_map(|adder| {
            vec![
                (adder.x_in.clone(), adder.bit_xor.clone()),
                (adder.x_in.clone(), adder.bit_and.clone()),
                (adder.y_in.clone(), adder.bit_xor.clone()),
                (adder.y_in.clone(), adder.bit_and.clone()),
                (adder.c_in.clone(), adder.s_out.clone()),
                (adder.c_in.clone(), adder.pre_c_out.clone()),
                (adder.bit_xor.clone(), adder.s_out.clone()),
                (adder.bit_xor.clone(), adder.pre_c_out.clone()),
                (adder.pre_c_out.clone(), adder.c_out.clone()),
                (adder.bit_and.clone(), adder.c_out.clone()),
                (format!("{}_", adder.s_out), adder.s_out.clone()),
            ]
        })
        .map(|(source, target)| format!("    {}-->{}", source, target))
        .join("\n");
    */

    let mermaid_connectors: String = device
        .gate_map
        .iter()
        .map(|(name, gate)| (name, gate.a.clone(), gate.b.clone(), gate.op.clone()))
        .map(|(name, a, b, op)| {
            format!(
                concat!("    {}-->{}[{}:{}]\n", "    {}-->{}\n"),
                a, name, op, name, b, name
            )
        })
        .collect();

    //.map(|(source, target)| format!("    {}-->{}", source, target))
    //.join("\n");

    [
        "\n",
        "flowchart TB\n",
        mermaid_adder_subgraphs.as_str(),
        mermaid_connectors.as_str(),
    ]
    .join("\n")
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input24.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input24.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_eq() {
        assert_eq!(
            Gate {
                a: "a".into(),
                b: "b".into(),
                op: GateType::AND
            },
            Gate {
                a: "b".into(),
                b: "a".into(),
                op: GateType::AND
            }
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input24.txt.test1"), 4);
        assert_eq!(part1("input/input24.txt.test2"), 2024);
    }
}
