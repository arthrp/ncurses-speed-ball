#![allow(unused_parens)]

extern crate ncurses;

use ncurses::*;
use std::{thread, time};

fn main() {
    initscr();
    keypad(stdscr(), true);
    noecho();
    halfdelay(1);

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let (mut x, mut y, mut max_x, mut max_y) = (0,0,0,0);
    let (mut speed_x, mut speed_y) = (1,1);
    let time_to_sleep = time::Duration::from_millis(50);
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let mut ch: i32;

    loop {
        ch = getch();

        adjust_speed(&ch, &mut speed_x);

        clear();

        mvprintw(y,x,"0");
        thread::sleep(time_to_sleep);

        refresh();

        if(x > max_x - 2){
            speed_x = -1;
        }
        else if(x < 1){
            speed_x = 1;
        }

        if(y > max_y - 2){
            speed_y = -1;
        }
        else if(y < 1){
            speed_y = 1;
        }

        x += speed_x;
        y += speed_y;
    }
}

fn adjust_speed(ch: &i32, speed_x: &mut i32){
    match *ch {
        KEY_RIGHT => *speed_x += 1,
        KEY_LEFT => *speed_x -= 1,
        _ => ()
    }
}
