use std::io::{self, Write};
use std::option::Option;
use std::process::Command;
use std::str;
use std::thread::sleep;
use std::time::Duration;

#[cfg(target_os = "linux")]
use sysfs_gpio::{Direction, Pin};

#[cfg(target_os = "windows")]
struct Pin {}
#[cfg(target_os = "windows")]
impl Pin {
    pub fn get_value(&self) -> Result<u32, ()> {
        Ok(0)
    }
    pub fn set_value(&self, _value: u32) -> Result<(), ()> {
        Ok(())
    }
}

const TRIGGER_ON: f64 = 50.0;
const TRIGGER_OFF: f64 = 45.0;

fn main() {
    let fan = configure_fan_pin();

    loop {
        let temp = read_temp();

        println!("Current temperature is {}°C", temp);

        if let Some(fan) = &fan {
            if fan.get_value().expect("Unable to read fan pin value") == 0 && temp >= TRIGGER_ON {
                fan.set_value(1).unwrap();
            }
            if fan.get_value().expect("Unable to read fan pin value") == 1 && temp <= TRIGGER_OFF {
                fan.set_value(0).unwrap();
            }
        }
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}

#[cfg(target_os = "windows")]
fn get_temp_cmd() -> Command {
    let mut cmd = Command::new("cmd");
    cmd.args(["/C", "echo temp=43.3'C"]);
    cmd
}

#[cfg(target_os = "linux")]
fn get_temp_cmd() -> Command {
    let mut cmd = Command::new("vcgencmd");
    cmd.arg("measure_temp");
    cmd
}

#[cfg(target_os = "linux")]
fn configure_fan_pin() -> Option<Pin> {
    let fan = Pin::new(23);
    fan.export().expect("Unable to export Pin");
    fan.set_direction(Direction::Out)
        .expect("Unable to set Pin as output");
    Some(fan)
}

#[cfg(target_os = "windows")]
fn configure_fan_pin() -> Option<Pin> {
    None
}

fn read_temp() -> f64 {
    let result_vec = get_temp_cmd()
        .output()
        .expect("Failed to read temperature")
        .stdout;
    let result_temp = str::from_utf8(&result_vec).expect("Command invalid");
    let temp_str = result_temp.split('=').collect::<Vec<&str>>()[1];
    let temp_number_str = temp_str.split('\'').collect::<Vec<&str>>()[0];
    temp_number_str.parse::<f64>().unwrap()
}
