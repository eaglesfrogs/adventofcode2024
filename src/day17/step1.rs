use regex::Regex;

struct Registers {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
}

impl Registers {
    pub fn get_combo(&self, x: i32) -> i32 {
        if x <= 3 {
            return x;
        } else if x == 4 {
            return self.reg_a;
        } else if x == 5 {
            return self.reg_b;
        } else if x == 6 {
            return self.reg_c;
        } else {
            panic!("whhhhhhhat?")
        }
    }
}

pub fn execute(data: &Vec<String>) {
    let re = Regex::new(r"Register ([A-Z]): (\d+)").unwrap();

    let mut registers = Registers {
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
    };
    let mut program = Vec::<i32>::new();

    for line in data {
        if line.starts_with("Register") {
            let (_, [reg, val]) = re.captures(line).map(|c| c.extract()).unwrap();
            if reg == "A" {
                registers.reg_a = val.parse::<i32>().unwrap();
            } else if reg == "B" {
                registers.reg_b = val.parse::<i32>().unwrap();
            } else if reg == "C" {
                registers.reg_c = val.parse::<i32>().unwrap();
            }
        } else if line.starts_with("Program") {
            let (_, program_string) = line.split_once(' ').unwrap();
            for p in program_string.split(',') {
                program.push(p.parse::<i32>().unwrap());
            }
        }
    }

    let mut output = Vec::<i32>::new();
    let mut cursor: usize = 0;

    while cursor < program.len() {
        let op = program[cursor];
        let v = program[cursor + 1];

        match op {
            0 => {
                //adv
                registers.reg_a = registers.reg_a / (2 as i32).pow(registers.get_combo(v) as u32);
                cursor += 2;
            }
            1 => {
                //bxl
                registers.reg_b = registers.reg_b ^ v;
                cursor += 2;
            }
            2 => {
                //bst
                registers.reg_b = registers.get_combo(v) % 8;
                cursor += 2;
            }
            3 => {
                //jnz
                if registers.reg_a == 0 {
                    cursor += 2;
                } else {
                    cursor = v as usize;
                }
            }
            4 => {
                //bxc
                registers.reg_b = registers.reg_b ^ registers.reg_c;
                cursor += 2;
            }
            5 => {
                //out
                output.push(registers.get_combo(v) % 8);
                cursor += 2;
            }
            6 => {
                //bdv
                registers.reg_b = registers.reg_a / (2 as i32).pow(registers.get_combo(v) as u32);
                cursor += 2;
            }
            7 => {
                //cdv
                registers.reg_c = registers.reg_a / (2 as i32).pow(registers.get_combo(v) as u32);
                cursor += 2;
            }
            _ => {}
        }
    }

    let mut output_str: String = String::new();
    for o in output {
        output_str = format!("{},{}", output_str, o);
    }

    println!("Step 1 Output {}", output_str);
}
