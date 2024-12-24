use std::collections::{HashMap, HashSet};

//https://www.electronics-lab.com/article/binary-adder/
//This one i did by finding the weirdness and then drawing out manually in draw.io and finding the specific discrepencies
//the answer is kcd,pfn,shj,tpk,wkb,z07,z23,z27
pub fn execute(data: &Vec<String>) {
    let mut gate_vector_map = HashMap::<String, Vec<(String, char, String, String)>>::new();

    let mut read_values = true;

    for line in data {
        if read_values {
            if line.len() == 0 {
                read_values = false;
            }
        } else {
            let mut tokens = line.split_ascii_whitespace();
            let t1 = String::from(tokens.next().unwrap());
            let t2 = tokens.next().unwrap();
            let t3 = String::from(tokens.next().unwrap());
            tokens.next().unwrap(); //Arrow
            let t4 = String::from(tokens.next().unwrap());
            let operator: char;

            if t2 == "AND" {
                operator = '&';
            } else if t2 == "OR" {
                operator = '|';
            } else {
                operator = '^';
            }

            let gate_vector = gate_vector_map.entry(t1.clone()).or_insert(Vec::new());
            gate_vector.push((t1.clone(), operator, t3.clone(), t4.clone()));

            let gate_vector = gate_vector_map.entry(t3.clone()).or_insert(Vec::new());
            gate_vector.push((t1.clone(), operator, t3.clone(), t4.clone()));
        }
    }

    let mut processed = HashSet::<(String, char, String, String)>::new();

    for (_, gates) in &gate_vector_map {
        for gate in gates {
            if processed.contains(gate) {
                continue;
            }

            processed.insert(gate.clone());
            let (v1, op, v2, o) = gate;

            // Skipping the ones that had the output directly to Z.  It didn't impact the result so i just never worked around it.  Caused errors otherwise though.
            if o.starts_with("z") {
                continue;
            }

            if v1.starts_with("x") || v2.starts_with("x") {
                let num_x = &v1[1..];
                let num_y = &v2[1..];

                // X# and Y# should always be paired - never occurred
                if num_x != num_y {
                    println!("X and Y aren't paired: {:?}", gate)
                }

                if *op == '^' {
                    let x_y_xor = gate_vector_map.get(o).unwrap();

                    for next_gate in x_y_xor {
                        let (_, next_op, _, next_o) = next_gate;
                        if *next_op == '^' {
                            // Checks that the following XOR after the X# ^ Y# results in Z#. Also checks that Z# is the right number
                            if next_o.starts_with('z') == false {
                                println!("2nd XOR doesn't result in Z: gate [{:?}] next_gate [{:?}]", gate, next_gate);
                            } else {
                                let num_z = &next_o[1..];
                                if num_x != num_z {
                                    println!("X and Y pointing to wrong Z");
                                }
                            }
                        }
                    }
                }
            }

            if *op == '|' {
                // OR gates always lead to a XOR and AND
                let mut has_xor = false;
                let mut has_and = false;
                let or_gates = gate_vector_map.get(o).unwrap();

                if or_gates.len() == 2 {
                    for (_, or_op, _, _) in or_gates {
                        if *or_op == '&' {
                            has_and = true;
                        } else if *or_op == '^' {
                            has_xor = true;
                        }
                    }

                    if (has_xor && has_and) == false {
                        println!("{}", o);
                    }
                } else {
                    println!("and outputting wrong number of gates from OR. gate [{:?}] and_gates [{:?}]", gate, or_gates);
                }
            } else if *op == '&' {
                // AND gates always lead to an OR except for dhb which is the initial carry bit
                let and_gates = gate_vector_map.get(o).unwrap();

                if and_gates.len() == 1 {
                    let mut has_or = false;

                    for (_, or_op, _, _) in and_gates {
                        if *or_op == '|' {
                            has_or = true;
                        }
                    }

                    if (has_or) == false {
                        println!("no OR. gate [{:?}] and_gates [{:?}]", gate, and_gates);
                    }
                } else {
                    println!("and outputting to more than just 1 OR. gate [{:?}] and_gates [{:?}]", gate, and_gates);
                }
            } else if *op == '^' {
                // XOR gates always lead to another XOR and AND (or a z output but we excluded those above)
                let mut has_xor = false;
                let mut has_and = false;
                let xor_gates = gate_vector_map.get(o).unwrap();

                if xor_gates.len() == 2 {
                    for (_, xor_op, _, _) in xor_gates {
                        if *xor_op == '&' {
                            has_and = true;
                        } else if *xor_op == '^' {
                            has_xor = true;
                        }
                    }

                    if (has_xor && has_and) == false {
                        println!("XOR should output to AND and XOR.  gate[{:?}]. xor_gates[{:?}]", gate, xor_gates);
                    }
                } else {
                    println!("and outputting wrong number of gates from XOR. gate [{:?}] and_gates [{:?}]", gate, xor_gates);
                }
            }
        }
    }

    println!("Step 1 Total ");
}
