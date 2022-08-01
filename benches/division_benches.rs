#![allow(unused_parens)]

use criterion::{criterion_group, criterion_main, Criterion};

criterion_group!(
  benches,
  test_u32_div_rem_1,
  test_u32_div_rem_2,
  test_u32_div_rem_3,
  test_u32_div_rem_4,
  test_u32_div_rem_5,
  test_u32_div_rem_6,
  test_u32_div_rem_7,
  test_u32_div_rem_8
);
criterion_main!(benches);

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

fn test_u32_div_rem_1(c: &mut Criterion) {
  c.bench_function("u32_div_rem_1", |b| {
    b.iter(|| bench_a_div_rem_fn(u32_div_rem_1))
  });
}

fn test_u32_div_rem_2(c: &mut Criterion) {
  c.bench_function("u32_div_rem_2", |b| {
    b.iter(|| bench_a_div_rem_fn(u32_div_rem_2))
  });
}

fn test_u32_div_rem_3(c: &mut Criterion) {
  c.bench_function("u32_div_rem_3", |b| {
    b.iter(|| bench_a_div_rem_fn(u32_div_rem_3))
  });
}

fn test_u32_div_rem_4(c: &mut Criterion) {
  c.bench_function("u32_div_rem_4", |b| {
    b.iter(|| bench_a_div_rem_fn(u32_div_rem_4))
  });
}

fn test_u32_div_rem_5(c: &mut Criterion) {
  c.bench_function("u32_div_rem_5", |b| {
    b.iter(|| bench_a_div_rem_fn(u32_div_rem_5))
  });
}

fn test_u32_div_rem_6(c: &mut Criterion) {
  c.bench_function("u32_div_rem_6", |b| {
    b.iter(|| bench_a_div_rem_fn(u32_div_rem_6))
  });
}

fn test_u32_div_rem_7(c: &mut Criterion) {
  c.bench_function("u32_div_rem_7", |b| {
    b.iter(|| bench_a_div_rem_fn(u32_div_rem_7))
  });
}

fn test_u32_div_rem_8(c: &mut Criterion) {
  c.bench_function("u32_div_rem_8", |b| {
    b.iter(|| bench_a_div_rem_fn(u32_div_rem_8))
  });
}
