#![allow(unused_parens)]
#![feature(test)]

extern crate test;
use test::Bencher;

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

fn bench_a_div_rem_fn(f: fn(u32, u32) -> (u32, u32)) {
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
#[bench]
fn bench_u32_div_rem_1(b: &mut Bencher) {
  b.iter(|| bench_a_div_rem_fn(u32_div_rem_1))
}
#[bench]
fn bench_u32_div_rem_2(b: &mut Bencher) {
  b.iter(|| bench_a_div_rem_fn(u32_div_rem_2))
}
#[bench]
fn bench_u32_div_rem_3(b: &mut Bencher) {
  b.iter(|| bench_a_div_rem_fn(u32_div_rem_3))
}
#[bench]
fn bench_u32_div_rem_4(b: &mut Bencher) {
  b.iter(|| bench_a_div_rem_fn(u32_div_rem_4))
}
#[bench]
fn bench_u32_div_rem_5(b: &mut Bencher) {
  b.iter(|| bench_a_div_rem_fn(u32_div_rem_5))
}
#[bench]
fn bench_u32_div_rem_6(b: &mut Bencher) {
  b.iter(|| bench_a_div_rem_fn(u32_div_rem_6))
}
#[bench]
fn bench_u32_div_rem_7(b: &mut Bencher) {
  b.iter(|| bench_a_div_rem_fn(u32_div_rem_7))
}
#[bench]
fn bench_u32_div_rem_8(b: &mut Bencher) {
  b.iter(|| bench_a_div_rem_fn(u32_div_rem_8))
}
#[bench]
#[cfg(target_arch = "arm")]
fn bench_aeabi_uidivmod_x(b: &mut Bencher) {
  fn aeabi_uidivmod_x(num: u32, denom: u32) -> (u32, u32) {
    unsafe {
      core::mem::transmute::<u64, (u32, u32)>(aeabi_uidivmod(num, denom))
    }
  }
  b.iter(|| bench_a_div_rem_fn(aeabi_uidivmod_x))
}
