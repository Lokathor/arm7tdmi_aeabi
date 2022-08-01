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

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
  r#".section ".text.aeabi.idiv"
  aeabi_idiv:
      cmp   r1, #0
      beq   aeabi_idiv0
      push  {r4, lr}
      eor   r4, r1, r0
      cmp   r0, #0
      rsblt r0, r0, #0
      cmp   r1, #0
      rsclt r1, r1, #0
      bl    .L_aeabi_uidiv_skip_zero_check
      cmp   r4, #0
      rsblt r0, r0, #0
      pop   {r4, lr}
      bx    lr
  .previous
  
  .section ".text.aeabi.uidiv"
  aeabi_uidiv: @ r0=num, r1=denom
      cmp   r1, #0
      beq   aeabi_idiv0
    .L_aeabi_uidiv_skip_zero_check:
      mov   r3, r1         @ r3(shifted_denom) = denom
      cmp   r3, r0, lsr #1
    2: @ left shift loop to line up m-s-bit of num and denom
      lslls r3, r3, #1     @ if shifted_denom < (num>>1): shifted_denom =<< 1;
      cmp   r3, r0, lsr #1
      bls   2b
      mov   r2, #0         @ r0=num, r1=denom, r2=quot(init 0), r3=shifted_denom
    3: @ subtraction loop
      cmp   r0, r3
      subcs r0, r0, r3     @ if no_underflow(num-shifted_denom): num -= shifted_denom;
      adc   r2, r2, r2     @ quot = 2*quot + no_underflow(num-shifted_denom)
      mov   r3, r3, lsr #1 @ shifted_denom >>= 1;
      cmp   r3, r1
      bcs   3b             @ if no_underflow(shifted_denom - denom): continue
      mov   r0, r2
      bx    lr
  .previous
  
  .section ".text.aeabi.idivmod"
  aeabi_idivmod:
      cmp   r1, #0
      beq   aeabi_idiv0
      push  {r4, r5, lr} @ temporarily mis-aligned stack!
      movs  r4, r0       @ store real num
      rsblt r0, r0, #0   @ num = abs(num)
      movs  r5, r1       @ store real denom
      rsblt r1, r1, #0   @ denom = abs(denom)
      bl    .L_aeabi_uidivmod_skip_zero_check
      eors  r12, r4, r5  @ num_sign != denom_sign: quot is negative
      rsblt r0, r0, #0
      cmp   r4, #0       @ num < 0: rem is neg
      rsblt r1, r1, #0
      pop   {r4, r5, lr}
      bx    lr
  .previous
  
  .section ".text.aeabi.uidivmod"
  aeabi_uidivmod: @ r0=num, r1=denom
      cmp   r1, #0
      beq   aeabi_idiv0
    .L_aeabi_uidivmod_skip_zero_check:
      mov   r12, r0 @ r12=num
      push  {r1, lr} @ ASSUMES UIDIV DOES NOT USE REGISTER 12!
      bl    .L_aeabi_uidiv_skip_zero_check @ r0=quot
      pop   {r1, lr}
      mul   r2, r0, r1 @ r2=quot*denom
      sub   r1, r12, r2 @ r1=num-(quot*denom)
      bx    lr
  .previous
  
  .section ".text.aeabi.idiv0"
  aeabi_idiv0:
      mov r1, r0
      mov r0, #0
      bx  lr
  .previous
"#,
  options(raw)
);

extern "C" {
  /// Performs `i32 / i32` division.
  ///
  /// This is part of the AEABI [integer division][aeabi-int-div] API.
  ///
  /// [aeabi-int-div]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
  ///
  /// * If `denominator` is 0, then the return value is 0.
  ///
  /// ## Safety
  /// * This is safe for all possible input values, Rust just has no simple way
  ///   to declare that an `extern "C"` function is always safe.
  pub fn aeabi_idiv(numerator: i32, denominator: i32) -> i32;

  /// Performs `u32 / u32` division.
  ///
  /// This is part of the AEABI [integer division][aeabi-int-div] API.
  ///
  /// [aeabi-int-div]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
  ///
  /// * If `denominator` is 0, then the return value is 0.
  ///
  /// ## Safety
  /// * This is safe for all possible input values, Rust just has no simple way
  ///   to declare that an `extern "C"` function is always safe.
  pub fn aeabi_uidiv(numerator: u32, denominator: u32) -> u32;

  /// Performs `[i32 / i32, i32 % i32]`, returning the data packed in a `u64`.
  ///
  /// The return value is stored in the `r0` and `r1` registers. For ABI
  /// reasons, the only way that we can get an Rust `extern "C"` function
  /// declaration to grab both of those registers as the return value is to
  /// declare the return type as a 64-bit integer. The quotent will be the lower
  /// 32 bits, and the remainder will be the upper 32 bits.
  ///
  /// This is part of the AEABI [integer division][aeabi-int-div] API.
  ///
  /// [aeabi-int-div]:
  ///     https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
  ///
  /// * If `denominator` is 0, then the return value is `[0, numerator]`.
  ///
  /// ## Safety
  /// * This is safe for all possible input values, Rust just has no simple way
  ///   to declare that an `extern "C"` function is always safe.
  pub fn aeabi_idivmod(numerator: i32, denominator: i32) -> u64;

  /// Performs `[u32 / u32, u32 % u32]`, returning the data packed in a `u64`.
  ///
  /// The return value is stored in the `r0` and `r1` registers. For ABI
  /// reasons, the only way that we can get an Rust `extern "C"` function
  /// declaration to grab both of those registers as the return value is to
  /// declare the return type as a 64-bit integer. The quotent will be the lower
  /// 32 bits, and the remainder will be the upper 32 bits.
  ///
  /// This is part of the AEABI [integer division][aeabi-int-div] API.
  ///
  /// [aeabi-int-div]:
  ///     https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
  ///
  /// * If `denominator` is 0, then the return value is `[0, numerator]`.
  ///
  /// ## Safety
  /// * This is safe for all possible input values, Rust just has no simple way
  ///   to declare that an `extern "C"` function is always safe.
  pub fn aeabi_uidivmod(numerator: u32, denominator: u32) -> u64;
}
