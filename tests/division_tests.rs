#![allow(dead_code)]
#![allow(unused_parens)]

include!("../src/division_fns.rs");

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
