use std::i32;

use regex::Regex;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Self {
            x: x,
            y: y,
            vx: vx,
            vy: vy,
        }
    }

    pub fn move_seconds(&mut self, seconds: i32) {
        for _ in 0..seconds {
            let mut new_x = self.x + self.vx;
            let mut new_y = self.y + self.vy;

            if new_x < 0 {
                new_x = WIDTH + new_x;
            } else if new_x >= WIDTH {
                new_x = new_x - WIDTH;
            }

            if new_y < 0 {
                new_y = HEIGHT + new_y;
            } else if new_y >= HEIGHT {
                new_y = new_y - HEIGHT;
            }

            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn get_quadrant(&self) -> Option<usize> {
        if self.x < 50 && self.y < 51 {
            return Some(0);
        } else if self.x > 50 && self.y < 51 {
            return Some(1);
        } else if self.x < 50 && self.y > 51 {
            return Some(2);
        } else if self.x > 50 && self.y > 51 {
            return Some(3);
        }

        return None;
    }
}

pub fn execute(data: &Vec<String>) {
    let line_re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut quadrants = [0; 4];

    for line in data {
        let (_, [x, y, vx, vy]) = line_re.captures(line).map(|c| c.extract()).unwrap();
        let mut robot = Robot::new(
            x.parse::<i32>().unwrap(),
            y.parse::<i32>().unwrap(),
            vx.parse::<i32>().unwrap(),
            vy.parse::<i32>().unwrap(),
        );
        robot.move_seconds(100);

        if let Some(quadrant) = robot.get_quadrant() {
            quadrants[quadrant] += 1;
        }
    }

    let total = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];

    println!("Step 1 Total {}", total);
}
