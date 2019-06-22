extern crate clap;
use clap::{App, Arg};
use std::io::Read;
use std::fs::File;
use std::error::Error;

mod cpu;


fn read_file(filename: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(filename)?;

    let mut buffer = vec![];
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

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

    let matches =
        App::new("rory")
            .version("0.1.0")
            .about("GameBoy emulator")
            .author("Dan D'Avella")
            .arg(input_arg)
            .arg(debug_arg)
            .get_matches();

    /* this is safe to unwrap since it is a required argument */
    let filename = matches.value_of("ROM_FILE").unwrap();
    let rom_array = match read_file(&filename) {
        Ok(result) => result,
        Err(error) => panic!("failed to read ROM file: {}", error)
    };

    cpu::emulate(rom_array);
}
