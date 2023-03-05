#![feature(cursor_remaining)]
extern crate console_error_panic_hook;
extern crate wasm_bindgen;

// Imports
use wasm_bindgen::prelude::*;
use polars::prelude::*;
use std::io::Cursor;
use js_sys;

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

    pub fn get_shape(&self) -> js_sys::Array{
        let x = vec![self.data.height() as u32, self.data.width() as u32];
        return x.into_iter().map(JsValue::from).collect();
    }

    pub fn get_cell(&self, row_index: usize, column_index: usize) -> String{
        let row = self.data.get(row_index).unwrap();
        let cell = row.get(column_index).unwrap().to_string();

        return cell.clone().replace('"', "").to_string()
    }

    pub fn set_cell(&mut self, row_index: usize, column_index: usize, data: String){

        let col_name = self.get_header(column_index);
        let mut s1 = self.data.clone().select_at_idx(column_index.clone()).unwrap().utf8().unwrap().clone();

        s1 = s1.clone().into_iter()
            .enumerate()
            .map(|(i,v)| match i {
                idx if idx == row_index => data.to_string(),
                _ => v.unwrap().to_string()
            }).collect();

        self.data.replace_at_idx(column_index, s1).unwrap();
        self.set_header(column_index, col_name) //This is dumb but the replace_at_idx is replacing the column name
    }

    pub fn get_header(&self, row_index: usize) -> String{
        return self.data.get_column_names()[row_index].to_string();
    }

    pub fn set_header(&mut self, row_index: usize, data: String){
        // Get the current column names
        let current_columns = &mut self.data.get_column_names_owned();

        // Update the current column names with the new value
        current_columns[row_index] = data;

        // Set the new column names
        self.data.set_column_names(&current_columns).unwrap();
    }



    pub fn to_csv(&self) -> String{

        let mut output = Vec::with_capacity(self.width.clone());
        let mut row_str = Vec::with_capacity(self.height.clone());

        //Iterate over header cells and create the header for
        for i in 0..self.width.clone(){
            row_str.push(self.get_header(i));
        }
        output.push(row_str.join(","));
        row_str = Vec::with_capacity(self.height.clone());

        // Iterate over the data cells and create csv rows
        for i in 0..self.height.clone(){
            for j in 0..self.width.clone(){
                row_str.push(self.get_cell(i,j));
            }
            output.push(row_str.join(","));
            row_str = Vec::with_capacity(self.height.clone());
        }

        return output.join("\n")
    }

    pub fn load_csv(&mut self, buff: &[u8]) {
        let shorts = unsafe {buff.align_to::<u8>().1};

        let cursor = Cursor::new(shorts);

        // Causes alot of logs to be generated in console for some reason
        let lf = CsvReader::new(cursor)
            .has_header(true)
            .with_ignore_parser_errors(true)
            .finish();

        let matched_df = match lf {
            Ok(df) => df,
            Err(e) => match e {
                polars::error::PolarsError::NoData(polars::error::ErrString::Borrowed("empty csv")) => DataFrame::default(),
                _ => panic!("Problem opening the file: {:?}", e)
            }
        };

        //Resize class instance
        self.height = matched_df.height();
        self.width = matched_df.width();

        // Update each of the cells with the new data
        self.data = matched_df.clone();

    }

}