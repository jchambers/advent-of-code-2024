use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let device = MonitoringDevice::from_str(fs::read_to_string(path)?.as_str())?;

        println!("z-value: {}", device.z_value());

        Ok(())
    } else {
        Err("Usage: day24 INPUT_FILE_PATH".into())
    }
}

struct MonitoringDevice {
    wires: HashMap<String, bool>,
    gates: Vec<Gate>,
}

impl MonitoringDevice {
    pub fn z_value(&self) -> u64 {
        let mut resolved_wires = self.wires.clone();
        let mut unresolved_gates = self.gates.clone();

        while !unresolved_gates.is_empty() {
            let resolvable_gate_indices = unresolved_gates
                .iter()
                .enumerate()
                .filter(|(_, gate)| {
                    resolved_wires.contains_key(&gate.inputs[0])
                        && resolved_wires.contains_key(&gate.inputs[1])
                })
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            resolvable_gate_indices.iter().rev().for_each(|&i| {
                let resolved_gate = unresolved_gates.remove(i);

                let a = resolved_wires.get(&resolved_gate.inputs[0]).unwrap();
                let b = resolved_wires.get(&resolved_gate.inputs[1]).unwrap();

                let result = match resolved_gate.operation {
                    Operation::And => a & b,
                    Operation::Or => a | b,
                    Operation::Xor => a ^ b,
                };

                resolved_wires.insert(resolved_gate.output, result);
            })
        }

        let mut z = 0;

        for i in 0..u64::BITS {
            let key = format!("z{:02}", i);
            let value = resolved_wires
                .get(&key)
                .map(|&v| if v { 1 } else { 0 })
                .unwrap_or(0);

            z |= value << i;
        }

        z
    }
}

impl FromStr for MonitoringDevice {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((wires, gates)) = s.split_once("\n\n") {
            let wires = wires
                .lines()
                .map(|line| {
                    if let Some((wire, value)) = line.split_once(": ") {
                        let value: Result<bool, Box<dyn Error>> = match value {
                            "1" => Ok(true),
                            "0" => Ok(false),
                            _ => Err("Unexpected value".into()),
                        };

                        value.map(|v| (wire.to_string(), v))
                    } else {
                        Err("Could not parse wire line".into())
                    }
                })
                .collect::<Result<HashMap<String, bool>, _>>()?;

            let gates = gates
                .lines()
                .map(Gate::from_str)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(MonitoringDevice { wires, gates })
        } else {
            Err("Could not parse monitoring device definition".into())
        }
    }
}

#[derive(Clone)]
struct Gate {
    inputs: [String; 2],
    output: String,
    operation: Operation,
}

impl FromStr for Gate {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((inputs, output)) = s.split_once(" -> ") {
            if let &[input_a, operation, input_b] =
                inputs.split_whitespace().collect::<Vec<_>>().as_slice()
            {
                Ok(Gate {
                    inputs: [input_a.to_string(), input_b.to_string()],
                    output: output.to_string(),
                    operation: Operation::from_str(operation)?,
                })
            } else {
                Err("Could not parse inputs".into())
            }
        } else {
            Err("Could not parse gate definition".into())
        }
    }
}

#[derive(Copy, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

impl FromStr for Operation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Operation::And),
            "OR" => Ok(Operation::Or),
            "XOR" => Ok(Operation::Xor),
            _ => Err("Unrecognized operation".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_DEVICE: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};

    #[test]
    fn test_z_value() {
        let device = MonitoringDevice::from_str(TEST_DEVICE).unwrap();

        assert_eq!(2024, device.z_value());
    }
}
