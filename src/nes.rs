use crate::bus::Bus;
use crate::cpu::Cpu;

// const MASTER_CLOCK: u32 = 21_477_272; // NTSC
const TICKS_PER_CPU_TICK: u8 = 12;
const TICKS_PER_PPU_TICK: u8 = 4;

pub struct Nes {
    bus: Bus,
    cpu: Cpu,
    cpu_tick_counter: u8,
    ppu_tick_counter: u8,
}

impl Nes {
    pub fn new() -> Self {
        let mut bus = Bus::new();
        let bus_pointer = &mut bus as *mut Bus;
        let cpu = Cpu::new(bus_pointer);

        Self {
            bus: bus,
            cpu: cpu,
            cpu_tick_counter: 0,
            ppu_tick_counter: 0,
        }
    }

    fn tick(&mut self) {
        self.ppu_tick_counter += 1;
        if self.ppu_tick_counter >= TICKS_PER_PPU_TICK {
            self.ppu_tick_counter = 0;
            self.bus.ppu_stub_tick();
        }

        self.cpu_tick_counter += 1;
        if self.cpu_tick_counter >= TICKS_PER_CPU_TICK {
            self.cpu_tick_counter = 0;
            self.cpu.tick();
        }
    }

    pub fn run_frame(&mut self) {
        self.tick();
    }
}
