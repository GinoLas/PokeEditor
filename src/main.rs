use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

use crate::entities::pc::Pc;
use crate::utils::{*};
mod utils;
mod entities;

fn main(){
    println!("Welcome to Pokemon Black editor!");
    
    let mut savefile =  match File::open("pokemon_nero.sav"){
        Ok(file) => {
            println!("File opened!");
            file
        },
        Err(err)=>{
            println!("Error opening file : {err}");
            return
        }
    };

    // dump_hex_region(&mut savefile, BOX_NAMES_OFFSET, BOX_NAMES_SIZE);
    // dump_hex_region(&mut savefile, TRAINER_CARD_OFFSET, TRAINER_CARD_SIZE);

    let mut file_out = match File::create("boxes.txt"){
        Ok(file) => file,
        Err(err)=>{
            println!("Error creating file : {err}");
            return
        }
     };

    // for i in 0..BOX_NUMBER{
    //     let mut _err = write!(& mut file_out,"------------ Box {i} ------------\n");
    //     _err = dump_hex_region(&mut savefile, BOXES_OFFSET + (i * BOX_SIZE) as u64, BOX_SIZE ,& mut file_out);
    // }

    // dump_hex_region(&mut savefile, BOX_NAMES_OFFSET, BOX_NAMES_SIZE, file_out);

    let mut box_data = [0u8;BOX_DATA_SIZE];
    savefile.seek(std::io::SeekFrom::Start(0));
    savefile.read_exact(&mut box_data);

    let pc = Pc::from_bytes(box_data);

    for i in 0..24 {
            write!{file_out,"{:?}\n",pc.boxes[i]};
    }



    println!("End of operations!");





}
