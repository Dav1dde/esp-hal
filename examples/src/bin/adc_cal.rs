//! Connect a potentiometer to PIN2 and see the read values change when
//! rotating the shaft. Alternatively you could also connect the PIN to GND or
//! 3V3 to see the maximum and minimum raw values read.

//% CHIPS: esp32c2 esp32c3 esp32c6 esp32s3

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    clock::ClockControl,
    delay::Delay,
    gpio::Io,
    peripherals::Peripherals,
    prelude::*,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    cfg_if::cfg_if! {
        if #[cfg(feature = "esp32s3")] {
            let analog_pin = io.pins.gpio3.into_analog();
        } else {
            let analog_pin = io.pins.gpio2.into_analog();
        }
    }

    // Create ADC instances
    // You can try any of the following calibration methods by uncommenting
    // them. Note that only AdcCalLine returns readings in mV; the other two
    // return raw readings in some unspecified scale.
    //
    type AdcCal = ();
    // type AdcCal = esp_hal::analog::adc::AdcCalBasic<ADC1>;
    // type AdcCal = esp_hal::analog::adc::AdcCalLine<ADC1>;
    // type AdcCal = esp_hal::analog::adc::AdcCalCurve<ADC1>;

    let mut adc1_config = AdcConfig::new();
    let mut adc1_pin =
        adc1_config.enable_pin_with_cal::<_, AdcCal>(analog_pin, Attenuation::Attenuation11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    let delay = Delay::new(&clocks);

    loop {
        let pin_mv = nb::block!(adc1.read_oneshot(&mut adc1_pin)).unwrap();
        println!("PIN2 ADC reading = {pin_mv} mV");
        delay.delay_millis(1500);
    }
}
