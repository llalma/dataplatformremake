#![feature(cursor_remaining)]
extern crate console_error_panic_hook;
extern crate wasm_bindgen;

// Imports
use wasm_bindgen::prelude::*;
use polars::prelude::*;
use std::io::Cursor;

pub use wasm_bindgen_rayon::init_thread_pool;

/*
Structs
*/
#[wasm_bindgen]
pub struct Grid {
    height: usize,
    width: usize,
    data: DataFrame,
}

#[wasm_bindgen]
impl Grid {

    #[wasm_bindgen(constructor)]
    pub fn new(size_x: usize, size_y: usize) -> Self {
        console_error_panic_hook::set_once();   //For better error messages in the console
        Self {
            height: size_x,
            width: size_y,
            data : df!("Index" => &["0", "1", "2", "4"],
                       "Fruit" => &["Apple", "Apple", "Pear", "dog"],
                       "Color" => &["Red", "Yellow", "Green", "Blue"]).unwrap()

        }
    }

    pub fn get_cell(&self, row_index: usize, column_index: usize) -> String{
        let row = self.data.get(row_index).unwrap();
        let cell = row.get(column_index).unwrap().to_string();

        return cell.clone().replace('"', "").to_string()
    }

    pub fn set_cell(&mut self, row_index: usize, column_index: usize, data: String){

        let mut s1 = self.data.clone().select_at_idx(column_index.clone()).unwrap().utf8().unwrap().clone();

        s1 = s1.clone().into_iter()
            .enumerate()
            .map(|(i,v)| match i {
                idx if idx == row_index => data.to_string(),
                _ => v.unwrap().to_string()
            }).collect();

        self.data.replace_at_idx(column_index, s1).unwrap();
    }

    pub fn to_csv(&self) -> String{

        let mut output = Vec::with_capacity(self.width.clone());
        let mut row_str = Vec::with_capacity(self.height.clone());

        for i in 0..self.height.clone(){

            for j in 0..self.width.clone(){
                row_str.push(self.get_cell(i,j));
            }
            output.push(row_str.join(","));
            row_str = Vec::with_capacity(self.height.clone());
        }

        return output.join("\n")
    }

    pub fn load_csv(&mut self, buff: &[u8]) -> String {
        let shorts = unsafe {buff.align_to::<u8>().1};

        let cursor = Cursor::new(shorts);

        let lf = CsvReader::new(cursor).finish().unwrap().lazy();

        &self.set_cell(1, 1, "test".to_owned());

        return lf.describe_plan();
    }

}