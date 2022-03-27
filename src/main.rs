extern crate ncurses;

use ncurses::*;

//Rodio lib
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {

//locale
let locale_conf = LcCategory::all;
setlocale(locale_conf, "en_US.UTF-8");

//pointless
let mut x = 0;

//converting int to &str
fn convert(x: u32) {
    let s: String = x.to_string();                                              
    let ss: &str = &s;
    addstr(ss);
    refresh();
}

//init ncurses-rs
initscr();
noecho();

//prints &str
addstr("Hello, world!\n");

//update
refresh();

x += x+1;
convert(x);
let mut quit = false;
while !quit {
    
    let key = getch();
    match key as u8 as char {
    'q'=>quit = true,
    'h'=> {
        addstr("\nhello");
   },
   'c'=> {
         clear();
   },
    _=>{}
    
    }
}
 
endwin();
}
