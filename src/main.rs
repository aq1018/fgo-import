#![warn(clippy::all)]

use clap::{App, Arg};
use postgres::{Client, NoTls};

mod data;
use data::Fgo;

fn main() {
    let matches = App::new("fgo-import")
        .version("0.1")
        .about("Load FGO data from a JSON file and export to PostgreSQL database.")
        .author("Aaron Q.")
        .arg(
            Arg::with_name("dburl")
                .short("d")
                .long("dburl")
                .value_name("URL")
                .help("specify postgresql database URL")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("specify JSON file that contains FGO data.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let dburl = matches.value_of("dburl").unwrap();
    let input = matches.value_of("input").unwrap();
    let fgo = Fgo::from_file(input).unwrap();
    let mut client = Client::connect(dburl, NoTls).unwrap();

    Fgo::reset_schema(&mut client).unwrap();
    fgo.save(&mut client).unwrap();
}
