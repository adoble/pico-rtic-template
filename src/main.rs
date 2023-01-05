//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

#[rtic::app(
    device = rp_pico::hal::pac, // TODO: Replace `some_hal::pac` with the path to the PAC
    dispatchers = [TIMER_IRQ_1] // TODO: Replace the `FreeInterrupt1, ...` with free interrupt vectors if software tasks are used
)]
mod app {

    use rp_pico::hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        sio::Sio,
        watchdog::Watchdog,
    };

    use rp_pico::XOSC_CRYSTAL_FREQ;

    use rp2040_monotonic::{
        fugit::Duration,
        fugit::RateExtU32, // For .kHz() conversion funcs
        Rp2040Monotonic,
    };

    #[monotonic(binds = TIMER_IRQ_0, default = true)]
    type Rp2040Mono = Rp2040Monotonic;

    // Tiner constants
    const MONO_NUM: u32 = 1;
    const MONO_DENOM: u32 = 1000000;
    const ONE_SEC_TICKS: u64 = 1000000;

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        led_state: bool,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("init");

        let mut watchdog = Watchdog::new(ctx.device.WATCHDOG);
        let clocks = init_clocks_and_plls(
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

    #[task(local = [led_state])]
    fn toggle_task(ctx: toggle_task::Context) {
        if *ctx.local.led_state {
            defmt::info!("led on");
            *ctx.local.led_state = false;
        } else {
            defmt::info!("led off");
            *ctx.local.led_state = true;
        }
        // Re-spawn this task after 1 second
        let one_second = Duration::<u64, MONO_NUM, MONO_DENOM>::from_ticks(ONE_SEC_TICKS);
        toggle_task::spawn_after(one_second).unwrap();
    }
}

// End of file
