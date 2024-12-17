use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let mut computer = Computer::from_str(fs::read_to_string(path)?.as_str())?;

        println!(
            "Program output: {}",
            computer
                .run_program()?
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );

        println!("Quine with register A: {}", computer.quine_register_a()?);

        Ok(())
    } else {
        Err("Usage: day17 INPUT_FILE_PATH".into())
    }
}

struct Computer {
    registers: [u64; 3],
    program: Vec<u8>,
}

impl Computer {
    pub fn run_program(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut instruction_pointer = 0;
        let mut output = Vec::new();

        while instruction_pointer < self.program.len() {
            let operand = self.program[instruction_pointer + 1];
            let mut jumped = false;

            match Instruction::try_from(self.program[instruction_pointer])? {
                Instruction::ADV => self.registers[0] >>= self.combo_operand(operand)?,
                Instruction::BXL => self.registers[1] ^= operand as u64,
                Instruction::BST => self.registers[1] = self.combo_operand(operand)? % 8,
                Instruction::JNZ => {
                    if self.registers[0] != 0 {
                        instruction_pointer = operand as usize;
                        jumped = true;
                    }
                }
                Instruction::BXC => self.registers[1] ^= self.registers[2],
                Instruction::OUT => output.push(self.combo_operand(operand)? % 8),
                Instruction::BDV => {
                    self.registers[1] = self.registers[0] >> self.combo_operand(operand)?
                }
                Instruction::CDV => {
                    self.registers[2] = self.registers[0] >> self.combo_operand(operand)?
                }
            }

            if !jumped {
                instruction_pointer += 2;
            }
        }

        Ok(output.iter().map(|&value| value as u8).collect())
    }

    pub fn quine_register_a(&self) -> Result<u64, Box<dyn Error>> {
        // Intentionally skip 0, since we know that will give us an empty result
        let mut stack: Vec<StackOperation> = (1..8)
            .rev()
            .filter(|&a| {
                if let Ok(digits) = self.run_program_with_register_a(a) {
                    self.program.ends_with(&digits)
                } else {
                    false
                }
            })
            .map(StackOperation::Explore)
            .collect();

        let mut a = 0;

        while let Some(stack_operation) = stack.pop() {
            match stack_operation {
                StackOperation::Explore(candidate_bits) => {
                    a = (a << 3) | candidate_bits;

                    if self.run_program_with_register_a(a)? == self.program {
                        return Ok(a);
                    }

                    stack.push(StackOperation::Backtrack);

                    stack.extend(
                        (0..8)
                            .rev()
                            .filter(|low_bits| {
                                let candidate_a = (a << 3) | low_bits;

                                if let Ok(digits) = self.run_program_with_register_a(candidate_a) {
                                    self.program.ends_with(&digits)
                                } else {
                                    false
                                }
                            })
                            .map(StackOperation::Explore),
                    );
                }

                StackOperation::Backtrack => a >>= 3,
            }
        }

        Err("Could not find quine value for register A".into())
    }

    fn run_program_with_register_a(&self, a: u64) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut computer = Computer {
            registers: [a, 0, 0],
            program: self.program.clone(),
        };

        computer.run_program()
    }

    fn combo_operand(&self, operand: u8) -> Result<u64, Box<dyn Error>> {
        match operand {
            0..=3 => Ok(operand as u64),
            4 => Ok(self.registers[0]),
            5 => Ok(self.registers[1]),
            6 => Ok(self.registers[2]),
            _ => Err("Unexpected combo operand".into()),
        }
    }
}

impl FromStr for Computer {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        let mut registers = [0; 3];

        if let Some(a) = lines[0].strip_prefix("Register A: ") {
            registers[0] = a.parse()?;
        } else {
            return Err("Could not parse line for register A".into());
        }

        if let Some(b) = lines[1].strip_prefix("Register B: ") {
            registers[1] = b.parse()?;
        } else {
            return Err("Could not parse line for register B".into());
        }

        if let Some(c) = lines[2].strip_prefix("Register C: ") {
            registers[2] = c.parse()?;
        } else {
            return Err("Could not parse line for register C".into());
        }

        let program;

        if let Some(instructions) = lines[4].strip_prefix("Program: ") {
            program = instructions
                .split(',')
                .map(|i| i.parse())
                .collect::<Result<_, _>>()?;
        } else {
            return Err("Could not parse program line".into());
        }

        Ok(Computer { registers, program })
    }
}

#[allow(clippy::upper_case_acronyms)]
enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl TryFrom<u8> for Instruction {
    type Error = Box<dyn Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::ADV),
            1 => Ok(Instruction::BXL),
            2 => Ok(Instruction::BST),
            3 => Ok(Instruction::JNZ),
            4 => Ok(Instruction::BXC),
            5 => Ok(Instruction::OUT),
            6 => Ok(Instruction::BDV),
            7 => Ok(Instruction::CDV),
            _ => Err("Unrecognized opcode".into()),
        }
    }
}

#[derive(Debug)]
enum StackOperation {
    Explore(u64),
    Backtrack,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_COMPUTER: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[test]
    fn test_run_program() {
        let mut computer = Computer::from_str(TEST_COMPUTER).unwrap();

        assert_eq!(
            "4,6,3,5,6,3,5,2,1,0",
            computer
                .run_program()
                .unwrap()
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}
