//! Print detailed information about every installed ASIO driver:
//! identity, channels (names and sample types), sample rate, buffer-size
//! constraints, and latencies.
//!
//! Runs on any platform; on non-Windows the driver list is empty and the
//! example exits successfully.

use maolan_asio::DriverInfo;
#[cfg(windows)]
use maolan_asio::driver::{Driver, SampleType};
use maolan_asio::drivers;

fn describe(info: &DriverInfo) {
    println!("== {} ==", info.name);
    if let Some(desc) = &info.description {
        println!("  description: {desc}");
    }
    #[cfg(not(windows))]
    {
        println!("  (ASIO is only available on Windows)");
    }
    #[cfg(windows)]
    {
        let Ok(guard) = maolan_asio::com::initialize() else {
            println!("  error: COM initialization failed on this thread");
            return;
        };
        let driver = match Driver::open(info, &guard) {
            Ok(driver) => driver,
            Err(e) => {
                println!("  open failed: {e}");
                return;
            }
        };
        if let Err(e) = describe_driver(&driver) {
            println!("  query failed: {}", e);
        }
    }
}

#[cfg(windows)]
fn describe_driver(driver: &Driver) -> Result<(), maolan_asio::error::AsioError> {
    println!("  driver name:    {}", driver.driver_name());
    println!("  driver version: {:#x}", driver.driver_version());
    let (inputs, outputs) = driver.channels()?;
    println!("  channels:       {inputs} in / {outputs} out");
    for input in [true, false] {
        let count = if input { inputs } else { outputs };
        let dir = if input { "input" } else { "output" };
        for i in 0..count {
            let ch = driver.channel_info(i, input)?;
            let type_name = sample_type_name(ch.sample_type);
            println!(
                "    {dir} {i}: {:<32} {type_name}{}",
                ch.name,
                if ch.is_active { "" } else { " (inactive)" },
            );
        }
    }
    println!("  sample rate:    {} Hz", driver.sample_rate()?);
    let sizes = driver.buffer_sizes()?;
    println!(
        "  buffer sizes:   min {} / max {} / preferred {} / granularity {}",
        sizes.min, sizes.max, sizes.preferred, sizes.granularity
    );
    let (in_lat, out_lat) = driver.latencies()?;
    println!("  latencies:      {in_lat} in / {out_lat} out (samples)");
    Ok(())
}

#[cfg(windows)]
fn sample_type_name(t: SampleType) -> &'static str {
    match t {
        SampleType::Int16Msb => "Int16 MSB",
        SampleType::Int24Msb => "Int24 MSB",
        SampleType::Int32Msb => "Int32 MSB",
        SampleType::Float32Msb => "Float32 MSB",
        SampleType::Float64Msb => "Float64 MSB",
        SampleType::Int32Msb16 => "Int32(16) MSB",
        SampleType::Int32Msb18 => "Int32(18) MSB",
        SampleType::Int32Msb20 => "Int32(20) MSB",
        SampleType::Int32Msb24 => "Int32(24) MSB",
        SampleType::Int16Lsb => "Int16 LSB",
        SampleType::Int24Lsb => "Int24 LSB",
        SampleType::Int32Lsb => "Int32 LSB",
        SampleType::Float32Lsb => "Float32 LSB",
        SampleType::Float64Lsb => "Float64 LSB",
        SampleType::Int32Lsb16 => "Int32(16) LSB",
        SampleType::Int32Lsb18 => "Int32(18) LSB",
        SampleType::Int32Lsb20 => "Int32(20) LSB",
        SampleType::Int32Lsb24 => "Int32(24) LSB",
        SampleType::Unknown(raw) => match raw {
            32..=39 => "DSD",
            _ => "unknown",
        },
    }
}

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
        describe(info);
    }
}
