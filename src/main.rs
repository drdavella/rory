extern crate clap;
use clap::{App, Arg};


fn main() {

    let input_arg =
        Arg::with_name("ROM_FILE")
             .help("Input ROM file name")
             .required(true)
             .index(1);

    let debug_arg =
        Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Run emulator with debug output");

    App::new("rory")
        .version("0.1.0")
        .about("GameBoy emulator")
        .author("Dan D'Avella")
        .arg(input_arg)
        .arg(debug_arg)
        .get_matches();
}
