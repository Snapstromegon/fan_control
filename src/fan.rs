#[cfg(target_os = "linux")]
use sysfs_gpio::{Direction, Pin};

pub struct Fan {
  #[cfg(target_os = "linux")]
  pin: Pin,
  #[cfg(target_os = "windows")]
  pin: u64,
  #[cfg(target_os = "windows")]
  _is_on: FanMode,
}

#[derive(PartialEq)]
pub enum FanMode {
  Off,
  On,
}

impl From<FanMode> for u8 {
  fn from(mode: FanMode) -> u8 {
    match mode {
      FanMode::Off => 0,
      FanMode::On => 1,
    }
  }
}

impl Fan {
  #[cfg(target_os = "windows")]
  pub fn new(pin: u64) -> Result<Self, ()> {
    Ok(Self {
      pin,
      _is_on: FanMode::Off,
    })
  }

  #[cfg(target_os = "linux")]
  pub fn new(pin: u64) -> Result<Self, sysfs_gpio::Error> {
    let fan = Self { pin: Pin::new(pin) };

    fan.configure_pin()?;

    Ok(fan)
  }

  #[cfg(target_os = "linux")]
  fn configure_pin(&self) -> Result<(), sysfs_gpio::Error> {
    self.pin.export()?;
    self.pin.set_direction(Direction::Out)?;
    Ok(())
  }

  #[cfg(target_os = "linux")]
  pub fn is_on(&self) -> Result<bool, sysfs_gpio::Error> {
    Ok(self.pin.get_value()? != 0)
  }

  #[cfg(target_os = "windows")]
  pub fn is_on(&self) -> Result<bool, ()> {
    Ok(self._is_on == FanMode::On)
  }

  #[cfg(target_os = "linux")]
  pub fn turn_on(&self) -> Result<(), sysfs_gpio::Error> {
    self.pin.set_value(FanMode::On.into())
  }

  #[cfg(target_os = "windows")]
  pub fn turn_on(&mut self) -> Result<(), ()> {
    println!("Turning Fan on Pin {} on", self.pin);
    self._is_on = FanMode::On;
    Ok(())
  }

  #[cfg(target_os = "linux")]
  pub fn turn_off(&self) -> Result<(), sysfs_gpio::Error> {
    self.pin.set_value(FanMode::Off.into())
  }

  #[cfg(target_os = "windows")]
  pub fn turn_off(&mut self) -> Result<(), ()> {
    println!("Turning Fan on Pin {} off", self.pin);
    self._is_on = FanMode::Off;
    Ok(())
  }
}
