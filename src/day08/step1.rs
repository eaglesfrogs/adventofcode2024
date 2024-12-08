use std::collections::HashMap;
use std::collections::HashSet;

const MAX_ROW: i32 = 49;
const MAX_COL: i32 = 49;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_anti_node(&self, other_point: &Point) -> Point {
        let xdiff = self.x - other_point.x;
        let ydiff = self.y - other_point.y;

        return Point {
            x: self.x + xdiff,
            y: self.y + ydiff,
        };
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

                if antinode1.x >= 0
                    && antinode1.x <= MAX_ROW
                    && antinode1.y >= 0
                    && antinode1.y <= MAX_COL
                {
                    anti_node_set.insert(antinode1);
                }

                if antinode2.x >= 0
                    && antinode2.x <= MAX_ROW
                    && antinode2.y >= 0
                    && antinode2.y <= MAX_COL
                {
                    anti_node_set.insert(antinode2);
                }
            }
        }
    }

    println!("Step 1 Total {}", anti_node_set.len());
}
