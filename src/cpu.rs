pub fn emulate(rom_array: Vec<u8>) {

    let mut index = 0;
    loop {
        index = decode(&rom_array, index);
    }
}

fn decode(rom_array: &Vec<u8>, index: usize) -> usize {
    println!("0x{:02x}", rom_array[index]);
    index + 1
}
