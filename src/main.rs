#![warn(clippy::all)]

use postgres::{Client, NoTls};

mod data;
use data::Fgo;

fn main() {
    let fgo = Fgo::from_file("./data/fgo.json").unwrap();

    let mut client = Client::connect("postgres://aqian@localhost/aqian", NoTls).unwrap();
    Fgo::reset_schema(&mut client).unwrap();
    fgo.save(&mut client).unwrap();
}
