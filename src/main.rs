//! Blinks the LED on a Pico board using the RTIC framework.
//!
//! This will blink an LED attached to GPIO25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

//use defmt::*;
use defmt as _;
use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(
    device = rp_pico::hal::pac, dispatchers = [TIMER_IRQ_1]
)]
mod app {

    //use core::pin::Pin;

    use embedded_hal::digital::v2::OutputPin;

    use rp_pico::hal::{
        clocks, gpio, gpio::pin::bank0::Gpio25, gpio::pin::PushPullOutput, sio::Sio,
        watchdog::Watchdog,
    };
    use rp_pico::XOSC_CRYSTAL_FREQ;

    use rp2040_monotonic::{fugit::ExtU64, Rp2040Monotonic};

    #[monotonic(binds = TIMER_IRQ_0, default = true)]
    type Rp2040Mono = Rp2040Monotonic;

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        led_state: bool,
        led_pin: gpio::Pin<Gpio25, PushPullOutput>,
        // TODO: Use own resources
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("init");

        // Setup the clock.
        let mut watchdog = Watchdog::new(ctx.device.WATCHDOG);
        let _clocks = clocks::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut ctx.device.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        // Set up the led pin
        let sio = Sio::new(ctx.device.SIO);
        let pins = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );

        let mut led_pin = pins.led.into_push_pull_output();
        led_pin.set_low().unwrap();

        // Setup the monotonic timer
        let mono = Rp2040Monotonic::new(ctx.device.TIMER);

        // Spawn the led toggle task
        toggle_task::spawn().ok();

        (
            Shared {
            // Initialization of shared resources go here
        },
            Local {
                // Initialization of local resources go here
                led_state: false,
                led_pin: led_pin,
            },
            init::Monotonics(mono),
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    // Toggle the led based on a local state
    #[task(local = [led_state, led_pin])]
    fn toggle_task(ctx: toggle_task::Context) {
        if *ctx.local.led_state {
            defmt::info!("led on");
            ctx.local.led_pin.set_high().unwrap();
            *ctx.local.led_state = false;
        } else {
            defmt::info!("led off");
            ctx.local.led_pin.set_low().unwrap();
            *ctx.local.led_state = true;
        }

        // Re-spawn this task after 1000 milliseconds
        let duration: u64 = 1000;
        toggle_task::spawn_after(duration.millis()).unwrap();
    }
}

// End of file
