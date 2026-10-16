//! Open the first installed ASIO driver and play a 440 Hz sine wave on
//! all output channels for a few seconds (duplex stream; inputs are
//! opened but ignored).
//!
//! Runs on any platform; exits successfully when no ASIO driver is
//! installed.

use maolan_asio::DriverInfo;
use maolan_asio::drivers;

fn main() {
    let list = match drivers() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("error enumerating ASIO drivers: {e}");
            std::process::exit(1);
        }
    };
    if list.is_empty() {
        println!("No ASIO drivers found.");
        return;
    }
    for info in &list {
        if play_sine(info).is_ok() {
            return;
        }
    }
    eprintln!("no driver could be opened");
    std::process::exit(1);
}

#[cfg(not(windows))]
fn play_sine(_info: &DriverInfo) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(windows)]
fn play_sine(info: &DriverInfo) -> Result<(), Box<dyn std::error::Error>> {
    use maolan_asio::driver::Driver;
    use maolan_asio::stream::StreamConfig;

    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    println!("opening {} ...", info.name);
    let guard =
        maolan_asio::com::initialize().map_err(|hr| std::io::Error::from_raw_os_error(hr.0))?;
    let mut driver = Driver::open(info, &guard)?;
    let buffer_size: usize = std::env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(512);
    let config = StreamConfig::for_device(&driver, 48_000.0, buffer_size)?;
    println!(
        "streaming: {} in / {} out, {} Hz, requested {} frames",
        config.num_inputs, config.num_outputs, config.sample_rate, config.buffer_size,
    );

    let phase = Arc::new(Mutex::new(0.0f32));
    let phase_cb = phase.clone();

    let mut stream = driver.stream(&config, move |mut ctx| {
        let mut ph = phase_cb.lock().unwrap();
        let step = 2.0 * std::f32::consts::PI * 440.0 / 48_000.0;
        for ch in 0..ctx.num_outputs() {
            let out = ctx.output(ch);
            for s in out.iter_mut() {
                *s = 0.2 * ph.sin();
                *ph += step;
                if *ph > 2.0 * std::f32::consts::PI {
                    *ph -= 2.0 * std::f32::consts::PI;
                }
            }
        }
    })?;

    stream.start()?;
    println!("playing 440 Hz sine for 5 seconds ...");
    std::thread::sleep(Duration::from_secs(5));
    stream.stop()?;
    let events = stream.take_events();
    if events != 0 {
        println!("driver events during playback: {events:#x}");
    }
    Ok(())
}
