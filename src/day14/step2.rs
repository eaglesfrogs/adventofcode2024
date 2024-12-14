use std::{i32, str::FromStr};

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

    pub fn move_robot(&mut self) {
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

pub fn execute(data: &Vec<String>) {
    let line_re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots: Vec<Robot> = Vec::new();

    for line in data {
        let (_, [x, y, vx, vy]) = line_re.captures(line).map(|c| c.extract()).unwrap();
        let robot = Robot::new(
            x.parse::<i32>().unwrap(),
            y.parse::<i32>().unwrap(),
            vx.parse::<i32>().unwrap(),
            vy.parse::<i32>().unwrap(),
        );

        robots.push(robot);
    }

    // scan the board for 10 points in a row
    let mut seconds = 0;
    'tree: loop {
        let mut board = [['.'; HEIGHT as usize]; WIDTH as usize];
        for r in &robots {
            board[r.x as usize][r.y as usize] = 'X';
        }

        for x in 0..WIDTH {
            for y in 0..HEIGHT - 10 {
                let mut found_it = true;
                for i in 0..10 {
                    found_it = board[x as usize][(y + i) as usize] == 'X';

                    if found_it == false {
                        break;
                    }
                }

                if found_it == true {
                    for i in 0..HEIGHT {
                        let mut line = String::from_str("").unwrap();
                        for j in 0..WIDTH {
                            line.push(board[j as usize][i as usize]);
                        }

                        println!("{}", line);
                    }

                    break 'tree;
                }
            }
        }

        for r in &mut robots {
            r.move_robot();
        }
        seconds += 1;
    }

    println!("Step 1 Total {}", seconds);
}
