#![allow(dead_code)]
#![allow(unused_imports)]

extern crate rand;
extern crate termion;
#[macro_use]
extern crate prettytable;
use prettytable::format::Alignment;
use prettytable::{color, Attr};
use rand::Rng;
mod terrain_selector;
use prettytable::{Cell, Row, Table};
use std::char;
use std::io::{stdin, stdout, Write};

const BOARD_HEIGHT: usize = 8;
const BOARD_WIDTH: usize = 12;

pub struct Terrain {
    name: String,
    rule_set: String,
    radius: u32,
}

impl Terrain {
    pub fn new() -> Terrain {
        Terrain {
            name: String::from("example name"),
            rule_set: String::from("example rule set"),
            radius: 1,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn rule_set(&self) -> &str {
        &self.rule_set
    }
    pub fn radius(&self) -> u32 {
        self.radius
    }
}

pub struct GridSquare {
    //Does it have a terrain piece?
    has_terrain: bool,
    terrain_piece: Terrain,
    terrain_id: char,
}

impl GridSquare {
    pub fn new() -> GridSquare {
        GridSquare {
            has_terrain: false,
            terrain_piece: Terrain::new(),
            terrain_id: 'x',
        }
    }
    pub fn place_terrain(&mut self, terrain: Terrain, id: char) -> bool {
        if self.has_terrain == true {
            return false;
        }
        self.has_terrain = true;
        self.terrain_piece = terrain;
        self.terrain_id = id;
        return true;
    }
}

fn main() {
    //let mut rng = rand::thread_rng();
    //let die = rand::thread_rng().gen_range(0, 100);
    let mut board = buildboard();
    let number_of_terrain_pieces = die_roll(2, 6) + 4;
    let mut terrain_collection: Vec<Terrain> =
        Vec::with_capacity(number_of_terrain_pieces as usize);

    terrain_selector::terrain_selector::choose_terrain(&mut terrain_collection);
    place_terrain_pieces(&mut terrain_collection, &mut board);
    let mut table = Table::new();

    for vector in board {
        let mut row = Row::empty();
        for square in &vector {
            let mut new_cell: Cell = if square.has_terrain {
                Cell::new_align(&square.terrain_id.to_string(), Alignment::CENTER).with_hspan(3)
            } else {
                Cell::new("x").with_hspan(3)
            };
            new_cell.align(Alignment::CENTER);
            row.add_cell(new_cell);
        }
        table.add_row(row);
        //println!("iterating through the board");
    }

    table.printstd();

    //let table = ptable!(["  ", "x", "x"], ["7", "  ", "x"], ["x", "x", "  "]);

    //println!("Number of terrain pieces: {}", number_of_terrain_pieces);

    //make a terrain selector. Give it a method that takes a length
    //and then returns an array
    //have an iterator push all of those onto the terrain_collection vec

    //use the random number generator to slect random x,y co-ordinates
    //and alter the GridSquare's field to contain the terrain piece
    //reroll if necessary

    //println!("Integer: {}", rng.gen_range(0, 10));

    //guarentee();
}

fn place_terrain_pieces(terrain_pieces: &mut Vec<Terrain>, board: &mut Vec<Vec<GridSquare>>) {
    //println!("Allocating {} pieces of terrain", terrain_pieces.capacity());
    for i in 0..terrain_pieces.capacity() {
        //println!("Allocating next piece");

        board[rand::thread_rng().gen_range(0, 12)][rand::thread_rng().gen_range(0, 8)]
            .place_terrain(
                terrain_pieces.pop().unwrap(),
                char::from_digit(i as u32, 10).unwrap(),
            );
    }

    //
}

fn buildboard() -> Vec<Vec<GridSquare>> {
    let mut board: Vec<Vec<GridSquare>> = (0..BOARD_WIDTH)
        .map(|_| Vec::with_capacity(BOARD_HEIGHT))
        .collect();
    for vector in &mut board {
        for _i in 0..8 {
            vector.push(GridSquare::new());
        }
    }
    return board;
}

pub fn die_roll(rolls: u32, sides: u32) -> u32 {
    let mut total = 0;
    for _i in 0..rolls {
        let die = rand::thread_rng().gen_range(0, sides + 1);
        total += die;
    }
    return total;
}

fn guarentee() {
    assert!(true);
    //test default values of terrain
    let terra_example_1 = Terrain::new();
    assert_eq!("example name", terra_example_1.name());
    assert_eq!("example rule set", terra_example_1.rule_set());
    assert_eq!(1, terra_example_1.radius());

    //test assigned values of creating terrain
    let terra_example_2 = Terrain {
        name: String::from("non_default name"),
        rule_set: String::from("non-default rule set"),
        radius: 2,
    };

    assert_eq!("non_default name", terra_example_2.name());
    assert_eq!("non-default rule set", terra_example_2.rule_set());
    assert_eq!(2, terra_example_2.radius());

    let cell_1 = GridSquare::new();
    assert_eq!(false, cell_1.has_terrain);
    assert_eq!("example name", cell_1.terrain_piece.name());
    assert_eq!("example rule set", cell_1.terrain_piece.rule_set());
    assert_eq!(1, cell_1.terrain_piece.radius());

    let mut cell_2 = GridSquare::new();
    let terra_example_3 = Terrain {
        name: String::from("non_default name"),
        rule_set: String::from("non-default rule set"),
        radius: 2,
    };
    cell_2.place_terrain(terra_example_3, '1');

    assert_eq!(true, cell_2.has_terrain);

    assert_eq!("non_default name", cell_2.terrain_piece.name());
    assert_eq!("non-default rule set", cell_2.terrain_piece.rule_set());
    assert_eq!(2, cell_2.terrain_piece.radius());
    assert_eq!('1', cell_2.terrain_id);

    let mut board = buildboard();
    for vector in &board {
        assert_eq!(8, vector.capacity());
        //println!("Capacity of vector: {}", vector.capacity());
        for GridSquare in vector {
            //println!("testing GridSquare");

            assert_eq!(false, GridSquare.has_terrain);
            assert_eq!("example name", GridSquare.terrain_piece.name());
            assert_eq!("example rule set", GridSquare.terrain_piece.rule_set());
            assert_eq!(1, GridSquare.terrain_piece.radius());
        }
    }
    assert_eq!(false, board[1][1].has_terrain);
    assert_eq!("example name", board[1][1].terrain_piece.name());
    assert_eq!("example rule set", board[1][1].terrain_piece.rule_set());
    assert_eq!(1, board[1][1].terrain_piece.radius());
}

//72 * 48 = 12 * 8, 6*6 inch cells to make the game board
