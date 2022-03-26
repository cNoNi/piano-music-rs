extern crate ncurses;

use ncurses::*;
int x = 5;
fn main()
{
  /* If your locale env is unicode, you should use `setlocale`. */
  // let locale_conf = LcCategory::all;
setlocale(locale_conf, "pl_PL.UTF-8"); // if your locale is like mine(zh_CN.UTF-8).

  /* Start ncurses. */
  initscr();

  /* Print to the back buffer. */
  addstr("Hello, world!");
  addstr(x);
  /* Print some unicode(Chinese) string. */
  /* Update the screen. */
  refresh(
      x = x + 1
  );

  /* Wait for a key press. */
  getch();

  /* Terminate ncurses. */
  endwin();
}