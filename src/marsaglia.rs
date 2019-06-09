// https://www.jstatsoft.org/article/view/v008i14

// page 4
pub struct XorShift32 {
  state: i32,
}

impl XorShift32 {
  pub fn seeded(seed: i32) -> Self {
    return Self {
      state: seed,
    };
  }
}

impl std::iter::Iterator for XorShift32 {
  type Item = i32;
  fn next(&mut self) -> Option<Self::Item> {
    let mut x: i32 = self.state;
    x = x ^ (x << 13);
    x = x ^ (x >> 17);
    x = x ^ (x << 5);
    self.state = x;
    return Some(x);
  }
}

// page 4?
pub struct XorShift64 {
  state: i64,
}

impl XorShift64 {
  pub fn seeded(seed: i64) -> Self {
    return Self {
      state: seed,
    };
  }
}

impl std::iter::Iterator for XorShift64 {
  type Item = i64;
  fn next(&mut self) -> Option<Self::Item> {
    let mut x: i64 = self.state;
    x = x ^ (x << 13);
    x = x ^ (x >> 7);
    x = x ^ (x << 17);
    self.state = x;
    return Some(x);
  }
}

// page 5
pub struct XorShift128 {
  state: [i32; 4],
}

impl XorShift128 {
  pub fn seeded(seed: [i32; 4]) -> Self {
    return Self {
      state: seed,
    };
  }
}

impl std::iter::Iterator for XorShift128 {
  type Item = i32;
  fn next(&mut self) -> Option<Self::Item> {
    let copy: [i32; 4] = self.state;
    let mut s: i32 = self.state[3];
    let mut t: i32 = self.state[3];
    self.state[3] = copy[2];
    self.state[2] = copy[1];
    self.state[1] = s;
    s = copy[0];
    t = t ^ (t << 11);
    t = t ^ (t >> 8);
    self.state[0] = t ^ s ^ (s >> 19);
    return Some(self.state[0]);
  }
}
