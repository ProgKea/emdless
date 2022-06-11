extern crate ncurses;

use ncurses::*;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn read_markdown_file() -> String {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\t{} <Markdown file>", args[0]);
        println!("Example:\n\t{} examples/ex_3.md", args[0]);
        panic!("Exiting");
    }
    if !args[1].ends_with(".md") {
        println!("Usage:\n\t{} <Markdown file>", args[0]);
        println!("Example:\n\t{} examples/ex_3.md", args[0]);
        panic!("Exiting");
    }
    if !Path::new(&args[1]).exists() {
        println!("Usage:\n\t{} <Markdown file>", args[0]);
        println!("Example:\n\t{} examples/ex_3.md", args[0]);
        panic!("File does not exist");
    }

    return fs::read_to_string(&args[1]).unwrap();
}

struct Pager {
    text: String,
    prev_ch: char,
    ch: char,
    is_heading_line: bool,
}

impl Pager {
    pub fn print_heading_lines(&mut self) {
        for ch in self.text.chars() {
            self.ch = ch;
            if ch == '#' && (self.prev_ch == '\0' || self.prev_ch == '\n') {
                self.is_heading_line = true;
            }
            if ch != '#' && (self.prev_ch == '\0' || self.prev_ch == '\n') {
                self.is_heading_line = false;
            }
            if self.is_heading_line {
                addch(ch as chtype);
            }
            self.prev_ch = ch;
        }
    }
}

fn main() {
    let markdown_file = read_markdown_file();
    let mut pager = Pager {
        text: markdown_file.clone(),
        prev_ch: '\0',
        ch: markdown_file.chars().nth(1).unwrap(),
        is_heading_line: false
    };

    print!("{}", markdown_file);

    initscr();
    noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    loop {
        let key = getch();
        match char::from_u32(key as u32).unwrap() {
            'q' => {
                break;
            }
            _ => {}
        }
    }

    endwin();
}
