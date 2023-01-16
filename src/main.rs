//! This template is intended as a starting point for developing rp-pico based application using the
//! [cortex-m-rtic](https://crates.io/crates/cortex-m-rtic) crate. It is based on
//! [this rp2040 template](https://github.com/rp-rs/rp2040-project-template) and
//! [this rtic example](https://github.com/rtic-rs/rtic-examples/blob/master/rtic_v1/rp-pico_local_initilzd_resources/src/main.rs).
//!
//! It does the following:
//! - Blinks the rp-pico on-board led (GPIO 25) using a timer
//! - Processes a interrupt when GPIO 17 is pulled low (e.g with a push button)

//! It includes all of the `knurling-rs` tooling as showcased in https://github.com/knurling-rs/app-template
//! (`defmt`, `defmt-rtt`, `panic-probe`, `flip-link`) to make development as easy as possible.

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

    use embedded_hal::digital::v2::{InputPin, OutputPin};

    use rp_pico::hal::gpio::PullUp;
    use rp_pico::hal::{
        clocks, gpio, gpio::pin::bank0::Gpio17, gpio::pin::bank0::Gpio25, gpio::pin::Input,
        gpio::pin::PushPullOutput, sio::Sio, watchdog::Watchdog,
    };
    use rp_pico::XOSC_CRYSTAL_FREQ;

    use rp2040_monotonic::{fugit::ExtU64, Rp2040Monotonic};

    #[monotonic(binds = TIMER_IRQ_0, default = true)]
    type Rp2040Mono = Rp2040Monotonic;

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add shared resources
    }

    // Local resources go here
    #[local]
    struct Local {
        led_state: bool,
        led_pin: gpio::Pin<Gpio25, PushPullOutput>,
        //button_pin: gpio::Pin<Gpio17, Disabled<PullDown>>,
        button_pin: gpio::Pin<Gpio17, Input<PullUp>>,
        // TODO: Use own resources
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("init");

        // Setup the clock. This is required.
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

        // Setup an interrupt om pin 16 to register button presses
        // This is configured as an input pin so that the value can be read.
        let button_pin = pins.gpio17.into_pull_up_input();

        button_pin.set_interrupt_enabled(gpio::Interrupt::EdgeLow, true); // ??? Does this work?

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
                led_pin,
                button_pin,
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

    // Service routine when GPIO 17 is pulled low.
    //
    // Note this is not a very good implementation if a push button
    // is being used to pull the pin low as no debounce is included.
    // As such a lot of interrupts are generated.
    #[task(binds = IO_IRQ_BANK0, local = [button_pin])]
    fn button_irq(ctx: button_irq::Context) {
        defmt::info!("Button pressed");

        ctx.local
            .button_pin
            .clear_interrupt(gpio::Interrupt::EdgeLow);

        // Read from the button
        if ctx.local.button_pin.is_high().unwrap() {
            defmt::info!("Button Pin HIGH");
        } else {
            defmt::info!("Button Pin LOW");
        }

        // TODO need to add debounce
    }
}

// End of file
