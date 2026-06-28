
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