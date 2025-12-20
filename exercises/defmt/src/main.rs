#![no_std]
#![no_main]

// ___fix_me___: missing imports:
// esp_backtrace, esp_println, defmt

use esp_hal::{
    delay::Delay,
    time::{Duration, Instant},
};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Print a log or a message using defmt

    let now = Instant::now();
    loop {
        defmt::println!("Loop...");
        delay.delay_millis(500u32);
        if now.elapsed() > Duration::from_millis(2000) {
            break;
        }
    }
    // Use a panic! macro to trigger a panic
}
