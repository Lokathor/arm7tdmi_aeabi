#![allow(dead_code)]
#![cfg(target_arch = "arm")]

core::arch::global_asm!(include_str!("../src/the_code.s"), options(raw));

include!("../src/fn_declarations.rs");

fn rand_bytes(n: usize) -> Vec<u8> {
  let mut v = vec![0; n];
  getrandom::getrandom(&mut v).unwrap();
  v
}

fn rand_halfwords(n: usize) -> Vec<u16> {
  let mut v = vec![0_u16; n];
  getrandom::getrandom(bytemuck::cast_slice_mut(&mut v)).unwrap();
  v
}

fn rand_words(n: usize) -> Vec<u32> {
  let mut v = vec![0_u32; n];
  getrandom::getrandom(bytemuck::cast_slice_mut(&mut v)).unwrap();
  v
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

#[test]
fn test_libc_memmove() {
  for d in 0..8 {
    for s in 0..8 {
      if d == s {
        continue;
      }
      for bytes in 0..100 {
        let mut buffer = rand_bytes(128);
        let mut clone = buffer.clone();
        unsafe {
          let p: *mut u8 = buffer.as_mut_ptr();
          let out = libc_memmove(p.add(d), p.add(s), bytes);
          assert_eq!(p.add(d), out);
        }
        clone.copy_within(s..(s + bytes), d);
        assert_eq!(
          clone,
          buffer,
          "\nd: {d:?},\ns: {s:?},\nbytes: {bytes}",
          d = unsafe { buffer.as_ptr().add(d) },
          s = unsafe { buffer.as_ptr().add(s) },
        );
      }
    }
  }

  let mut lcg = Lcg::new();
  for _ in 0..100 {
    for bytes in 0..128 {
      let mut buffer = rand_bytes(256);
      let mut clone = buffer.clone();
      let d = (lcg.next_u32() % 128) as usize;
      let s = (lcg.next_u32() % 128) as usize;
      unsafe {
        let p: *mut u8 = buffer.as_mut_ptr();
        let out = libc_memmove(p.add(d), p.add(s), bytes);
        assert_eq!(p.add(d), out);
      }
      clone.copy_within(s..(s + bytes), d);
      assert_eq!(clone, buffer, "\nd: {d},\ns: {s},\nbytes: {bytes}");
    }
  }
}

#[test]
fn test_libc_memset() {
  for count in 0..99 {
    for d in 0..8 {
      if d >= count {
        continue;
      }
      let byte = count as i32;
      let mut v = vec![0_u8; count];
      let out = unsafe { libc_memset(v.as_mut_ptr().add(d), byte, count - d) };
      assert_eq!(unsafe { v.as_ptr().add(d) }, out);
      assert!(
        v[d..].iter().all(|&b| (b as i32) == byte),
        "\n=dest: {d:?},\n=count: {count},\n=byte: {byte},\nv: {v:?}\n",
        d = v.as_ptr(),
      );
    }
  }
}

#[test]
fn test_aeabi_uread4() {
  let v: Vec<u8> = (1..).take(16).collect();
  for x in 0..8 {
    let expected = u32::from_ne_bytes(v[x..(x + 4)].try_into().unwrap());
    let actual = unsafe { aeabi_uread4(v.as_ptr().add(x).cast::<u32>()) };
    assert_eq!(expected, actual);
  }
}

#[test]
fn test_aeabi_uread8() {
  let v: Vec<u8> = (1..).take(32).collect();
  for x in 0..16 {
    let expected = u64::from_ne_bytes(v[x..(x + 8)].try_into().unwrap());
    let actual = unsafe { aeabi_uread8(v.as_ptr().add(x).cast::<u64>()) };
    assert_eq!(expected, actual);
  }
}

#[test]
fn test_aeabi_uwrite4() {
  let mut buffer: Vec<u8> = (1..).take(16).collect();
  let mut clone: Vec<u8> = buffer.clone();
  for x in 0..8 {
    let u: u32 = 0x7799AABB;
    clone[x..(x + 4)].copy_from_slice(&u.to_ne_bytes());
    let out =
      unsafe { aeabi_uwrite4(u, buffer.as_mut_ptr().add(x).cast::<u32>()) };
    assert_eq!(out, u);
    assert_eq!(buffer, clone);
  }
}

#[test]
fn test_aeabi_uwrite8() {
  let mut buffer: Vec<u8> = (1..).take(32).collect();
  let mut clone: Vec<u8> = buffer.clone();
  for x in 0..8 {
    let u: u64 = 0x7799AABB_CCDDEEFF;
    clone[x..(x + 8)].copy_from_slice(&u.to_ne_bytes());
    let out =
      unsafe { aeabi_uwrite8(u, buffer.as_mut_ptr().add(x).cast::<u64>()) };
    assert_eq!(out, u);
    assert_eq!(buffer, clone);
  }
}

#[test]
fn test_aeabi_idiv() {
  let mut lcg = Lcg::new();
  for denom in [-1, -2, -3, -4, -5] {
    let num = i32::MIN;
    let expected = num.wrapping_div(denom);
    let actual = unsafe { aeabi_idiv(num, denom) };
    assert_eq!(expected, actual);
  }
  for _ in 0..100 {
    let num = lcg.next_u32() as i32;
    let denom = lcg.next_u32() as i32;
    if denom == 0 {
      continue;
    }
    let expected = num / denom;
    let actual = unsafe { aeabi_idiv(num, denom) };
    assert_eq!(
      expected, actual,
      "\nnum: {num},\ndenom: {denom},\nexpected: {expected},\nactual: {actual}"
    );
  }
}

#[test]
fn test_aeabi_uidiv() {
  let mut lcg = Lcg::new();
  for denom in [u32::MAX, u32::MAX - 1, u32::MAX - 2] {
    let num = u32::MAX;
    let expected = num / denom;
    let actual = unsafe { aeabi_uidiv(num, denom) };
    assert_eq!(expected, actual);
  }
  for _ in 0..100 {
    let num = lcg.next_u32();
    let denom = lcg.next_u32();
    if denom == 0 {
      continue;
    }
    let expected = num / denom;
    let actual = unsafe { aeabi_uidiv(num, denom) };
    assert_eq!(
      expected, actual,
      "\nnum: {num},\ndenom: {denom},\nexpected: {expected},\nactual: {actual}"
    );
  }
}

#[test]
fn test_aeabi_idivmod() {
  let mut lcg = Lcg::new();
  {
    let num = i32::MIN;
    let denom = -1;
    let expected = [num.wrapping_div(denom), num.wrapping_rem(denom)];
    let actual: [i32; 2] = unsafe {
      core::mem::transmute::<u64, [i32; 2]>(aeabi_idivmod(num, denom))
    };
    assert_eq!(expected, actual);
  }
  for _ in 0..100 {
    let num = lcg.next_u32() as i32;
    let denom = lcg.next_u32() as i32;
    if denom == 0 {
      continue;
    }
    let expected = [num / denom, num % denom];
    let actual: [i32; 2] = unsafe {
      core::mem::transmute::<u64, [i32; 2]>(aeabi_idivmod(num, denom))
    };
    assert_eq!(expected, actual,
      "\nnum: {num},\ndenom: {denom},\nexpected: {expected:?},\nactual: {actual:?}"
    );
  }
}

#[test]
fn test_aeabi_uidivmod() {
  let mut lcg = Lcg::new();
  for _ in 0..100 {
    let num = lcg.next_u32();
    let denom = lcg.next_u32();
    if denom == 0 {
      continue;
    }
    let expected = [num / denom, num % denom];
    let actual: [u32; 2] = unsafe {
      core::mem::transmute::<u64, [u32; 2]>(aeabi_uidivmod(num, denom))
    };
    assert_eq!(expected, actual,
      "\nnum: {num},\ndenom: {denom},\nexpected: {expected:?},\nactual: {actual:?}"
    );
  }
}
