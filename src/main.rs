extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::{color, style};

struct Cell {
    //Does it have a terrain piece?
    placed: bool,

    terrain_name: String,
}

struct Terrain {
    name: String,
    rule_set: String,
    radius: u32,
}

impl Terrain {
    pub fn new() -> Terrain {
        Terrain {
            name: String::from("s: &String"),
            rule_set: String::from("s: &String"),
            radius: 1,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn rule_set(&self) -> &str {
        &self.rule_set
    }
    pub fn name(&self) -> u32 {
        &self.radius
    }
}

fn main() {}
