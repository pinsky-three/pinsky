use std::error::Error;

use wfc_rs::{Wfc, WfcImage};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let image = WfcImage::from_file("images/pinsky_tile_32x32.png").unwrap();

    let mut wfc = Wfc::overlapping(128, 128, image, 3, 3, false, false, false, false).unwrap();

    wfc.run(None, None)?;

    wfc.export("output_1.png")?;

    Ok(())
}
