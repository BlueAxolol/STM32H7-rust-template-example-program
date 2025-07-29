#![no_main]
#![no_std]

// Program that uses pins A1 (PC0), A2 (PC3), A3 (PB1), User_Btn (PC13) on the Nucleo-H753ZI as inputs and the
// built in LEDs as outputs

use cortex_m_semihosting::debug;
use defmt_rtt as _; // global logger
use panic_probe as _;
use stm32h7xx_hal::{self as _, block}; // memory layout

use stm32h7xx_hal::{pac, prelude::*};

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}

/// Hardfault handler.
///
/// Terminates the application and makes a semihosting-capable debug tool exit
/// with an error. This seems better than the default, which is to spin in a
/// loop.
#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    defmt::println!("Setup PWR...                  ");
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    defmt::println!("Setup RCC...                  ");
    let rcc = dp.RCC.constrain();

    // CCDR (Core Clock Distribution and Reset)
    let ccdr = rcc.sys_ck(400.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    // gpio ports initialisation
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    // turn ports into outputs
    let mut led_red = gpiob.pb14.into_push_pull_output();
    let mut led_green = gpiob.pb0.into_push_pull_output();
    let mut led_yellow = gpioe.pe1.into_push_pull_output();
    // turn ports into inputs
    let btn_a1 = gpioc.pc0.into_pull_down_input();
    let btn_a2 = gpioc.pc3.into_pull_down_input();
    let btn_a3 = gpiob.pb1.into_pull_down_input();
    let user_btn = gpioc.pc13.into_pull_down_input();

    // see the following examples for more info:
    // https://github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/blinky.rs (GPIO)
    // https://github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/serial.rs (UART)

    // for rising edge detection
    let mut btn_a2_prev = false;
    let mut user_btn_prev = false;
    led_yellow.set_low();

    loop {
        // sets leds low if no button is pressed
        led_green.set_low();
        led_red.set_low();

        // rising edge detection
        if btn_a2.is_high() && btn_a2_prev {
            led_yellow.toggle();
        }
        if user_btn.is_high() && user_btn_prev {
            led_yellow.toggle();
        }
        // XOR for other btns (doesn't send if both buttons pressed simultaneously)
        if btn_a3.is_high() ^ btn_a1.is_high() {
            if btn_a3.is_high() {
                led_red.set_high();
            } else if btn_a1.is_high() {
                led_green.set_high();
            }
        }
        // resets btn a1 and user_btn for rising edge detection
        btn_a2_prev = btn_a2.is_low();
        user_btn_prev = user_btn.is_low();
        // waits 5ms before starting loop again
        delay.delay_ms(5_u8);
    }
}

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }
}
