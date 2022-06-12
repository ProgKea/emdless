extern crate ncurses;

use ncurses::*;
use std::env;
use std::fs;
use std::path::Path;
use strip_markdown::strip_markdown;

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

#[derive(Debug)]
struct Item {
    heading: String,
    child: String,
    shown_item: String,
    show_child: bool,
}

#[derive(Debug)]
struct Pager {
    text: String,
    items: Vec<Item>,
    index: usize,
}

impl Pager {
    pub fn new(text: &str) -> Pager {
        let mut pager = Pager {
            text: text.to_owned(),
            items: Vec::new(),
            index: 1,
        };
        pager.add_items();

        return pager;
    }

    fn add_items(&mut self) {
        let text_vec: Vec<&str> = self.text.split('\n').collect();
        let mut heading_exists = false;
        let mut self_item_index: usize = 0;

        for line in text_vec {
            if line.starts_with('#') {
                self.items.push(Item {
                    heading: line.to_string(),
                    child: String::new(),
                    shown_item: line.to_string(),
                    show_child: false,
                });
                self_item_index += 1;
                heading_exists = true;
            }
            if heading_exists {
                if !line.starts_with('#') {
                    self.items[self_item_index - 1].child += line;
                    self.items[self_item_index - 1].child.push('\n');
                }
                if line.starts_with('#')
                    && line.to_owned() != self.items[self_item_index - 1].heading
                {
                    break;
                }
            }
        }
    }

    pub fn print_items(&mut self) {
        for (i, item) in self.items.iter_mut().enumerate() {
            if i + 1 == self.index {
                attron(A_REVERSE() | A_BOLD());
            } else {
                attroff(A_REVERSE() | A_BOLD());
            }
            if item.show_child {
                if !item.shown_item.ends_with(&item.child) {
                    item.shown_item.push_str(&item.child);
                }
                addstr(&strip_markdown(&item.shown_item.to_string()));
                addstr("\n\n");
            } else {
                addstr(&strip_markdown(&item.heading.to_string()));
            }
        }
    }

    pub fn unselect_all(&mut self) {
        for item in self.items.iter_mut() {
            item.show_child = false;
        }
    }

    pub fn go_up(&mut self) {
        if self.index > 1 {
            self.index -= 1;
        }
    }

    pub fn go_down(&mut self) {
        if self.index < self.items.len() {
            self.index += 1;
        }
    }

    pub fn select_item(&mut self) {
        if self.items[self.index - 1].show_child == false {
            self.items[self.index - 1].show_child = true;
        } else {
            self.items[self.index - 1].show_child = false;
        }
    }
}

fn main() {
    let markdown_file = read_markdown_file();
    let mut pager = Pager::new(&markdown_file);

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    loop {
        clear();
        pager.print_items();
        let key = getch();
        match char::from_u32(key as u32).unwrap() {
            'q' => break,
            'k' => pager.go_up(),
            'j' => pager.go_down(),
            '\n' => pager.select_item(),
            ' ' => pager.unselect_all(),
            _ => {}
        }
    }
    endwin();
}
