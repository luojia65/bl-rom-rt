#![no_std]
#![no_main]

extern crate panic_halt;

#[bl_rom_rt::entry]
fn main() -> ! {
    loop {}
}
