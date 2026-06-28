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
    Pokemon data structure offsets
*/
pub const PID : usize = 0x0; // 2 bytes 
pub const POKEMON_CHECKSUM : usize = 0x6; // 2 bytes 
// Bytes 0x4 and 0x5 are unused

/*
    Each Pokemon contains 120 bytes of encrypted data 
    They are organized in blocks of 30 bytes each
    Offsets are calculated from the beginning of the pokemon file
*/

//Block A 
pub const NATIONAL_POKEDEX_ID: usize = 0x08;
pub const HELD_ITEM: usize           = 0x0A;
pub const OT_ID: usize               = 0x0C;
pub const OT_SECRET_ID: usize        = 0x0E;
pub const EXPERIENCE_POINTS: usize   = 0x10;
pub const FRIENDSHIP: usize          = 0x14; // If the pokemon is an egg, this value represents the required egg cycles to hatch
pub const ABILITY: usize             = 0x15;
pub const MARKINGS: usize            = 0x16;
pub const ORIGINAL_LANGUAGE: usize   = 0x17;
pub const HP_EV: usize               = 0x18;
pub const ATK_EV: usize              = 0x19;
pub const DEF_EV: usize              = 0x1A;
pub const SPD_EV: usize              = 0x1B;
pub const SPATK_EV: usize            = 0x1C;
pub const SPDEF_EV: usize            = 0x1D;
pub const COOL_CONTEST_VALUE: usize  = 0x1E;
pub const BEAUTY_CONTEST_VALUE: usize = 0x1F;
pub const CUTE_CONTEST_VALUE: usize  = 0x20;
pub const SMART_CONTEST_VALUE: usize = 0x21;
pub const TOUGH_CONTEST_VALUE: usize = 0x22;
pub const SHEEN_CONTEST_VALUE: usize = 0x23;
pub const SINNOH_RIBBON_SET: usize   = 0x24;
pub const UNOVA_RIBBON_SET: usize    = 0x26;

//Block B 
pub const MOVE1_ID: usize               = 0x28;
pub const MOVE2_ID: usize               = 0x2A;
pub const MOVE3_ID: usize               = 0x2C;
pub const MOVE4_ID: usize               = 0x2E;
pub const MOVE_1_PP: usize              = 0x30;
pub const MOVE_2_PP: usize              = 0x31;
pub const MOVE_3_PP: usize              = 0x32;
pub const MOVE_4_PP: usize              = 0x33;
pub const MOVE_PP_UPS: usize            = 0x34;
pub const IV_EGG_NICK: usize            = 0x38;
pub const HOENN_RIBBON_SET_1: usize     = 0x3C;
pub const HOENN_RIBBON_SET_2: usize     = 0x3E;
pub const FORM_BYTE: usize              = 0x40;
pub const NATURE: usize                 = 0x41;
pub const DREAM_N: usize                = 0x42;

//IV_EGG_NICK masks 
/*
Each IV is 5 bits long
*/
pub const HP_IV_SHIFT: u32        = 0;
pub const ATK_IV_SHIFT: u32       = 5;
pub const DEF_IV_SHIFT: u32       = 10;
pub const SPD_IV_SHIFT: u32       = 15;
pub const SPATK_IV_SHIFT: u32     = 20;
pub const SPDEF_IV_SHIFT: u32     = 25;
pub const IS_EGG_SHIFT: u32       = 30;
pub const IS_NICKNAMED_SHIFT: u32 = 31;

pub const HP_IV_MASK: u32        = 0x1F << HP_IV_SHIFT;
pub const ATK_IV_MASK: u32       = 0x1F << ATK_IV_SHIFT;
pub const DEF_IV_MASK: u32       = 0x1F << DEF_IV_SHIFT;
pub const SPD_IV_MASK: u32       = 0x1F << SPD_IV_SHIFT;
pub const SPATK_IV_MASK: u32     = 0x1F << SPATK_IV_SHIFT;
pub const SPDEF_IV_MASK: u32     = 0x1F << SPDEF_IV_SHIFT;
pub const IS_EGG_MASK: u32       = 1 << IS_EGG_SHIFT;
pub const IS_NICKNAMED_MASK: u32 = 1 << IS_NICKNAMED_SHIFT; 

//Form byte masks 
pub const FATEFUL_ENCOUNTER_MASK : usize    = 1 << 0;
pub const FEMALE_MASK : usize               = 1 << 1;
pub const GENDERLESS_MASK : usize           = 1 << 2;
pub const ALTERNATE_FORMS : usize           = 1 << 3;

//Dream N masks
pub const HAS_DREAM_ABILITY_MASK : usize    = 1 << 0;
pub const IS_N_POKEMON : usize              = 1 << 1;

//Block C
pub const NICKNAME: usize             = 0x48;
pub const UNKNOWN: usize              = 0x5E;
pub const ORIGIN_GAME: usize          = 0x5F;
pub const SINNOH_RIBBON_SET_3: usize  = 0x60;
pub const SINNOH_RIBBON_SET_4: usize  = 0x62;

//Block D
pub const OT_NAME: usize              = 0x68;
pub const DATE_EGG_RECEIVED: usize    = 0x78;
pub const DATE_MET: usize             = 0x7B;
pub const EGG_LOCATION: usize         = 0x7E;
pub const MET_AT_LOCATION: usize      = 0x80;
pub const POKERUS: usize              = 0x82;
pub const POKE_BALL: usize            = 0x83;
pub const MET_AT_LEVEL_GENDER: usize  = 0x84;
pub const ENCOUNTER_TYPE: usize       = 0x85;

pub const MET_LEVEL_MASK: u8          = 0x7F;
pub const FEMALE_OT_GENDER_MASK: u8   = 1 << 7;

pub const POKERUS_STRAIN_SHIFT: u8 = 4;
pub const POKERUS_DAYS_SHIFT: u8   = 0;

pub const POKERUS_STRAIN_MASK: u8  = 0x0F << POKERUS_STRAIN_SHIFT;
pub const POKERUS_DAYS_MASK: u8    = 0x0F << POKERUS_DAYS_SHIFT;

/*
    Battle stats
*/
pub const STATUS_CONDITION: usize     = 0x90;
pub const LEVEL: usize                = 0x94;
pub const CAPSULE_INDEX: usize        = 0x95;
pub const CURRENT_HP: usize           = 0x96;
pub const MAX_HP: usize               = 0x98;
pub const ATTACK: usize               = 0x9A;
pub const DEFENSE: usize              = 0x9C;
pub const SPEED: usize                = 0x9E;
pub const SPECIAL_ATTACK: usize       = 0xA0;
pub const SPECIAL_DEFENSE: usize      = 0xA2;
pub const MAIL_MESSAGE: usize         = 0xA4;

pub const ASLEEP_SHIFT: u8            = 0;
pub const ASLEEP_MASK: u8             = 0x07 << ASLEEP_SHIFT;

pub const POISONED_MASK: u8           = 1 << 3;
pub const BURNED_MASK: u8             = 1 << 4;
pub const FROZEN_MASK: u8             = 1 << 5;
pub const PARALYZED_MASK: u8          = 1 << 6;
pub const TOXIC_MASK: u8              = 1 << 7;



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

/*
    PRNG implementation
    Pseudorandom Number Generator
*/

// pub fn prng(seed : u32) -> u32{

// }
