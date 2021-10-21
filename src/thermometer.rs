#[cfg(target_os = "linux")]
use std::fmt;
#[cfg(target_os = "linux")]
use std::io;
#[cfg(target_os = "linux")]
use std::num::ParseFloatError;
#[cfg(target_os = "linux")]
use std::process::Command;
#[cfg(target_os = "linux")]
use std::str;

#[derive(Debug)]
pub enum ThermometerError {
  #[cfg(target_os = "linux")]
  ParseFloatError(ParseFloatError),
  #[cfg(target_os = "linux")]
  Utf8Error(str::Utf8Error),
  #[cfg(target_os = "linux")]
  IOError(io::Error),
}

#[cfg(target_os = "linux")]
impl From<ParseFloatError> for ThermometerError {
  fn from(err: ParseFloatError) -> Self {
    Self::ParseFloatError(err)
  }
}

#[cfg(target_os = "linux")]
impl From<str::Utf8Error> for ThermometerError {
  fn from(err: str::Utf8Error) -> Self {
    Self::Utf8Error(err)
  }
}

#[cfg(target_os = "linux")]
impl From<io::Error> for ThermometerError {
  fn from(err: io::Error) -> Self {
    Self::IOError(err)
  }
}

#[cfg(target_os = "linux")]
impl fmt::Display for ThermometerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Thermometer Error")
  }
}

#[cfg(target_os = "linux")]
fn get_temp_cmd() -> Command {
  let mut cmd = Command::new("vcgencmd");
  cmd.arg("measure_temp");
  cmd
}

#[cfg(target_os = "linux")]
pub fn read_temp() -> Result<f64, ThermometerError> {
  let result_vec = get_temp_cmd().output()?.stdout;
  let result_temp = str::from_utf8(&result_vec)?;
  let temp_str = result_temp.split('=').collect::<Vec<&str>>()[1];
  let temp_number_str = temp_str.split('\'').collect::<Vec<&str>>()[0];
  Ok(temp_number_str.parse()?)
}

#[cfg(target_os = "windows")]
pub fn read_temp() -> Result<f64, ThermometerError> {
  Ok(43.0)
}
