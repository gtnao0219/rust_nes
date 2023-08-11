use crate::rom::ROM;

const HEADER_SIZE: usize = 0x10;
const PROGRAM_ROM_UNIT_SIZE: usize = 0x4000;
const CHARACTER_ROM_UNIT_SIZE: usize = 0x2000;

pub struct Cartridge {
    pub program_rom: ROM,
    pub character_rom: ROM,
}

impl Cartridge {
    pub fn new(data: &[u8]) -> Self {
        let program_rom_size = data[4] as usize * PROGRAM_ROM_UNIT_SIZE;
        let character_rom_size = data[5] as usize * CHARACTER_ROM_UNIT_SIZE;

        let program_rom_start = HEADER_SIZE;
        let program_rom_end = program_rom_start + program_rom_size;
        let character_rom_start = program_rom_end;
        let character_rom_end = character_rom_start + character_rom_size;

        let program_rom_data = data[program_rom_start..program_rom_end].to_vec();
        let character_rom_data = data[character_rom_start..character_rom_end].to_vec();
        let program_rom = ROM::new(program_rom_data);
        let character_rom = ROM::new(character_rom_data); 

        Cartridge {
            program_rom,
            character_rom,
        }
    }
}
