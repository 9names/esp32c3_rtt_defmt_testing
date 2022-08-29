#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, pac::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO, Delay};
use riscv_rt::entry;
use rtt_target::{rtt_init};
use panic_halt as _;
use defmt::info;
use defmt_rtt_target as _;


#[entry]
fn main() -> ! {
    let channels = rtt_init! {
        up: {
            0: {
                size: 1024
                mode: BlockIfFull
                name: "defmt"
            }
        }
        down: {
            0: {
                size: 16
                name: "Terminal"
            }
        }
    };

    defmt_rtt_target::init(channels.up.0);

    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Set GPIO5 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio7.into_push_pull_output();

    led.set_high().unwrap();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let mut delay = Delay::new(&clocks);

    let mut counter:u32 = 0;
    loop {
        led.toggle().unwrap();
        delay.delay_ms(500u32);
        info!("output {}", counter);
        counter += 1;
    }
}
