#[cfg(feature = "sysfs")]
use std::fmt;
#[cfg(feature = "sysfs")]
use std::fs::{read_dir, read_to_string};
#[cfg(feature = "sysfs")]
use std::io;
#[cfg(feature = "sysfs")]
use std::num::ParseFloatError;
#[cfg(feature = "sysfs")]
use std::str;

pub struct Thermometer {
  #[cfg(feature = "sysfs")]
  thermal_zone: String,
}

#[cfg(feature = "sysfs")]
impl Thermometer {
  pub fn new(thermal_zone: &str) -> Self {
    Thermometer {
      thermal_zone: thermal_zone.into(),
    }
  }

  pub fn read_temp(&self) -> Result<f64, ThermometerError> {
    let string_temp = read_to_string(&format!("/sys/class/thermal/{}/temp", self.thermal_zone))?;
    let temp_float: f64 = string_temp.trim().parse()?;
    Ok(temp_float / 1000.0)
  }

  pub fn get_available_thermal_zones() -> Result<Vec<String>, io::Error> {
    Ok(
      read_dir("/sys/class/thermal/")?
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().into_string())
        .filter_map(|e| e.ok())
        .collect(),
    )
  }
}
#[cfg(not(feature = "sysfs"))]
impl Thermometer {
  pub fn new(_thermal_zone: &str) -> Self {
    Thermometer {}
  }

  pub fn read_temp(&self) -> Result<f64, ThermometerError> {
    Ok(43.0)
  }
}

impl Default for Thermometer {
  fn default() -> Self {
    Thermometer::new("thermal_zone0")
  }
}

#[cfg(feature = "sysfs")]
#[derive(Debug)]
pub enum ThermometerError {
  ParseFloat(ParseFloatError),
  Utf8(str::Utf8Error),
  IO(io::Error),
}

#[cfg(not(feature = "sysfs"))]
#[derive(Debug)]
pub enum ThermometerError {}

#[cfg(feature = "sysfs")]
impl From<ParseFloatError> for ThermometerError {
  fn from(err: ParseFloatError) -> Self {
    Self::ParseFloat(err)
  }
}

#[cfg(feature = "sysfs")]
impl From<str::Utf8Error> for ThermometerError {
  fn from(err: str::Utf8Error) -> Self {
    Self::Utf8(err)
  }
}

#[cfg(feature = "sysfs")]
impl From<io::Error> for ThermometerError {
  fn from(err: io::Error) -> Self {
    Self::IO(err)
  }
}

#[cfg(feature = "sysfs")]
impl fmt::Display for ThermometerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Thermometer Error")
  }
}
