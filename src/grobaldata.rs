use crate::{environment::*, evaluation::*, Search::*};
use std::{
    collections::{HashMap, HashSet},
    vec,
};

pub struct GrobalData {
    weight: [f64; Evaluation::WEIGHT_COUNT as usize],
    hole_eval: Vec<[f64; 3]>,
    heights_without_ido: Vec<Vec<i32>>,
    row_height: Vec<[i32; Environment::FIELD_WIDTH as usize]>,
    vec_field: Vec<Vec<[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT]>>,
    hashmap_long: Vec<HashMap<i64, Pattern>>,
    hashset_long: Vec<HashSet<i64>>,
}

impl GrobalData {
    pub fn new(length: u32) -> Self {
        let mut obj = GrobalData {
            weight: [0.0; Evaluation::WEIGHT_COUNT as usize],
            hole_eval: Vec::new(),
            heights_without_ido: Vec::new(),
            row_height: Vec::new(),
            vec_field: Vec::new(),
            hashmap_long: Vec::new(),
            hashset_long: Vec::new(),
        };

        for _i in 0..length {
            obj.heights_without_ido.push(Vec::<i32>::new());
            obj.hole_eval.push([0.0; 3]);
            obj.row_height.push([0; Environment::FIELD_WIDTH]);
            obj.vec_field.push(Vec::<
                [bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT],
            >::new());
            obj.hashmap_long.push(HashMap::<i64, Pattern>::new());
            obj.hashset_long.push(HashSet::<i64>::new());
        }

        obj
    }

    pub fn get_heights_without_ido_vec(&mut self, index: i32) -> &mut Vec<Vec<i32>> {
        &mut self.heights_without_ido
    }

    pub fn get_array_rowheight(
        &mut self,
        length: u32,
        index: usize,
    ) -> &mut [i32; Environment::FIELD_WIDTH] {
        &mut self.row_height[index as usize]
    }
    pub fn get_vec_field(
        &mut self,
        index: usize,
    ) -> &mut Vec<[bool; Environment::FIELD_WIDTH * Environment::FIELD_HEIGHT]> {
        &mut self.vec_field[index]
    }
    pub fn get_hashmap_long(&mut self, index: usize) -> &mut HashMap<i64, Pattern> {
        &mut self.hashmap_long[index]
    }
    pub fn get_hashset_long(&mut self, index: usize) -> &mut HashSet<i64> {
        &mut self.hashset_long[index]
    }
}
