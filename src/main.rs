use std::borrow::Borrow;

mod adb;
mod utils;

fn main() {
    adb::run_adb();
}
