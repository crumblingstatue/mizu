mod apu;
mod cartridge;
mod cpu;
mod joypad;
mod memory;
mod ppu;
mod serial;
mod timer;

//#[cfg(test)]
//TODO: re enable tests once the CGB implementation is finished
//mod tests;

use cartridge::{Cartridge, CartridgeError};
use cpu::Cpu;
use memory::Bus;
use std::fs::File;
use std::io::Read;

use std::path::Path;

pub use joypad::JoypadButton;

pub struct GameBoy {
    cpu: Cpu,
    bus: Bus,
}

impl GameBoy {
    pub fn new<P: AsRef<Path>>(
        file_path: P,
        boot_rom_file: Option<P>,
    ) -> Result<Self, CartridgeError> {
        let cartridge = Cartridge::from_file(file_path)?;

        let (bus, cpu) = if let Some(boot_rom_file) = boot_rom_file {
            let mut boot_rom_file = File::open(boot_rom_file)?;
            let mut data = [0; 0x900];

            // make sure the boot_rom is the exact same size
            assert_eq!(
                boot_rom_file.metadata()?.len(),
                data.len() as u64,
                "boot_rom file size is not correct"
            );

            boot_rom_file.read_exact(&mut data)?;

            (Bus::new_with_boot_rom(cartridge, data), Cpu::new())
        } else {
            (
                Bus::new_without_boot_rom(cartridge),
                Cpu::new_without_boot_rom(),
            )
        };

        Ok(Self { bus, cpu })
    }

    /// Note entirly accurate, but its better than looping over a fixed
    /// number of CPU instructions per frame
    pub fn clock_for_frame(&mut self) {
        const CPU_CYCLES_PER_FRAME: u32 = 16384 * 256 / 4 / 60;
        let mut cycles = 0u32;
        while cycles < CPU_CYCLES_PER_FRAME {
            self.cpu.next_instruction(&mut self.bus);

            cycles += self.bus.elapsed_cpu_cycles() as u32;
        }
    }

    pub fn screen_buffer(&self) -> &[u8] {
        self.bus.screen_buffer()
    }

    pub fn audio_buffer(&mut self) -> Vec<f32> {
        self.bus.audio_buffer()
    }

    pub fn press_joypad(&mut self, button: JoypadButton) {
        self.bus.press_joypad(button);
    }

    pub fn release_joypad(&mut self, button: JoypadButton) {
        self.bus.release_joypad(button);
    }
}
