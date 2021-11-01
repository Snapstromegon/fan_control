mod fan;
mod thermometer;
use env_logger::Env;
use log::{debug, error, info};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Fan Control")]
struct Opt {
    #[structopt(short = "p", long = "gpio_fan_pin", default_value = "23", env)]
    fan_pin: u64,
    #[structopt(short = "i", long = "fan_temp_on", default_value = "70", env)]
    trigger_on: f64,
    #[structopt(short = "o", long = "fan_temp_off", default_value = "60", env)]
    trigger_off: f64,
    #[structopt(
        short = "t",
        long = "thermal_zone",
        default_value = "thermal_zone0",
        env
    )]
    thermal_zone: String,
}

fn main() -> Result<(), thermometer::ThermometerError> {
    let opt = Opt::from_args();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Fan Control starting");
    info!("Running Config:");

    info!("Available Thermal Zones: {:?}", thermometer::Thermometer::get_available_thermal_zones().expect("Unable to list available thermal zones!"));
    info!("Using Thermal Zong {}.", opt.thermal_zone);
    info!("Using GPIO {} for CPU cooler.", opt.fan_pin);
    info!("Trigger Fan On temperature is {}°C", opt.trigger_on);
    info!("Trigger Fan Off temperature is {}°C", opt.trigger_off);

    #[cfg(not(feature = "sysfs"))]
    let mut fan = fan::Fan::new(opt.fan_pin).expect("Unable to create Pin");
    #[cfg(feature = "sysfs")]
    let fan = fan::Fan::new(opt.fan_pin).expect("Unable to create Pin");
    info!(
        "Fan is currently {}.",
        match fan.is_on() {
            Ok(true) => "on",
            Ok(false) => "off",
            _ => {
                error!("Unable to get fan state!");
                std::process::exit(1);
            }
        }
    );

    let thermometer = thermometer::Thermometer::new(&opt.thermal_zone);
    info!(
        "Temperature is currently {}.",
        match thermometer.read_temp() {
            Ok(x) => x,
            _ => {
                error!("Unable to read temperature!");
                std::process::exit(1);
            }
        }
    );

    debug!("Setup done, start monitioring temperature.");

    loop {
        let temp = thermometer.read_temp()?;

        debug!("Current temperature is {}°C", temp);

        let is_fan_on = fan.is_on().expect("Unable to read fan state!");

        if !is_fan_on && temp >= opt.trigger_on {
            info!(
                "CPU Temperature is {}°C >= {} -> Turning Fan On",
                temp, opt.trigger_on
            );
            fan.turn_on().expect("Unable to turn fan on");
        } else if is_fan_on && temp <= opt.trigger_off {
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
