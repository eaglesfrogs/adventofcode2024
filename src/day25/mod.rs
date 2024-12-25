mod step1;

use crate::util;

const DATA: &str = "src/day25/data";

pub fn execute() {
    let data = util::read_file(DATA);
    step1::execute(&data);
}
