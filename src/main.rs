extern crate ncurses;

use ncurses::*;

fn main()
{
  /* If your locale env is unicode, you should use `setlocale`. */
  // let locale_conf = LcCategory::all;
setlocale(locale_conf, "pl_PL.UTF-8"); // if your locale is like mine(zh_CN.UTF-8).

  /* Start ncurses. */
  initscr();

  /* Print to the back buffer. */
  addstr("Hello, world!");

  /* Print some unicode(Chinese) string. */
  // addstr("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。");

  /* Update the screen. */
  refresh();

  /* Wait for a key press. */
  getch();

  /* Terminate ncurses. */
  endwin();
}