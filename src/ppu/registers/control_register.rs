use bitflags::bitflags;

bitflags! {
  #[derive(Default)]
  pub struct ControlRegister: u8 {
    const NAME_TABLE_1 =          0b0000_0001;
    const NAME_TABLE_2 =          0b0000_0010;
    const VRAM_ADD_INCREMENT =    0b0000_0100;
    const SPRITE_PATTERN_ADDR =   0b0000_1000;
    const BG_PATTERN_ADDR =       0b0001_0000;
    const SPRITE_SIZE =           0b0010_0000;
    const MASTER_SLAVE_SELECT =   0b0100_0000;
    const GENERATE_NMI =          0b1000_0000;
  }
}

impl ControlRegister {

  pub fn new() -> Self {
    ControlRegister::from_bits_truncate(0)
  }

  // ? There has to be a better way to do this
  // ? Probably somewhere in the bitflags documentation
  pub fn update(&mut self, data: u8) {
    self.remove(ControlRegister::from_bits_truncate(self.bits()));
    self.insert(ControlRegister::from_bits_truncate(data));
  }

  pub fn get_vram_addr_increment(&self) -> u8 {
    if self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
      32
    } else {
      1
    }
  }

  pub fn should_generate_vblank_nmi(&self) -> bool {
    self.contains(ControlRegister::GENERATE_NMI)
  }

  pub fn background_pattern_address(&self) -> u16 {
    if self.contains(ControlRegister::BG_PATTERN_ADDR) {
      0x1000
    } else {
      0
    }
  }

  pub fn sprite_pattern_address(&self) -> u16 {
    if self.contains(ControlRegister::SPRITE_PATTERN_ADDR) {
      0x1000
    } else {
      0
    }
  }

}

#[cfg(test)]
mod tests{

  use super::*;

  #[test]
  fn test_update() {

    let mut reg = ControlRegister::default();

    let dummy_status = 0b1010_1010;
    reg.update(dummy_status);

    let new_status = 0b0101_0101;
    reg.update(new_status);

    assert_eq!(reg.bits(), new_status)

  }

  #[test]
  fn test_get_vram_addr_increment() {

    let mut reg = ControlRegister::default();
    assert_eq!(reg.get_vram_addr_increment(), 1);

    reg.insert(ControlRegister::VRAM_ADD_INCREMENT);
    assert_eq!(reg.get_vram_addr_increment(), 32);

  }

  #[test]
  fn test_get_background_pattern_addr() {

    let mut reg = ControlRegister::default();
    assert_eq!(reg.background_pattern_address(), 0);

    reg.insert(ControlRegister::BG_PATTERN_ADDR);
    assert_eq!(reg.background_pattern_address(), 0x1000);

  }

  #[test]
  fn test_get_sprite_pattern_addr() {

    let mut reg = ControlRegister::default();
    assert_eq!(reg.sprite_pattern_address(), 0);

    reg.insert(ControlRegister::SPRITE_PATTERN_ADDR);
    assert_eq!(reg.sprite_pattern_address(), 0x1000);

  }

}