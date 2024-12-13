use std::collections::{HashMap, VecDeque};

const ROWS_COLS: usize = 140;

#[derive(PartialEq)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

pub fn execute(data: &Vec<String>) {
    let mut puzzle = [['.'; ROWS_COLS]; ROWS_COLS];
    let mut total: u32 = 0;

    let mut i = 0;
    for line in data {
        let mut j = 0;

        for c in line.chars() {
            puzzle[i][j] = c;

            j += 1;
        }

        i += 1;
    }

    for i in 0..ROWS_COLS {
        for j in 0..ROWS_COLS {
            let entry = puzzle[i][j];

            if entry.is_ascii_lowercase() {
                continue;
            }

            let mut queue: Vec<(usize, usize)> = Vec::new();
            let mut sides = HashMap::<(usize, usize), VecDeque<(usize, usize)>>::new();

            queue.push((i, j));

            let mut area = 0;

            while queue.is_empty() == false {
                let (x, y) = queue.pop().unwrap();

                if puzzle[x][y].is_ascii_lowercase() {
                    continue;
                }

                area += 1;

                if x == 0 || puzzle[x - 1][y] != entry {
                    if x == 0 || puzzle[x - 1][y] != entry.to_ascii_lowercase() {
                        let vec = sides.entry((x, y)).or_insert(VecDeque::new());
                        vec.push_back((x, y + 1));
                    }
                } else {
                    queue.push((x - 1, y));
                }

                if y == 0 || puzzle[x][y - 1] != entry {
                    if y == 0 || puzzle[x][y - 1] != entry.to_ascii_lowercase() {
                        let vec = sides.entry((x + 1, y)).or_insert(VecDeque::new());
                        vec.push_back((x, y));
                    }
                } else {
                    queue.push((x, y - 1));
                }

                if x == ROWS_COLS - 1 || puzzle[x + 1][y] != entry {
                    if x == ROWS_COLS - 1 || puzzle[x + 1][y] != entry.to_ascii_lowercase() {
                        let vec = sides.entry((x + 1, y + 1)).or_insert(VecDeque::new());
                        vec.push_back((x + 1, y));
                    }
                } else {
                    queue.push((x + 1, y));
                }

                if y == ROWS_COLS - 1 || puzzle[x][y + 1] != entry {
                    if y == ROWS_COLS - 1 || puzzle[x][y + 1] != entry.to_ascii_lowercase() {
                        let vec = sides.entry((x, y + 1)).or_insert(VecDeque::new());
                        vec.push_back((x + 1, y + 1));
                    }
                } else {
                    queue.push((x, y + 1));
                }

                puzzle[x][y] = entry.to_ascii_lowercase();
            }

            let mut point_set = Vec::<(usize, usize)>::new();
            for k in sides.keys() {
                point_set.push(k.clone());
            }

            let mut point = (i, j);
            let mut perimeter = 0;
            let mut dir = None;
            let mut start = (i, j);

            loop {
                let (point_x, point_y) = point;
                let v = sides.get_mut(&point).unwrap();
                let (next_point_x, next_point_y) = v.pop_front().unwrap();
                let mut next_dir: Option<Directions> = None;

                if point_x == next_point_x && point_y + 1 == next_point_y {
                    next_dir = Some(Directions::Right);
                } else if point_x == next_point_x && point_y - 1 == next_point_y {
                    next_dir = Some(Directions::Left);
                } else if point_x + 1 == next_point_x && point_y == next_point_y {
                    next_dir = Some(Directions::Down);
                } else if point_x - 1 == next_point_x && point_y == next_point_y {
                    next_dir = Some(Directions::Up);
                }

                if dir != next_dir {
                    perimeter += 1;
                }

                let index = point_set.iter().position(|x| *x == point);
                if let Some(i) = index {
                    point_set.remove(i);
                }

                point = (next_point_x, next_point_y);
                dir = next_dir;

                if point == start {
                    if point_set.is_empty() {
                        break;
                    }

                    let (mut x1, mut y1) = point_set[0];
                    for p in &point_set {
                        let (x2, y2) = p.clone();

                        if x2 < x1 || (x2 == x1 && y2 < y1) {
                            x1 = x2;
                            y1 = y2;
                        }
                    }

                    point = (x1, y1);
                    start = (x1, y1);
                }
            }

            total += area * perimeter;
        }
    }

    println!("Step 2 Total {}", total);
}
