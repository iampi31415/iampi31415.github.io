#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::{delay::Delay, main};

use esp_println::println;
use esp_backtrace as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // two lines needed for no "broken pipe"
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let _peripherals = esp_hal::init(config);

    let delay = Delay::new();
    let mut a = 5;
    loop {
        a += 1;
        println!("hi");
        delay.delay_millis(1500);
        println!("{a}");
    }
}
