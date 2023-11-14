//! A tool for serializing the layout struct into toml to see if it's
//! usable by humans.

mod layout2;
mod layout_elements;

use layout2::*;
use layout_elements::*;

use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let layout = HudLayout2::new();
    let buf = toml::to_string_pretty(&layout)?;
    println!("{buf}");

    let buf = fs::read_to_string("./example.toml")?;
    let example = toml::from_str::<HudLayout2>(&buf)?;
    println!("Read the example layout successfully.");

    Ok(())
}
