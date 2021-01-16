pub struct Math;

impl Math {
  pub fn log10(n: u32) -> u32 {
    if n == 0 {
      0
    } else {
      1 + Math::log10(n / 10)
    }
  }
}