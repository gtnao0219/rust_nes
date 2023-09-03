use crate::rom::ROM;

const HEADER_SIZE: usize = 0x0010;
const PROGRAM_ROM_UNIT_SIZE: usize = 0x4000; // 16KB
const CHARACTER_ROM_UNIT_SIZE: usize = 0x2000; // 8KB

pub struct Cartridge {
    pub program_rom: ROM,
    pub character_rom: ROM,
    pub is_horizontal_mirroring: bool,
}

impl Cartridge {
    pub fn new(data: &[u8]) -> Self {
        let program_rom_size = data[4] as usize * PROGRAM_ROM_UNIT_SIZE;
        let character_rom_size = data[5] as usize * CHARACTER_ROM_UNIT_SIZE;
        let is_horizontal_mirroring = data[6] & 0b0000_0001 == 0;

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
            is_horizontal_mirroring,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let header_data = vec![
            0x4e, 0x45, 0x53, 0x1a, // NES^Z
            0x02, 0x01, // page sizes
            0x01, // vertical mirroring
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // others
        ];
        let program_rom_data = vec![0x00; 0x8000];
        let character_rom_data = vec![0x00; 0x2000];
        let mut data = [
            &header_data[..],
            &program_rom_data[..],
            &character_rom_data[..],
        ]
        .concat();
        let cartridge = Cartridge::new(&data);
        assert_eq!(cartridge.program_rom.size(), 0x8000);
        assert_eq!(cartridge.character_rom.size(), 0x2000);
        assert_eq!(cartridge.is_horizontal_mirroring, false);

        data[6] = 0x00;
        let cartridge = Cartridge::new(&data);
        assert_eq!(cartridge.is_horizontal_mirroring, true);
    }
}
