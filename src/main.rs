mod fan;
mod thermometer;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use log::{info, debug};
use env_logger::Env;

const TRIGGER_ON: f64 = 70.0;
const TRIGGER_OFF: f64 = 60.0;

const FAN_PIN: u64 = 23;

fn main() -> Result<(), thermometer::ThermometerError>{
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Fan Control starting");
    info!("Running Config:");
    info!("Using GPIO {} for CPU cooler.", FAN_PIN);
    info!("Trigger Fan On temperature is {}°C", TRIGGER_ON);
    info!("Trigger Fan Off temperature is {}°C", TRIGGER_OFF);

    #[cfg(target_os = "windows")]
    let mut fan = fan::Fan::new(FAN_PIN).expect("Unable to create Pin");
    #[cfg(target_os = "linux")]
    let fan = fan::Fan::new(FAN_PIN).expect("Unable to create Pin");

    let thermometer = thermometer::Thermometer::default();

    debug!("Fan initialized, start to monitor temperature.");

    loop {
        let temp = thermometer.read_temp()?;

        debug!("Current temperature is {}°C", temp);

        if fan.is_on().expect("Unable to read fan pin value") && temp >= TRIGGER_ON {
            info!(
                "CPU Temperature is {}°C >= {} -> Turning Fan On",
                temp, TRIGGER_ON
            );
            fan.turn_on().expect("Unable to turn fan on");
        }
        if fan.is_on().expect("Unable to read fan pin value") && temp <= TRIGGER_OFF {
            info!(
                "CPU Temperature is {}°C <= {} -> Turning Fan Off",
                temp, TRIGGER_OFF
            );
            fan.turn_off().expect("Unable to turn fan on");
        }
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}
