#![no_std]
#![no_main]

use defmt_rtt as _;
use hal::{pac, prelude::*};
use panic_probe as _;
use stm32h7xx_hal as hal;

#[defmt_test::tests]
mod tests {
    use super::*;
    use cortex_m::asm::delay;
    struct TestArgs {
        led: stm32h7xx_hal::gpio::Pin<'A', 5, stm32h7xx_hal::gpio::Output>,
        pc0: stm32h7xx_hal::gpio::Pin<'C', 0, stm32h7xx_hal::gpio::Input>,
        pb1: stm32h7xx_hal::gpio::Pin<'B', 1, stm32h7xx_hal::gpio::Input>,
    }

    #[init]
    // Shared setup function
    // initialize everything you need in your tests and add it to your TestArgs Struct
    // to run multiple tests with the same power configuration
    fn setup() -> TestArgs {
        let dp = pac::Peripherals::take().unwrap();

        // Power configuration
        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.freeze();

        // Clock configuration
        let rcc = dp.RCC.constrain();
        let ccdr = rcc.sys_ck(400.MHz()).freeze(pwrcfg, &dp.SYSCFG);

        // GPIO Port configuration
        let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
        let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
        let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

        // output configuration
        let led_ = gpioa.pa5.into_push_pull_output();

        // input configuration
        let pc0_ = gpioc.pc0.into_pull_down_input();
        let pb1_ = gpiob.pb1.into_pull_down_input();

        TestArgs {
            led: led_,
            pc0: pc0_,
            pb1: pb1_,
        }
    }

    #[test]
    fn led_toggles(ta: &mut TestArgs) {
        ta.led.set_high();
        delay(8_000_000);
        assert!(ta.led.is_set_high());
        ta.led.set_low();
        delay(8_000_000);
        assert!(ta.led.is_set_low());
    }

    #[test]
    fn button_press(ta: &mut TestArgs) {
        assert!(ta.pb1.is_low());
        delay(8_000_000);
        assert!(ta.pc0.is_low());
        delay(8_000_000);
    }
}
