use regex::Regex;

//Cramer's rule - https://www.purplemath.com/modules/cramers.htm
fn how_dare_you_make_me_do_linear_algebra(machine: &Machine) -> Option<i64> {
    let determinant = machine.ax * machine.by - machine.bx * machine.ay;
    let a_determinant = machine.prizex * machine.by - machine.prizey * machine.bx;
    let b_determinant = machine.prizey * machine.ax - machine.prizex * machine.ay;

    let a_presses: i64 = a_determinant / determinant;
    let b_presses: i64 = b_determinant / determinant;

    if machine.ax * a_presses + machine.bx * b_presses == machine.prizex
        && machine.ay * a_presses + machine.by * b_presses == machine.prizey
    {
        return Some(a_presses * 3 + b_presses);
    }
    return None;
}

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    prizex: i64,
    prizey: i64,
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

    let mut total: i64 = 0;
    let mut machine = Machine::new();

    for line in data {
        if line.starts_with("Button A") {
            machine = Machine::new();

            let (_, [x, y]) = button_re.captures(line).map(|c| c.extract()).unwrap();
            machine.ax = x.parse::<i64>().unwrap();
            machine.ay = y.parse::<i64>().unwrap();
        } else if line.starts_with("Button B") {
            let (_, [x, y]) = button_re.captures(line).map(|c| c.extract()).unwrap();
            machine.bx = x.parse::<i64>().unwrap();
            machine.by = y.parse::<i64>().unwrap();
        } else if line.starts_with("Prize") {
            let (_, [x, y]) = prize_re.captures(line).map(|c| c.extract()).unwrap();
            machine.prizex = x.parse::<i64>().unwrap() + 10000000000000;
            machine.prizey = y.parse::<i64>().unwrap() + 10000000000000;

            if let Some(t) = how_dare_you_make_me_do_linear_algebra(&machine) {
                total += t;
            }
        }
    }

    println!("Step 2 Total {}", total);
}
