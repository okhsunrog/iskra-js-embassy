#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::rcc::{
    AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv, PllQDiv, PllSource,
    Sysclk,
};
use embassy_stm32::time::Hertz;
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    // Iskra JS has an 8 MHz crystal. 8 / 8 * 336 / 2 = 168 MHz SYSCLK.
    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll_src = PllSource::HSE;
    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::DIV8,
        mul: PllMul::MUL336,
        divp: Some(PllPDiv::DIV2),
        divq: Some(PllQDiv::DIV7), // 48 MHz clock for USB/RNG/SDIO.
        divr: None,
    });
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV4; // 42 MHz; TIM2 runs at 84 MHz.
    config.rcc.apb2_pre = APBPrescaler::DIV2; // 84 MHz.
    config.enable_debug_during_sleep = true;
    let p = embassy_stm32::init(config);
    let mut led = Output::new(p.PB6, Level::Low, Speed::Low);

    defmt::info!("Iskra JS: HSE 8 MHz, SYSCLK 168 MHz; PB6 blink, 500 ms per level");

    loop {
        led.set_high();
        defmt::info!("PB6 high");
        Timer::after_millis(500).await;

        led.set_low();
        defmt::info!("PB6 low");
        Timer::after_millis(500).await;
    }
}
