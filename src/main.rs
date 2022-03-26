extern crate ncurses;

use ncurses::*;

fn main()
{
  /* If your locale env is unicode, you should use `setlocale`. */
  // let locale_conf = LcCategory::all;
//    setlocale(locale_conf, "pl_PL.UTF-8"); // if your locale is like mine(zh_CN.UTF-8).
 let x = 5;
  /* Start ncurses. */
initscr();
  /* Print to the back buffer. */
addstr("Hello, world!\n");
  /* Print some unicode(Chinese) string. */
  /* Update the screen. */
refresh();

let x = x+1;
let s: String = x.to_string();
let ss: &str = &s;
addstr(ss);
   
  /* Wait for a key press. */
 let mut quit = false;
 while !quit {
    
    let key = getch();
    match key as u8 as char {
    'q'=>quit = true,
    'h'=> {
        addstr("hello");
    },
    _=>{}
    
 }
 
 }

  endwin();
}
