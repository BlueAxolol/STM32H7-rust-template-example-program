#![no_main]
#![no_std]

use cortex_m_semihosting::debug;
use defmt_rtt as _; // global logger
use panic_probe as _;
use stm32h7xx_hal::{self as _, block}; // memory layout
//use stm32h7xx_hal::gpio::gpiob;
//use stm32h7xx_hal::rng::Rng;
use stm32h7xx_hal::{
    pac,
    prelude::*,
    //serial::{Serial, config::Config},
};

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

    //gpio ports initialisation
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    //turn ports into outputs
    let mut led_red = gpiob.pb14.into_push_pull_output();
    let mut led_green = gpiob.pb0.into_push_pull_output();
    let mut led_yellow = gpioe.pe1.into_push_pull_output();
    //turn ports into inputs
    let btn_a1 = gpioc.pc0.into_pull_down_input();
    let btn_a2 = gpioc.pc3.into_pull_down_input();
    let btn_a3 = gpiob.pb1.into_pull_down_input();
    let user_btn = gpioc.pc13.into_pull_down_input();
    // TODO: initialize peripherals (UART and GPIOs)
    let tx = gpiod.pd8.into_alternate();
    let rx = gpiod.pd9.into_alternate();
    let serial = dp
        .USART3
        .serial(
            (tx, rx),
            115_200.bps(),
            ccdr.peripheral.USART3,
            &ccdr.clocks,
        )
        .unwrap();

    let (mut tx, _rx) = serial.split();
    // see the following examples for more info:
    // https://github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/blinky.rs (GPIO)
    // https://github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/serial.rs (UART)

    //for rising edge detection
    let mut btn_a2_prev = false;
    let mut user_btn_prev = false;
    led_yellow.set_low();

    loop {
        led_green.set_low();
        led_red.set_low();

        if btn_a2.is_high() && btn_a2_prev {
            led_yellow.toggle();
            block!(tx.write(b'p')).ok();
        }
        if user_btn.is_high() && user_btn_prev {
            block!(tx.write(b'h')).ok();
        }
        if btn_a3.is_high() ^ btn_a1.is_high() {
            if btn_a3.is_high() {
                led_red.set_high();
                block!(tx.write(b'd')).ok();
            } else if btn_a1.is_high() {
                led_green.set_high();
                block!(tx.write(b'u')).ok();
            }
        }
        btn_a2_prev = btn_a2.is_low();
        user_btn_prev = user_btn.is_low();
        delay.delay_ms(17_u8);

        //traffic light program

        // led_red.set_high();
        // while user_btn.is_low() {}
        // while user_btn.is_high() {}
        // delay.delay_us(50_u16);

        // delay.delay_ms(1000_u16);
        // led_red.set_low();
        // led_yellow.set_high();
        // delay.delay_ms(3000_u16);
        // led_yellow.set_low();
        // led_green.set_high();

        // while user_btn.is_low() {}
        // while user_btn.is_high() {}
        // delay.delay_us(50_u16);

        // for _i in 1..4 {
        //     led_green.set_low();
        //     delay.delay_ms(1000_u16);
        //     led_green.set_high();
        //     delay.delay_ms(1000_u16);
        // }
        // led_green.set_low();
        // led_yellow.set_high();
        // delay.delay_ms(3000_u16);
        // led_yellow.set_low();
        // led_red.set_high();
        // TODO read input pins and send UART command
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
