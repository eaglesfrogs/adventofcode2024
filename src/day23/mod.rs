mod step1;
mod step2;

use crate::util;

const DATA: &str = "src/day23/data";

pub fn execute() {
    let data = util::read_file(DATA);
    step1::execute(&data);
    step2::execute(&data);
}
