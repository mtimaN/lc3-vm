use std::fs::File;
use std::mem::size_of;

fn swap16(x: u16) -> u16 {
    (x << 8) | (x >> 8)
}

fn read_image_file(file: File) {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    origin = file.take(size_of(u16));
}

fn read_image(image_path: String) -> Result<(), ()> {
    let mut file = File::open(image_path)?;
}
