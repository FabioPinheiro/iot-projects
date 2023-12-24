use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// The simple-signal crate is used to handle incoming signals.
use simple_signal::{self, Signal};

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    // Retrieve the GPIO pin and configure it as an output.
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();
    thread::sleep(Duration::from_millis(500));
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();

    let running = Arc::new(AtomicBool::new(true));

    // When a SIGINT (Ctrl-C) or SIGTERM signal is caught, atomically set running to false.
    simple_signal::set_handler(&[Signal::Int, Signal::Term], {
        let running = running.clone();
        move |_| {
            running.store(false, Ordering::SeqCst);
        }
    });

    // Blink the LED until running is set to false.
    while running.load(Ordering::SeqCst) {
        pin.toggle();
        thread::sleep(Duration::from_millis(100));
    }

    // After we're done blinking, turn the LED off.
    pin.set_low();

    Ok(())

    // When the pin variable goes out of scope, the GPIO pin mode is automatically reset
    // to its original value, provided reset_on_drop is set to true (default).
}

// GPIO pinout https://pinout.xyz/pinout/pin16_gpio23/

// +-----+-----+---------+------+---+----------+---+------+---------+-----+-----+
// | BCM | wPi |   Name  | Mode | V | Physical | V | Mode | Name    | wPi | BCM |
// +-----+-----+---------+------+---+----++----+---+------+---------+-----+-----+
// |     |     |    3.3v |      |   |  1 || 2  |   |      | 5v      |     |     |
// |   2 |   8 |   SDA.1 |   IN | 1 |  3 || 4  |   |      | 5v      |     |     |
// |   3 |   9 |   SCL.1 |   IN | 1 |  5 || 6  |   |      | 0v      |     |     |
// |   4 |   7 | GPIO. 7 |   IN | 1 |  7 || 8  | 1 | ALT0 | TxD     | 15  | 14  |
// |     |     |      0v |      |   |  9 || 10 | 1 | ALT0 | RxD     | 16  | 15  |
// |  17 |   0 | GPIO. 0 |   IN | 0 | 11 || 12 | 1 | IN   | GPIO. 1 | 1   | 18  |
// |  27 |   2 | GPIO. 2 |   IN | 0 | 13 || 14 |   |      | 0v      |     |     |
// |  22 |   3 | GPIO. 3 |   IN | 0 | 15 || 16 | 1 | IN   | GPIO. 4 | 4   | 23  |
// |     |     |    3.3v |      |   | 17 || 18 | 0 | IN   | GPIO. 5 | 5   | 24  |
// |  10 |  12 |    MOSI |   IN | 0 | 19 || 20 |   |      | 0v      |     |     |
// |   9 |  13 |    MISO |   IN | 0 | 21 || 22 | 0 | IN   | GPIO. 6 | 6   | 25  |
// |  11 |  14 |    SCLK |   IN | 0 | 23 || 24 | 1 | IN   | CE0     | 10  | 8   |
// |     |     |      0v |      |   | 25 || 26 | 1 | IN   | CE1     | 11  | 7   |
// |   0 |  30 |   SDA.0 |   IN | 1 | 27 || 28 | 1 | IN   | SCL.0   | 31  | 1   |
// |   5 |  21 | GPIO.21 |   IN | 1 | 29 || 30 |   |      | 0v      |     |     |
// |   6 |  22 | GPIO.22 |   IN | 1 | 31 || 32 | 0 | IN   | GPIO.26 | 26  | 12  |
// |  13 |  23 | GPIO.23 |   IN | 0 | 33 || 34 |   |      | 0v      |     |     |
// |  19 |  24 | GPIO.24 |   IN | 0 | 35 || 36 | 0 | IN   | GPIO.27 | 27  | 16  |
// |  26 |  25 | GPIO.25 |   IN | 0 | 37 || 38 | 0 | IN   | GPIO.28 | 28  | 20  |
// |     |     |      0v |      |   | 39 || 40 | 0 | IN   | GPIO.29 | 29  | 21  |
// +-----+-----+---------+------+---+----++----+---+------+---------+-----+-----+
// | BCM | wPi |   Name  | Mode | V | Physical | V | Mode | Name    | wPi | BC  |
// +-----+-----+---------+------+---+--B Plus--+---+------+---------+-----+-----+
