use std::collections::HashMap;
use std::collections::HashSet;

const MAX_ROW: i32 = 49;
const MAX_COL: i32 = 49;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_anti_node(&self, other_point: &Point) -> HashSet<Point> {
        let xdiff = self.x - other_point.x;
        let ydiff = self.y - other_point.y;

        let mut result: HashSet<Point> = HashSet::new();

        let mut new_x = self.x + xdiff;
        let mut new_y = self.y + ydiff;

        while new_x >= 0 && new_x <= MAX_ROW && new_y >= 0 && new_y <= MAX_COL {
            result.insert(Point { x: new_x, y: new_y });

            new_x += xdiff;
            new_y += ydiff;
        }

        result
    }
}

pub fn execute(data: &Vec<String>) {
    let mut anti_node_set: HashSet<Point> = HashSet::new();
    let mut point_map: HashMap<char, Vec<Point>> = HashMap::new();

    let mut r = 0;

    for line in data {
        let mut c = 0;

        for antenna in line.chars() {
            if antenna != '.' {
                let point = Point { x: r, y: c };
                let point_vec = point_map.entry(antenna).or_insert(Vec::new());
                point_vec.push(point);
            }

            c += 1;
        }

        r += 1;
    }

    for antenna in point_map.keys() {
        let point_vec = point_map.get(antenna).unwrap();

        for i in 0..point_vec.len() {
            let point1 = &point_vec[i];

            for j in i + 1..point_vec.len() {
                let point2 = &point_vec[j];

                let antinode1 = point1.get_anti_node(point2);
                let antinode2 = point2.get_anti_node(point1);

                anti_node_set.extend(antinode1.into_iter());
                anti_node_set.extend(antinode2.into_iter());
                anti_node_set.insert(point1.clone());
                anti_node_set.insert(point2.clone());
            }
        }
    }

    println!("Step 2 Total {}", anti_node_set.len());
}
