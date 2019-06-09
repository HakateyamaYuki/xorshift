// http://xoshiro.di.unimi.it/

pub struct XoroShiro256StarStar {
  state: [i64; 4],
}

impl XoroShiro256StarStar {
  pub fn seeded(seed: [i64; 4]) -> Self {
    return Self {
      state: seed,
    };
  }
}

impl std::iter::Iterator for XoroShiro256StarStar {
  type Item = i64;
  fn next(&mut self) -> Option<Self::Item> {
    let result = (self.state[1] * 5).rotate_left(7) * 9;
    let t = self.state[1] << 17;

    self.state[2] ^= self.state[0];
    self.state[3] ^= self.state[1];
    self.state[1] ^= self.state[2];
    self.state[0] ^= self.state[3];

    self.state[2] ^= t;
    self.state[3] = self.state[3].rotate_left(45);
    return Some(result);
  }
}

pub struct XoroShiro256Plus {
  state: [i64; 4],
}

impl XoroShiro256Plus {
  pub fn seeded(seed: [i64; 4]) -> Self {
    return Self {
      state: seed,
    };
  }
}

impl std::iter::Iterator for XoroShiro256Plus {
  type Item = i64;
  fn next(&mut self) -> Option<Self::Item> {
    let result = self.state[0] + self.state[3];
    let t = self.state[1] << 17;

    self.state[2] ^= self.state[0];
    self.state[3] ^= self.state[1];
    self.state[1] ^= self.state[2];
    self.state[0] ^= self.state[3];

    self.state[2] ^= t;
    self.state[3] = self.state[3].rotate_left(45);
    return Some(result);
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn xoroshiro256plus() {
    use super::XoroShiro256Plus;
    let mut xs = XoroShiro256Plus::seeded([0x00499198a, 0x991a21, 0xaa884f8d1, 0x41af140b]);
    assert_eq!(xs.next().unwrap(), 1179135381);
  }

  #[test]
  fn xoroshiro256starstar() {
    use super::XoroShiro256StarStar;
    let mut xs = XoroShiro256StarStar::seeded([0x00499198a, 0x991a21, 0xaa884f8d1, 0x41af140b]);
    assert_eq!(xs.next().unwrap(), 57794094720);
  }
}