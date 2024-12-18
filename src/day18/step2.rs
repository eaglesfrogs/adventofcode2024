use std::{cmp::min, collections::HashSet};

const MAX_X_Y: usize = 71;

/*
Dijkstras Algorithm

Start with an arbitrary node in your graph and an empty data structure which has an efficient getLeastElement() or similar method (ie priority queue).

This node has distance 0 (to itself). No path to any other nodes have been discovered so we give them inifinite distance (in practice largest int that doesn't cause overflow). Set that node to CurrentNode.

Follow every edge from CurrentNode. Set the node on the opposite end of each edge to be Target.

If Target is 'visited', skip it. Otherwise. Set Target's distance to be the minimum of it's current value and the CurrentNode's distance + length of edge.

Mark your CurrentNode as visited.

If destination is visited or all unvisited nodes have distance infinity (there is no known path to any other nodes), stop.

Check the set of unvisited nodes for the one with the least distance. Make that your next CurrentNode and go to step 3.
*/

pub fn execute(data: &Vec<String>) {
    let mut initial_board = [['.'; MAX_X_Y]; MAX_X_Y];
    let mut i = 0;

    let mut failable_points = Vec::<(usize, usize)>::new();

    for line in data {
        let (x_str, y_str) = line.split_once(",").unwrap();
        let x = x_str.parse::<usize>().unwrap();
        let y = y_str.parse::<usize>().unwrap();

        if i < 1024 {
            initial_board[x][y] = '#';
        } else {
            failable_points.push((x, y));
        }

        i += 1;
    }

    for failable_point in failable_points {
        let (fail_x, fail_y) = failable_point;
        initial_board[fail_x][fail_y] = '#';

        let board = initial_board.clone();
        let mut visited_board = [[false; MAX_X_Y]; MAX_X_Y];
        let mut distance_board = [[i32::MAX; MAX_X_Y]; MAX_X_Y];
        let mut next_nodes = HashSet::<(usize, usize)>::new();

        let mut x: usize = 0;
        let mut y: usize = 0;

        distance_board[x][y] = 0;
        next_nodes.insert((x, y));

        while next_nodes.len() > 0 {
            let mut min_node: Option<(usize, usize)> = None;

            for n in next_nodes.iter() {
                if min_node == None {
                    min_node = Some(n.clone());
                } else {
                    let (x1, y1) = min_node.unwrap();
                    let (x2, y2) = n.clone();

                    if distance_board[x2][y2] < distance_board[x1][y1] {
                        min_node = Some((x2, y2));
                    }
                }
            }

            (x, y) = min_node.unwrap();
            next_nodes.remove(&(x, y));

            if x > 0 && board[x - 1][y] != '#' && visited_board[x - 1][y] == false {
                distance_board[x - 1][y] = min(distance_board[x - 1][y], distance_board[x][y] + 1);
                next_nodes.insert((x - 1, y));
            }

            if y > 0 && board[x][y - 1] != '#' && visited_board[x][y - 1] == false {
                distance_board[x][y - 1] = min(distance_board[x][y - 1], distance_board[x][y] + 1);
                next_nodes.insert((x, y - 1));
            }

            if x < MAX_X_Y - 1 && board[x + 1][y] != '#' && visited_board[x + 1][y] == false {
                distance_board[x + 1][y] = min(distance_board[x + 1][y], distance_board[x][y] + 1);
                next_nodes.insert((x + 1, y));
            }

            if y < MAX_X_Y - 1 && board[x][y + 1] != '#' && visited_board[x][y + 1] == false {
                distance_board[x][y + 1] = min(distance_board[x][y + 1], distance_board[x][y] + 1);
                next_nodes.insert((x, y + 1));
            }

            visited_board[x][y] = true;
        }

        let total = distance_board[MAX_X_Y - 1][MAX_X_Y - 1];

        if total == i32::MAX {
            println!("Bad point is {:?}", failable_point);
            return;
        }
    }
}
