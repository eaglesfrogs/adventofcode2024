use std::i32;

use regex::Regex;

fn push_buttons(machine: &Machine) -> Option<i32> {
    let mut a_presses = 0;
    let mut total: Option<i32> = None;

    while a_presses * machine.ax <= machine.prizex && a_presses * machine.ay <= machine.prizey {
        let mut b_presses = 0;

        let mut x = a_presses * machine.ax + b_presses * machine.bx;
        let mut y = a_presses * machine.ay + b_presses * machine.by;

        while x < machine.prizex && y < machine.prizey {
            b_presses += 1;

            x = a_presses * machine.ax + b_presses * machine.bx;
            y = a_presses * machine.ay + b_presses * machine.by;
        }

        if x == machine.prizex && y == machine.prizey {
            let t = Some(3 * a_presses + b_presses);

            if total == None || t.unwrap() < total.unwrap() {
                total = t;
            }
        }

        a_presses += 1;
    }

    return total;
}

struct Machine {
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    prizex: i32,
    prizey: i32,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            ax: 0,
            ay: 0,
            bx: 0,
            by: 0,
            prizex: 0,
            prizey: 0,
        }
    }
}

pub fn execute(data: &Vec<String>) {
    let button_re = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d++)").unwrap();

    let mut total: i32 = 0;
    let mut machine = Machine::new();

    for line in data {
        if line.starts_with("Button A") {
            machine = Machine::new();

            let (_, [x, y]) = button_re.captures(line).map(|c| c.extract()).unwrap();
            machine.ax = x.parse::<i32>().unwrap();
            machine.ay = y.parse::<i32>().unwrap();
        } else if line.starts_with("Button B") {
            let (_, [x, y]) = button_re.captures(line).map(|c| c.extract()).unwrap();
            machine.bx = x.parse::<i32>().unwrap();
            machine.by = y.parse::<i32>().unwrap();
        } else if line.starts_with("Prize") {
            let (_, [x, y]) = prize_re.captures(line).map(|c| c.extract()).unwrap();
            machine.prizex = x.parse::<i32>().unwrap();
            machine.prizey = y.parse::<i32>().unwrap();

            if let Some(t) = push_buttons(&machine) {
                total += t;
            }
        }
    }

    println!("Step 1 Total {}", total);
}
