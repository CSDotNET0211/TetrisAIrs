use crate::{beemsearch::SearchedPattern, environment::*, evaluation::*};
use std::{
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut},
};

pub struct GrobalData {
    pub weight: [f64; Evaluation::WEIGHT_COUNT as usize],
    pub data: Vec<Data>,
}

impl GrobalData {
    pub fn new(length: u32) -> Self {
        let mut obj = GrobalData {
            weight: [0.0; Evaluation::WEIGHT_COUNT as usize],
            data: Vec::new(),
        };

        for _i in 0..length {
            obj.data.push(Data::new());
        }

        obj
    }
}

pub struct Data {
    pub hole_eval: [f64; 3],
    pub heights_without_ido: Vec<i32>,
    pub row_height: [i32; Environment::FIELD_WIDTH as usize],
    pub vec_field: Vec<[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT]>,
    pub searched_data: HashMap<i64, SearchedPattern>,
    pub passed_tree_route_set: HashSet<i64>,
}

impl Index<i32> for Data {
    fn index(&self, index: i32) -> &Data {
        &self[index]
    }

    type Output = Data;
}

impl IndexMut<i32> for Data {
    fn index_mut(&mut self, index: i32) -> &mut Data {
        &mut self[index]
    }
}

impl Data {
    pub fn new() -> Self {
        Data {
            hole_eval: [0.0; 3],
            heights_without_ido: Vec::<i32>::new(),
            row_height: [0; Environment::FIELD_WIDTH],
            vec_field: Vec::<[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT]>::new(),
            searched_data: HashMap::<i64, SearchedPattern>::new(),
            passed_tree_route_set: HashSet::<i64>::new(),
        }
    }
}
