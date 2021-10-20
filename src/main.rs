mod fan;
mod thermometer;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

const TRIGGER_ON: f64 = 50.0;
const TRIGGER_OFF: f64 = 45.0;

fn main() {
    #[cfg(target_os = "windows")]
    let mut fan = fan::Fan::new(23).expect("Unable to create Pin");
    #[cfg(target_os = "linux")]
    let fan = fan::Fan::new(23).expect("Unable to create Pin");

    loop {
        let temp = thermometer::read_temp();

        println!("Current temperature is {}°C", temp);

        if fan.is_on().expect("Unable to read fan pin value") && temp >= TRIGGER_ON {
            fan.turn_on().expect("Unable to turn fan on");
        }
        if fan.is_on().expect("Unable to read fan pin value") && temp <= TRIGGER_OFF {
            fan.turn_off().expect("Unable to turn fan on");
        }
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}
