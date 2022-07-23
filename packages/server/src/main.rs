extern crate database;

use database::collections::deduction::Deduction;

fn main() {
  let deduc = Deduction::new(
    "".to_string(),
    0,
    "".to_string(),
    0.0,
    0,
    0,
    "".to_string(),
  );
  deduc.register();
}
