mod fan;
mod thermometer;
use env_logger::Env;
use log::{debug, info};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "p", long = "gpio_fan_pin", default_value = "23")]
    fan_pin: u64,
    #[structopt(short = "i", long = "fan_temp_on", default_value = "70")]
    trigger_on: f64,
    #[structopt(short = "o", long = "fan_temp_off", default_value = "60")]
    trigger_off: f64,
}

fn main() -> Result<(), thermometer::ThermometerError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Fan Control starting");
    info!("Running Config:");

    let opt = Opt::from_args();

    info!("Using GPIO {} for CPU cooler.", opt.fan_pin);
    info!("Trigger Fan On temperature is {}°C", opt.trigger_on);
    info!("Trigger Fan Off temperature is {}°C", opt.trigger_off);

    #[cfg(target_os = "windows")]
    let mut fan = fan::Fan::new(opt.fan_pin).expect("Unable to create Pin");
    #[cfg(target_os = "linux")]
    let fan = fan::Fan::new(opt.fan_pin).expect("Unable to create Pin");

    let thermometer = thermometer::Thermometer::default();

    debug!("Fan initialized, start to monitor temperature.");

    loop {
        let temp = thermometer.read_temp()?;

        debug!("Current temperature is {}°C", temp);

        if fan.is_on().expect("Unable to read fan pin value") && temp >= opt.trigger_on {
            info!(
                "CPU Temperature is {}°C >= {} -> Turning Fan On",
                temp, opt.trigger_on
            );
            fan.turn_on().expect("Unable to turn fan on");
        }
        if fan.is_on().expect("Unable to read fan pin value") && temp <= opt.trigger_off {
            info!(
                "CPU Temperature is {}°C <= {} -> Turning Fan Off",
                temp, opt.trigger_off
            );
            fan.turn_off().expect("Unable to turn fan on");
        }
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}
