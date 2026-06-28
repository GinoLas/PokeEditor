use std::array;

use crate::utils::{POKEMON_CHECKSUM, prng};


#[derive(Clone,Copy,Debug)]
pub struct PcPokemon{
    bytes : [u8;136],
}

impl PcPokemon{

    pub fn from_bytes(bytes : [u8;136]) -> Self{
        Self{bytes}
    }

    pub fn to_bytes(&self) -> [u8;136]{
        self.bytes
    }
}

impl Default for PcPokemon{
    fn default() -> Self{
        Self{bytes : [0u8 ; 136]}
    }
}

pub fn decrypt_pokemon(bytes : &[u8;136]) -> [u8;136]{
    let mut unencrypted_pokemon = [0u8;136];

    unencrypted_pokemon[0..8].copy_from_slice(&bytes[0..8]); // I primi 8 byte sono non criptati li copio come sono 

    let mut seed: u32 = u16::from_le_bytes(bytes[POKEMON_CHECKSUM..POKEMON_CHECKSUM + 2].try_into().unwrap()) as u32;

    for i in (8..136).step_by(2){
        seed = prng(seed);
        let key : u16 = (seed >> 16) as u16;
        let encrypted_bytes : u16 = u16::from_le_bytes(bytes[i..i+2].try_into().unwrap());
        let unencrypted_bytes : u16 = encrypted_bytes ^ key;
        let unencrypted_array : [u8;2] = unencrypted_bytes.to_le_bytes();
        unencrypted_pokemon[i..i+2].copy_from_slice(&unencrypted_array);
    };

    unencrypted_pokemon
}