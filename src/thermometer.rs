#[cfg(target_os = "linux")]
use std::fmt;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::io;
#[cfg(target_os = "linux")]
use std::num::ParseFloatError;
#[cfg(target_os = "linux")]
use std::str;

pub struct Thermometer {
  #[cfg(target_os = "linux")]
  path: String,
}

#[cfg(target_os = "linux")]
impl Thermometer {
  pub fn new(sysfs_thermometer_path: &str) -> Self {
    Thermometer {
      path: sysfs_thermometer_path.into(),
    }
  }

  pub fn read_temp(&self) -> Result<f64, ThermometerError> {
    let string_temp = fs::read_to_string(&self.path)?;
    let temp_float: f64 = string_temp.trim().parse()?;
    Ok(temp_float / 1000.0)
  }
}
#[cfg(not(target_os = "linux"))]
impl Thermometer {
  pub fn new(_sysfs_thermometer_path: &str) -> Self {
    Thermometer {}
  }

  pub fn read_temp(&self) -> Result<f64, ThermometerError> {
    Ok(43.0)
  }
}

impl Default for Thermometer {
  fn default() -> Self {
    Thermometer::new("/sys/class/thermal/thermal_zone0/temp")
  }
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub enum ThermometerError {
  ParseFloat(ParseFloatError),
  Utf8(str::Utf8Error),
  IO(io::Error),
}

#[cfg(not(target_os = "linux"))]
#[derive(Debug)]
pub enum ThermometerError {}

#[cfg(target_os = "linux")]
impl From<ParseFloatError> for ThermometerError {
  fn from(err: ParseFloatError) -> Self {
    Self::ParseFloat(err)
  }
}

#[cfg(target_os = "linux")]
impl From<str::Utf8Error> for ThermometerError {
  fn from(err: str::Utf8Error) -> Self {
    Self::Utf8(err)
  }
}

#[cfg(target_os = "linux")]
impl From<io::Error> for ThermometerError {
  fn from(err: io::Error) -> Self {
    Self::IO(err)
  }
}

#[cfg(target_os = "linux")]
impl fmt::Display for ThermometerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Thermometer Error")
  }
}
