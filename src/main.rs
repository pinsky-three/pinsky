use std::error::Error;

use wfc_rs::{Wfc, WfcImage};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let image = WfcImage::from_file("images/pinsky_tile_8x8.png").unwrap();

    let mut wfc = Wfc::overlapping(128, 128, image, 2, 2, true, true, true, true).unwrap();

    wfc.run(None, None)?;

    wfc.export("output.png")?;

    Ok(())
}
