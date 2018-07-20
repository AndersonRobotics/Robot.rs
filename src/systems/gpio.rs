use std::fs::File;
use std::io::Write;

pub struct DigitalChip {
  export_handle: File,
  remove_handle: File,
}

pub struct AnalogChip {
  export_handle: File,
  remove_handle: File,
}

pub struct DigitalLine {
  value_handle: File,
  direc_handle: File,
}

pub struct AnalogLine {
  period_handle: File,
  dCycle_handle: File,
}


impl DigitalChip {
  fn init () -> DigitalChip {
    DigitalChip {
      export_handle: File::open("/sys/class/gpio/export").unwrap(),
      remove_handle: File::open("/sys/class/gpio/unexport").unwrap(),
    }
  }

  pub fn export (&mut self, line: u8) -> DigitalLine {
    self.export_handle.write(format!("{}", line).as_bytes()).unwrap();
    let value_path = format!("/sys/class/gpio/gpio{}/value", line);
    let direc_path = format!("/sys/class/gpio/gpio{}/direction", line);

    DigitalLine {
      value_handle: File::open(value_path).unwrap(),
      direc_handle: File::open(direc_path).unwrap(),
    }
  }

  pub fn remove (&mut self, line: u8) {
    self.remove_handle.write(format!("{}", line).as_bytes()).unwrap();
  }
}


impl AnalogChip {
  fn init () -> AnalogChip {
    AnalogChip {
      export_handle: File::open("/sys/class/pwm/pwmchip0/export").unwrap(),
      remove_handle: File::open("/sys/class/pwm/pwmchip0/unexport").unwrap(),
    }
  }

  pub fn export (&mut self, line: u8) -> AnalogLine {
    self.export_handle.write(format!("{}", line).as_bytes()).unwrap();
    let period_path = format!("/sys/class/pwm/pwmchip0/pwm{}/period", line);
    let dCycle_path = format!("/sys/class/pwm/pwmchip0/pwm{}/duty_cycle", line);

    AnalogLine {
      period_handle: File::open(period_path).unwrap(),
      dCycle_handle: File::open(dCycle_path).unwrap(),
    }
  }

  pub fn remove (&mut self, line: u8) {
    self.remove_handle.write(format!("{}", line).as_bytes()).unwrap();
  } 
}


impl DigitalLine {
  pub fn set_high (&mut self) {
    self.value_handle.write("1".as_bytes()).unwrap();
  }

  pub fn set_low (&mut self) {
    self.value_handle.write("0".as_bytes()).unwrap();
  }

  pub fn set_input (&mut self) {
    self.direc_handle.write("in".as_bytes()).unwrap();
  }

  pub fn set_output (&mut self) {
    self.direc_handle.write("out".as_bytes()).unwrap();
  }
}


impl AnalogLine {
  pub fn set_period (&mut self, ns: u16) {
    self.period_handle.write(format!("{}", ns).as_bytes()).unwrap();
  }

  pub fn set_duty (&mut self, ns: u16) {
    self.dCycle_handle.write(format!("{}", ns).as_bytes()).unwrap();
  }
}


lazy_static! {
  static ref DIGITAL_CHIP: DigitalChip = DigitalChip::init();
  static ref ANALOG_CHIP: AnalogChip = AnalogChip::init();
}