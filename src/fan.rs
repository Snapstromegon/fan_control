#[cfg(target_os = "linux")]
use sysfs_gpio::{Direction, Pin};

#[cfg(target_os = "linux")]
pub struct Fan {
  pin: Pin,
}

#[cfg(not(target_os = "linux"))]
pub struct Fan {
  pin: u64,
  _is_on: FanMode,
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
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

#[cfg(target_os = "linux")]
impl Fan {
  pub fn new(pin: u64) -> Result<Self, sysfs_gpio::Error> {
    let pin = Pin::new(pin);
    pin.export()?;
    pin.set_direction(Direction::Out)?;
    Ok(Self { pin })
  }

  pub fn is_on(&self) -> Result<bool, sysfs_gpio::Error> {
    Ok(self.pin.get_value()? != 0)
  }

  pub fn turn_on(&self) -> Result<(), sysfs_gpio::Error> {
    self.pin.set_value(FanMode::On.into())
  }

  pub fn turn_off(&self) -> Result<(), sysfs_gpio::Error> {
    self.pin.set_value(FanMode::Off.into())
  }
}

#[cfg(not(target_os = "linux"))]
impl Fan {
  pub fn new(pin: u64) -> Result<Self, ()> {
    Ok(Self {
      pin,
      _is_on: FanMode::Off,
    })
  }

  pub fn is_on(&self) -> Result<bool, ()> {
    Ok(self._is_on == FanMode::On)
  }

  pub fn turn_on(&mut self) -> Result<(), ()> {
    println!("Turning Fan on Pin {} on", self.pin);
    self._is_on = FanMode::On;
    Ok(())
  }

  pub fn turn_off(&mut self) -> Result<(), ()> {
    println!("Turning Fan on Pin {} off", self.pin);
    self._is_on = FanMode::Off;
    Ok(())
  }
}
