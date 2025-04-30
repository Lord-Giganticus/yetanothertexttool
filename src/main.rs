#![allow(dead_code)]

mod bmg;
mod font;
mod util;

use bmg::BMG;
use binrw::prelude::*;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = Cursor::new(std::fs::read("Message.bmg")?);
    let bmg = BMG::read(&mut data)?;
    let inf1 = bmg.get_inf1().unwrap();
    let dat1 = bmg.get_dat1().unwrap();
    let flw1 = bmg.get_flw1().unwrap();
    let mut converted = vec![false; flw1.nodenum as usize];
    for i in 0..inf1.entrynum as usize {
        let entry = inf1.entries[i];
        println!("{}", entry.get_info());
        if entry.has_text(dat1) {
            println!("{}", inf1.get_string(i, dat1))
        }
        if entry.has_flow(i as u16, flw1) {
            let txt = flw1.findnode(i as u16, &mut converted)?;
            if !txt.is_empty() {
                println!("[beginflow]\n{txt}[endflow]")
            }
        }
    }
    for i in 0..converted.len() {
        if !converted[i] {
            let txt = flw1.findnode(i as u16, &mut converted)?;
            if !txt.is_empty() {
                println!("[beginflow]\n{txt}[endflow]")
            }
        }
    }
    Ok(())
}
