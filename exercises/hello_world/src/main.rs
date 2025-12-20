// do not load `std` crate since we have no OS.
// functions like `std::Vec` require a heap allocator etc.
#![no_std]
#![no_main]

// Handles our code panicking, and if we use `panic!`.
use esp_backtrace as _;

// Log
// Besides `println` there is `dbg` and `print` (no added line).
use esp_println::{/*logger,*/ println};
// use log::{debug, trace};

// More on: https://docs.espressif.com/projects/rust/esp-bootloader-esp-idf/0.4.0/esp32/esp_bootloader_esp_idf/index.html
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    // Notice the `default()`.
    // They implement:
    // ```
    // impl Default for esp_hal::Config {
    //     fn default() -> Self { ... }
    // }
    // ```
    let config = esp_hal::Config::default();

    // Initialise the peripherals with default config.
    let _peripherals = esp_hal::init(config);

    // `log` must be initialised
    // logger::init_logger_from_env();

    // try `use esp-println::dbg;` uncomment below!
    // dbg!("Hi from dbg..!");
    let delay = esp_hal::delay::Delay::new();
    // Try changing the ESP_LOG level to trace
    // trace!("My Trace Log");
    // info!("My Info Log");
    loop {
        // `loop` is `!` type, since it never returns.
        // We are good!
        delay.delay_millis(2000);
        println!("loop..!");
    }
}
