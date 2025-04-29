#![allow(dead_code)]

mod bmg;
mod font;
mod util;

use bmg::BMG;
use binrw::prelude::*;
use std::io::*;

fn main() -> BinResult<()> {
    let mut data = Cursor::new(std::fs::read("Message.bmg")?);
    let bmg = BMG::read(&mut data)?;
    let inf1 = bmg.get_inf1().unwrap();
    let dat1 = bmg.get_dat1().unwrap();
    for i in 0..inf1.entries.len() {
        let entry = inf1.entries[i];
        println!("{}", entry.get_info());
        let message = inf1.get_string(i, dat1);
        println!("{}", message);
    }
    Ok(())
}
