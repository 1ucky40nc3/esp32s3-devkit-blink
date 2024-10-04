use esp_idf_svc::{
    hal::{delay::FreeRtos, gpio::PinDriver, prelude::Peripherals},
    sys::EspError,
};

fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio38)?;

    loop {
        log::info!("Blink!");
        led.set_high()?;
        // Sleep to to prevent watchdog trigger
        FreeRtos::delay_ms(1000);
        led.set_low()?;
        FreeRtos::delay_ms(1000);
    }

    Ok(())
}
