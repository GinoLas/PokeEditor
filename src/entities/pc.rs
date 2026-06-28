use std::array;

use crate::{utils::{*}};

use super::pokemon::PcPokemon;

/*
    Models a Pc Box, which contains 30 pokemon each.
    Each pokemon in the box is 136B long
*/
#[derive(Debug)]
pub struct PcBox{
    pub name : String,
    pub index : u8,
    pub wallpaper : u8,
    pub pokemon : [PcPokemon;30],
    pub checksum : u16
}

impl PcBox{
    pub fn from_bytes(name : Vec<u16>,index : u8,wallpaper : u8, checksum : u16, bytes : [u8;BOX_SIZE]) -> Self{
        let pokemon : [PcPokemon;30] = std::array::from_fn(|i|{
            let start = 136 * i;
            let end = start + 136;
            //I can unwrap safely since I know beforehand that every pokemon is 136 byte
            let pokemon_bytes : [u8;136] = bytes[start..end].try_into().unwrap(); 

            PcPokemon::from_bytes(pokemon_bytes)
        });
        Self { name: (String::from_utf16_lossy(&name)), index , wallpaper, pokemon, checksum}
    }
}
#[derive(Debug)]
pub struct Pc{
    pub boxes : [PcBox;24],
    pub checksum : u16
}

impl Pc{

    pub fn from_bytes(bytes : [u8;BOX_DATA_SIZE]) -> Self{
        let boxes = array::from_fn(|i|{
            let name_start = i * BOX_NAME_SIZE;
            let name_end = name_start + BOX_NAME_SIZE;
            let name_bytes = &bytes[name_start+4..name_end];
            //Serve a convertire i byte grezzi in caratteri Little Endian UTF-16
            let name_u16: Vec<u16> = name_bytes
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .take_while(|&ch| ch != 0xFFFF && ch != 0x0000)
                .collect();
            let box_start = BOXES_OFFSET + (i * BOX_STEP);
            let box_end: usize = box_start + BOX_SIZE;
            println!("Box {i} start {:x}, end {:x}",box_start,box_end);
            let wallpaper = bytes[BOX_WALLPAPER_OFFSET + i];
            let box_bytes : [u8;BOX_SIZE] = bytes[box_start..box_end].try_into().unwrap();
            let checksum_start = BOX_CHECKSUM_OFFSET + BOX_STEP*i;
            let checksum_end = checksum_start + CHECKSUM_SIZE;
            println!("Checksum {i} start {:x}, end {:x}",checksum_start,checksum_end);
            let checksum = u16::from_le_bytes(bytes[checksum_start..checksum_end].try_into().unwrap());

            PcBox::from_bytes(name_u16,i as u8,wallpaper,checksum,box_bytes)
        });

        let checksum = u16::from_le_bytes(bytes[BOX_NAMES_CHECKSUM_OFFSET..BOX_NAMES_CHECKSUM_OFFSET+2].try_into().unwrap());

        Self{boxes,checksum}
    }

}