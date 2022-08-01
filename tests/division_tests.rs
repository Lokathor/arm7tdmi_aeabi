#![allow(dead_code)]
#![allow(unused_parens)]

fn u32_normalization_shift(duo: u32, divisor: u32) -> usize {
  let mut shl: usize;
  {
    let mut test = duo;
    shl = 0usize;
    let mut lvl = 32 >> 1;
    loop {
      let tmp = test >> lvl;
      if divisor <= tmp {
        test = tmp;
        shl += lvl;
      }
      lvl >>= 1;
      if lvl == 0 {
        break;
      }
    }
  }
  shl
}

pub fn u32_div_rem_1(duo: u32, div: u32) -> (u32, u32) {
  let mut duo = duo;
  // handle edge cases before calling `u32_normalization_shift`
  if div == 0 {
    return (0, div);
  }
  if duo < div {
    return (0, duo);
  }
  let div_original = div;
  let mut shl = u32_normalization_shift(duo, div);
  let mut quo = 0;
  loop {
    let div_shifted = div << shl;
    let sub = duo.wrapping_sub(div_shifted);
    if 0 <= (sub as i32) {
      duo = sub;
      quo += 1 << shl;
      if duo < div_original {
        // this branch is optional
        return (quo, duo);
      }
    }
    if shl == 0 {
      return (quo, duo);
    }
    shl -= 1;
  }
}

pub fn u32_div_rem_2(duo: u32, div: u32) -> (u32, u32) {
  let mut duo = duo;
  // handle edge cases before calling `32ormalization_shift`
  if div == 0 {
    return (0, div);
  }
  if duo < div {
    return (0, duo);
  }

  let div_original = div;
  let shl = u32_normalization_shift(duo, div);
  let mut div: u32 = div << shl;
  let mut pow: u32 = 1 << shl;
  let mut quo: u32 = 0;
  loop {
    let sub = duo.wrapping_sub(div);
    if 0 <= (sub as i32) {
      duo = sub;
      quo |= pow;
      if duo < div_original {
        return (quo, duo);
      }
    }
    div >>= 1;
    pow >>= 1;
  }
}

pub fn u32_div_rem_3(duo: u32, div: u32) -> (u32, u32) {
  let mut duo = duo;
  // handle edge cases before calling `32ormalization_shift`
  if div == 0 {
    return (0, div);
  }
  if duo < div {
    return (0, duo);
  }

  let div_original = div;
  let mut shl = u32_normalization_shift(duo, div);
  let mut div: u32 = (div << shl);
  let mut quo: u32 = 1;
  duo = duo.wrapping_sub(div);
  if duo < div_original {
    return (1 << shl, duo);
  }
  let div_neg: u32;
  if (div as i32) < 0 {
    div >>= 1;
    div_neg = div.wrapping_neg();
    let (sub, carry) = duo.overflowing_add(div_neg);
    duo = sub;
    quo = quo.wrapping_add(quo).wrapping_add(carry as u32);
    if !carry {
      duo = duo.wrapping_add(div);
    }
    shl -= 1;
  } else {
    div_neg = div.wrapping_neg();
  }
  let mut i = shl;
  loop {
    if i == 0 {
      break;
    }
    i -= 1;
    let (sub, carry) = duo.wrapping_shl(1).overflowing_add(div_neg);
    duo = sub;
    quo = quo.wrapping_add(quo).wrapping_add(carry as u32);
    if !carry {
      duo = duo.wrapping_add(div);
    }
  }
  return (quo, duo >> shl);
}

pub fn u32_div_rem_4(duo: u32, div: u32) -> (u32, u32) {
  let mut duo = duo;
  if div == 0 {
    return (0, div);
  }
  if duo < div {
    return (0, duo);
  }

  let div_original = div;
  let mut shl = u32_normalization_shift(duo, div);
  let mut div: u32 = (div << shl);
  duo = duo.wrapping_sub(div);
  let mut quo: u32 = 1 << shl;
  if duo < div_original {
    return (quo, duo);
  }
  let mask: u32;
  if (div as i32) < 0 {
    div >>= 1;
    shl -= 1;
    let tmp = 1 << shl;
    mask = tmp - 1;
    let sub = duo.wrapping_sub(div);
    if (sub as i32) >= 0 {
      // restore
      duo = sub;
      quo |= tmp;
    }
    if duo < div_original {
      return (quo, duo);
    }
  } else {
    mask = quo - 1;
  }
  let div: u32 = div.wrapping_sub(1);
  let mut i = shl;
  loop {
    if i == 0 {
      break;
    }
    i -= 1;
    duo = duo.wrapping_shl(1).wrapping_sub(div);
    if (duo as i32) < 0 {
      // restore
      duo = duo.wrapping_add(div);
    }
  }
  // unpack the results of SWAR
  return ((duo & mask) | quo, duo >> shl);
}

pub fn u32_div_rem_5(duo: u32, div: u32) -> (u32, u32) {
  let mut duo = duo;
  if div == 0 {
    return (0, div);
  }
  if duo < div {
    return (0, duo);
  }
  let shl = u32_normalization_shift(duo, div);
  let mut div: u32 = div << shl;
  let mut pow: u32 = 1 << shl;
  let mut quo: u32 = 0;
  loop {
    let sub = duo.wrapping_sub(div);
    let sign_mask = !((sub as i32).wrapping_shr(32 - 1) as u32);
    duo -= div & sign_mask;
    quo |= pow & sign_mask;
    div >>= 1;
    pow >>= 1;
    if pow == 0 {
      break;
    }
  }
  return (quo, duo);
}

pub fn u32_div_rem_6(duo: u32, div: u32) -> (u32, u32) {
  let mut duo = duo;
  if div == 0 {
    return (0, div);
  }
  if duo < div {
    return (0, duo);
  }
  let div_original = div;
  let mut shl = u32_normalization_shift(duo, div);
  let mut div: u32 = (div << shl);
  duo = duo.wrapping_sub(div);
  let mut quo: u32 = 1 << shl;
  if duo < div_original {
    return (quo, duo);
  }
  let mask: u32;
  if (div as i32) < 0 {
    div >>= 1;
    shl -= 1;
    let tmp = 1 << shl;
    mask = tmp - 1;
    let sub = duo.wrapping_sub(div);
    if (sub as i32) >= 0 {
      duo = sub;
      quo |= tmp;
    }
    if duo < div_original {
      return (quo, duo);
    }
  } else {
    mask = quo - 1;
  }

  // central loop
  div = div.wrapping_sub(1);
  let mut i = shl;
  loop {
    if i == 0 {
      break;
    }
    i -= 1;
    // shift left 1 and subtract
    duo = duo.wrapping_shl(1).wrapping_sub(div);
    // create mask
    let mask = (duo as i32).wrapping_shr(32 - 1) as u32;
    // restore
    duo = duo.wrapping_add(div & mask);
  }
  // unpack
  return ((duo & mask) | quo, duo >> shl);
}

pub fn u32_div_rem_7(duo: u32, div: u32) -> (u32, u32) {
  let mut duo = duo;
  if div == 0 {
    return (0, div);
  }
  if duo < div {
    return (0, duo);
  }
  let div_original = div;
  let shl = u32_normalization_shift(duo, div);
  let mut div: u32 = (div << shl);
  let mut pow: u32 = 1 << shl;
  let mut quo: u32 = pow;
  duo = duo.wrapping_sub(div);
  if duo < div_original {
    return (quo, duo);
  }
  div >>= 1;
  pow >>= 1;
  loop {
    if (duo as i32) < 0 {
      // Negated binary long division step.
      duo = duo.wrapping_add(div);
      quo = quo.wrapping_sub(pow);
    } else {
      // Normal long division step.
      if duo < div_original {
        return (quo, duo);
      }
      duo = duo.wrapping_sub(div);
      quo = quo.wrapping_add(pow);
    }
    pow >>= 1;
    div >>= 1;
  }
}

pub fn u32_div_rem_8(duo: u32, div: u32) -> (u32, u32) {
  let mut duo = duo;
  if div == 0 {
    return (0, div);
  }
  if duo < div {
    return (0, duo);
  }
  // SWAR opening
  let div_original = div;
  let mut shl = u32_normalization_shift(duo, div);
  let mut div: u32 = (div << shl);
  duo = duo.wrapping_sub(div);
  let mut quo: u32 = 1 << shl;
  if duo < div_original {
    return (quo, duo);
  }
  let mask: u32;
  if (div as i32) < 0 {
    div >>= 1;
    shl -= 1;
    let tmp = 1 << shl;
    let sub = duo.wrapping_sub(div);
    if (sub as i32) >= 0 {
      // restore
      duo = sub;
      quo |= tmp;
    }
    if duo < div_original {
      return (quo, duo);
    }
    mask = tmp - 1;
  } else {
    mask = quo - 1;
  }
  // central loop
  let div: u32 = div.wrapping_sub(1);
  let mut i = shl;
  loop {
    if i == 0 {
      break;
    }
    i -= 1;
    // note: the `wrapping_shl(1)` can be factored out, but would require
    // another restoring division step to prevent `(duo as i32)` from
    // overflowing
    if (duo as i32) < 0 {
      // Negated binary long division step.
      duo = duo.wrapping_shl(1).wrapping_add(div);
    } else {
      // Normal long division step.
      duo = duo.wrapping_shl(1).wrapping_sub(div);
    }
  }
  if (duo as i32) < 0 {
    // Restore. This was not needed in the original nonrestoring algorithm
    // because of the `duo < div_original` checks.
    duo = duo.wrapping_add(div);
  }
  // unpack
  return ((duo & mask) | quo, duo >> shl);
}

fn rand_u32() -> u32 {
  let mut bytes = [0; 4];
  getrandom::getrandom(&mut bytes).unwrap();
  u32::from_ne_bytes(bytes)
}

struct Lcg(u32);
impl Lcg {
  fn new() -> Self {
    Self(rand_u32())
  }
  fn next_u32(&mut self) -> u32 {
    self.0 = self.0.wrapping_mul(747796405).wrapping_add(1);
    self.0
  }
}

fn run_div_rem_tests_on(f: fn(u32, u32) -> (u32, u32)) {
  for num in [0, 1, 2, 3, u32::MAX, u32::MAX - 1, u32::MAX - 2, u32::MAX - 3] {
    for denom in
      [1, 2, 3, 4, 5, u32::MAX, u32::MAX - 1, u32::MAX - 2, u32::MAX - 3]
    {
      let expected = (num / denom, num % denom);
      let actual = f(num, denom);
      assert_eq!(expected, actual);
    }
  }
  let mut lcg = Lcg::new();
  for _ in 0..20 {
    let num = lcg.next_u32();
    let denom = lcg.next_u32();
    if denom == 0 {
      continue;
    }
    let expected = (num / denom, num % denom);
    let actual = f(num, denom);
    assert_eq!(expected, actual);
  }
}

#[test]
fn test_u32_div_rem_1() {
  run_div_rem_tests_on(u32_div_rem_1);
}
#[test]
fn test_u32_div_rem_2() {
  run_div_rem_tests_on(u32_div_rem_2);
}
#[test]
fn test_u32_div_rem_3() {
  run_div_rem_tests_on(u32_div_rem_3);
}
#[test]
fn test_u32_div_rem_4() {
  run_div_rem_tests_on(u32_div_rem_4);
}
#[test]
fn test_u32_div_rem_5() {
  run_div_rem_tests_on(u32_div_rem_5);
}
#[test]
fn test_u32_div_rem_6() {
  run_div_rem_tests_on(u32_div_rem_6);
}
#[test]
fn test_u32_div_rem_7() {
  run_div_rem_tests_on(u32_div_rem_7);
}
#[test]
fn test_u32_div_rem_8() {
  run_div_rem_tests_on(u32_div_rem_8);
}
