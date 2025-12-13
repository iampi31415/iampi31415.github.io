// View ourtput with `minicom -D /dev/tty.mydevice`.
// Use "reset" once after opening it.
#![no_std]
#![no_main]

// Logging
use panic_rtt_target as _;
use rtt_target::rtt_init_defmt;

use esp_hal::{
    Blocking, main,
    uart::{self, DataBits, Parity, StopBits, Uart},
};

// make app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

// Constants
const WELCOME_MSG: &[u8] = b"Welcome to UART echo server.\r\n";
const MAX_LEN: usize = 100; // Max bytes (\r\n take 2 spaces already.)
const CR: u8 = 0x0D; // carriage return hex code.
const LF: u8 = 0x0A; // line feed hex code.

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let uart_config = uart::Config::default()
        .with_baudrate(9600)
        .with_data_bits(DataBits::_8) // because parity is off
        .with_parity(Parity::None)
        .with_stop_bits(StopBits::_1);
    rtt_init_defmt!(); // init rtt printer

    let mut uart = Uart::new(peripherals.UART0, uart_config)
        .expect("Should set up the driver.")
        .with_tx(peripherals.GPIO16)
        .with_rx(peripherals.GPIO17);

    write_flush(&mut uart, WELCOME_MSG);

    let mut buf = [0u8; MAX_LEN];
    let mut index = 0;
    while index < MAX_LEN - 1 {
        index += uart.read(&mut buf[index..]).unwrap();
        if index == 1 {
            let last = buf[0];
            if last == CR || last == LF {
                buf[0] = CR;
                buf[1] = LF;
                write_flush(&mut uart, &buf[..2]);
                // overwrite buf
                buf[..index].fill(0);
                // reset index
                index = 0
            }
        } else if index > 1 {
            let to: usize = match (buf[index - 2], buf[index - 1]) {
                (CR, LF) => index,
                (_, LF) => {
                    buf[index - 1] = CR;
                    buf[index] = LF;
                    index + 1
                }
                (_, CR) => {
                    buf[index] = LF;
                    index + 1
                }
                _ => continue,
            };
            write_flush(&mut uart, b"You said: ");
            write_flush(&mut uart, &buf[..to]);
            // overwrite what we had written with 0s.
            buf[..index].fill(0);
            // reset index
            index = 0;
        }
    }
    panic!("Too much talk.")
}

// MCU(Tx) => Laptop(Rx)
fn write_flush(uart: &mut Uart<'_, Blocking>, buf: &[u8]) {
    uart.write(buf).expect("Should write bytes to buffer.");
    uart.flush().expect("Send bytes from UART_Tx to laptop.");
}
