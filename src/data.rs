extern crate nalgebra_glm as glm;
use bevy::{
    prelude::*,
};

pub const SIZE: (usize, usize) = (100, 100);

#[derive(Component, Copy, Clone)]
pub struct Cell {
    pub alive: bool,
    x: usize,
    y: usize,
}

impl Cell {
    pub fn get_pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Resource)]
pub struct Field {
    size: (usize, usize), //Holds the width and height of the playfield
    matrix: Vec<Vec<Cell>>,
}

#[allow(unused_variables)]
impl FromWorld for Field {
    fn from_world(world: &mut World) -> Self {

        let mut matrix = vec![vec![Cell::new(0,0);SIZE.1];SIZE.0];

        for y in 0..SIZE.1 {
            for x in 0..SIZE.0 {
                matrix[x][y] = Cell::new(x, y);
            }
        }


        Field {
            size: SIZE,
            matrix: matrix,
        }
    }
}

impl Field {
    pub fn get_width(&self) -> usize {
        self.size.0
    }

    pub fn get_height(&self) -> usize {
        self.size.1
    }

    pub fn get_mat(&self) -> Vec<Vec<Cell>> {
        self.matrix.to_owned()
    }
}

impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Cell {
            alive: false,
            x: x,
            y: y,
        }    
    }
}