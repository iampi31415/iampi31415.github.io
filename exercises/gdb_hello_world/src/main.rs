#![no_std]
#![no_main]

use esp_hal::{Config, delay::Delay};

use esp_backtrace as _;
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    let _peripherals = esp_hal::init(Config::default());

    let delay = Delay::new();
    let mut a = 0;
    loop {
        a += 1;
        println!("hi");
        println!("{a}\n");
        delay.delay_millis(1500);
    }
}
