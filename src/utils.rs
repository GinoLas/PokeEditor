use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

/*
    Offset delle varie sezioni
*/

pub const CHECKSUM_SIZE : usize = 0x2;
pub const BOX_END_PADDING : usize = 0xE;

// ==========================================================================
// SEZIONE 1: DATO GENERALI DEI BOX (Inizio del file .sav)
// ==========================================================================

pub const BOX_NAMES_OFFSET: usize = 0x0;
pub const BOX_NAME_SIZE : usize= 0x28;
pub const BOX_NAMES_SIZE: usize = 0x3E0;
pub const BOX_NAMES_PADDING: usize = 0x1C;

pub const BOXES_OFFSET: usize = 0x400;
pub const BOX_SIZE: usize = 0xFF0; 
pub const BOX_CHECKSUM_OFFSET : usize = 0x013F2; 
pub const BOX_STEP : usize = BOX_SIZE + CHECKSUM_SIZE + BOX_END_PADDING;
pub const BOX_NUMBER: usize = 24;  
pub const BOX_WALLPAPER_OFFSET : usize = 0x3C4;

//We have to consider checksum after each block, so after box names and after each box
pub const BOX_DATA_SIZE : usize = BOX_NAMES_SIZE + CHECKSUM_SIZE + BOX_NAMES_PADDING + BOX_STEP * BOX_NUMBER;

pub const BOX_NAMES_CHECKSUM_OFFSET: usize = 0x003E2; // 2 byte

// ==========================================================================
// SEZIONE 2: BLOCK A (Dati di gioco principali)
// ==========================================================================
pub const INVENTORY_OFFSET: usize = 0x18400;
pub const INVENTORY_SIZE: usize = 0x940;

pub const PARTY_POKEMON: u64 = 0x18E00;
pub const PARTY_POKEMON_SIZE: usize = 0x528; 

pub const TRAINER_CARD_OFFSET: u64 = 0x19340;
pub const TRAINER_CARD_SIZE: usize = 0xC0;

// --- Sotto-offset interni alla Trainer Card (DA VERIFICARE) ---
pub const TRAINER_NAME_OFFSET: u64 = 0x19344; // Stringa UTF-16
pub const TRAINER_ID_OFFSET: u64 = 0x1935C;   // 2 byte
pub const TRAINER_SECRET_ID_OFFSET: u64 = 0x1935E; // 2 byte
pub const MONEY_OFFSET: u64 = 0x19360;       // 4 byte

pub const PASSERSBY_ANALYTICS_OFFSET: u64 = 0x19400;
pub const PASSERSBY_ANALYTICS_SIZE: usize = 0x900;

pub const MUSICAL_DATA_OFFSET: u64 = 0x19D00;
pub const MUSICAL_DATA_SIZE: usize = 0x300;

pub const POKEDEX_OFFSET: u64 = 0x1A000;
pub const POKEDEX_SIZE: usize = 0x540;

pub const ADVENTURE_STARTED_OFFSET: u64 = 0x1A540;
pub const ADVENTURE_STARTED_SIZE: usize = 0x10;

pub const GAME_FLAGS_OFFSET: u64 = 0x1A550;
pub const GAME_FLAGS_SIZE: usize = 0xAA0;

pub const POKE_TRANSFER_OFFSET: u64 = 0x1B000;
pub const POKE_TRANSFER_SIZE: usize = 0x340;

pub const C_GEAR_OFFSET: u64 = 0x1B340;
pub const C_GEAR_SIZE: usize = 0x16A0;

pub const PAL_PAD_OFFSET: u64 = 0x1C9E0;
pub const PAL_PAD_SIZE: usize = 0x2A0;

pub const INTRALINK_OFFSET: u64 = 0x1CC80;
pub const INTRALINK_SIZE: usize = 0x100;

pub const POKEDEX_RATING_OFFSET: u64 = 0x1CD80;
pub const POKEDEX_RATING_SIZE: usize = 0x40;

pub const MY_FILE_SYSTEM_OFFSET: u64 = 0x1CDC0;
pub const MY_FILE_SYSTEM_SIZE: usize = 0x200;

pub const BATTLE_SUBWAY_OFFSET: u64 = 0x1CFC0;
pub const BATTLE_SUBWAY_SIZE: usize = 0xBC0;

pub const BLOCK_A_CHECKSUM_OFFSET: u64 = 0x1DBFE; // 2 byte

// ==========================================================================
// SEZIONE 3: BLOCK B (Copia speculare di backup del Block A)
// Gli offset iniziano a 0x24400. La struttura interna è IDENTICA al Block A.
// ==========================================================================
pub const BLOCK_B_START_OFFSET: u64 = 0x24400;
pub const BLOCK_B_CHECKSUM_OFFSET: u64 = 0x39BFE; // 2 byte

/*
    Utility definitions
*/

pub const DUMP_LINE_WIDTH:u64 = 40;

/*
    Utility functions
*/

pub fn dump_hex_region(savefile :&mut File, offset:usize, size : usize, mut out : impl Write)->Result<(),std::io::Error>{
    let mut buffer  = vec![0u8 ; size];
    savefile.seek(SeekFrom::Start(offset as u64))?;
    savefile.read_exact(&mut buffer)?;

    let mut i = 0;
    while i < size {
        let current_offset = offset + i ;
        
        // cout << "\n0x" << hex << setw(8) << setfill('0') ...
        // :08X significa: esadecimale (X) maiuscolo, largo 8 caratteri, riempito di zeri (08)
        write!(out,"\n0x{:08X}: ", current_offset)?;

        // --- PARTE HEX ---
        for j  in 0..DUMP_LINE_WIDTH {
            if i + j as usize >= size {
                // Stampa spazi vuoti se andiamo oltre la dimensione
                write!(out,"   ")?; // 3 spazi (2 per il byte vuoto + 1 di separazione)
                continue;
            }

            let byte = buffer[i + j as usize];
            // :02X significa: esadecimale, largo 2 caratteri, riempito di zeri
            write!(out,"{:02X} ", byte)?;
        }

        write!(out," | ")?;

        // --- PARTE CHAR ---
        for j in 0..DUMP_LINE_WIDTH {
            if i + j as usize >= size {
                break;
            }

            let byte = buffer[i + j as usize];
            
            if byte == 0x0A {
                // Gestione newline come facevi tu
                write!(out,"\\n ")?;
            } else if byte >= 32 && byte <= 126 {
                // Carattere ASCII stampabile
                write!(out,"{} ", byte as char)?;
            } else {
                // Carattere non stampabile
                write!(out,". ")?;
            }
        }

        i += DUMP_LINE_WIDTH as usize;
    }
    
    write!(out,"\n")?; // Giusto un a capo finale per pulizia

    Ok(())


}
