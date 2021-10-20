mod fan;
mod thermometer;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

const TRIGGER_ON: f64 = 70.0;
const TRIGGER_OFF: f64 = 60.0;

const FAN_PIN: u64 = 23;

fn main() {
    println!("Fan Control starting");
    println!("Running Config:");
    println!("Using GPIO {} for CPU cooler.", FAN_PIN);
    println!("Trigger Fan On temperature is {}°C", TRIGGER_ON);
    println!("Trigger Fan Off temperature is {}°C", TRIGGER_OFF);

    #[cfg(target_os = "windows")]
    let mut fan = fan::Fan::new(FAN_PIN).expect("Unable to create Pin");
    #[cfg(target_os = "linux")]
    let fan = fan::Fan::new(FAN_PIN).expect("Unable to create Pin");

    println!("Fan initialized, start to monitor temperature.");

    loop {
        let temp = thermometer::read_temp();

        println!("Current temperature is {}°C", temp);

        if fan.is_on().expect("Unable to read fan pin value") && temp >= TRIGGER_ON {
            println!(
                "CPU Temperature is {}°C >= {} -> Turning Fan On",
                temp, TRIGGER_ON
            );
            fan.turn_on().expect("Unable to turn fan on");
        }
        if fan.is_on().expect("Unable to read fan pin value") && temp <= TRIGGER_OFF {
            println!(
                "CPU Temperature is {}°C <= {} -> Turning Fan Off",
                temp, TRIGGER_OFF
            );
            fan.turn_off().expect("Unable to turn fan on");
        }
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}
