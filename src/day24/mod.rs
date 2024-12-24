mod step1;
mod step2;

use crate::util;

const DATA: &str = "src/day24/data";

pub fn execute() {
    let data = util::read_file(DATA);
    step1::execute(&data);
    step2::execute(&data);
    // DATA2 is ran with the fixed values for tpk, wkb, z27, kcd, z07, and shj
    let data = util::read_file(format!("{}2", DATA).as_str());
    step2::execute(&data);
}
