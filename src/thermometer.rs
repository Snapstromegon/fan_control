#[cfg(target_os = "linux")]
use std::process::Command;
#[cfg(target_os = "linux")]
use std::str;

#[cfg(target_os = "linux")]
fn get_temp_cmd() -> Command {
  let mut cmd = Command::new("vcgencmd");
  cmd.arg("measure_temp");
  cmd
}

#[cfg(target_os = "linux")]
pub fn read_temp() -> f64 {
  let result_vec = get_temp_cmd()
    .output()
    .expect("Failed to read temperature")
    .stdout;
  let result_temp = str::from_utf8(&result_vec).expect("Command invalid");
  let temp_str = result_temp.split('=').collect::<Vec<&str>>()[1];
  let temp_number_str = temp_str.split('\'').collect::<Vec<&str>>()[0];
  temp_number_str.parse::<f64>().unwrap()
}

#[cfg(target_os = "windows")]
pub fn read_temp() -> f64 {
  43.0
}
