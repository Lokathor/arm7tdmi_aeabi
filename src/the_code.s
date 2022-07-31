
.global libc_memcpy
.global aeabi_memcpy
.global aeabi_memcpy4
.global aeabi_memcpy8
.global gba_memcpy_sram

.global libc_memmove
.global aeabi_memmove
.global aeabi_memmove4
.global aeabi_memmove8

.global libc_memset
.global aeabi_memset
.global aeabi_memset4
.global aeabi_memset8

.global aeabi_memclr
.global aeabi_memclr4
.global aeabi_memclr8

.global aeabi_uread4
.global aeabi_uwrite4
.global aeabi_uread8
.global aeabi_uwrite8

.global aeabi_idiv
.global aeabi_uidiv
.global aeabi_idivmod
.global aeabi_uidivmod
.global aeabi_idiv0
.global aeabi_ldiv0

.section ".text.libc.memcpy"
libc_memcpy:
    push   {r0, lr}
    bl     aeabi_memcpy
    pop    {r0, lr}
    bx     lr
.previous

.section ".text.libc.memmove"
libc_memmove:
    push   {r0, lr}
    bl     aeabi_memmove
    pop    {r0, lr}
    bx     lr
.previous

.section ".text.aeabi.memcpy"
aeabi_memmove8:
aeabi_memmove4:
aeabi_memmove:
    cmp    r0, r1 @ if d > s, reverse copy
    bgt    .L_r_copy_gain_align
    @ else fallthrough
aeabi_memcpy:
  .L_f_copy_gain_align:
    eor    r3, r0, r1
    lsls   r3, r3, #31
    bmi    .L_f_copy_max_coalign1
    bcs    .L_f_copy_max_coalign2
    @ else fallthrough
  .L_f_copy_max_coalign4:
    tst    r0, #3
    bne    .L_f_copy_fixup4
    @ else fallthrough
aeabi_memcpy8:
aeabi_memcpy4:
  .L_f_copy_coalign4_assured:
    cmp    r2, #32
    bge    .L_f_copy_block
  .L_f_copy_post_block:
    @ copy 4 words, two at a time
    tst    r2, #0b10000
    ldmne  r1!, {r3, r12}
    stmne  r0!, {r3, r12}
    ldmne  r1!, {r3, r12}
    stmne  r0!, {r3, r12}
    bics   r2, r2, #0b10000
    bxeq   lr
    @ copy 2 and/or 1 words
    lsls   r3, r2, #29
    ldmcs  r1!, {r3, r12}
    stmcs  r0!, {r3, r12}
    ldrmi  r3, [r1], #4
    strmi  r3, [r0], #4
    bics   r2, r2, #0b1100
    bxeq   lr
    @ copy halfword and/or byte
    lsls   r3, r2, #31
    ldrhcs r3, [r1], #2
    strhcs r3, [r0], #2
    ldrbmi r3, [r1], #1
    strbmi r3, [r0], #1
    bx     lr

  .L_f_copy_block:
    push   {r4-r9}
  1:
    subs   r2, r2, #32
    ldmge  r1!, {r3-r9, r12}
    stmge  r0!, {r3-r9, r12}
    bgt    1b
    pop    {r4-r9}
    bxeq   lr
    b      .L_f_copy_post_block

  .L_f_copy_fixup4:
    cmp    r2, #7 @ if count <= (fix+word): just byte copy
    ble    .L_f_copy_max_coalign1
    lsls   r3, r0, #31
    submi  r2, r2, #1
    ldrbmi r3, [r1], #1
    strbmi r3, [r0], #1
    subcs  r2, r2, #2
    ldrhcs r3, [r1], #2
    strhcs r3, [r0], #2
    b      .L_f_copy_coalign4_assured

  .L_f_copy_max_coalign2:
    tst     r0, #1
    bne     .L_f_copy_fixup2
  .L_f_copy_coalign2_assured:
  1:
    subs    r2, r2, #2
    ldrhge  r3, [r1], #2
    strhge  r3, [r0], #2
    bgt     1b
    bxeq    lr
    tst     r2, #1
    ldrbne  r3, [r1], #1
    strbne  r3, [r0], #1
    bx      lr

  .L_f_copy_fixup2:
    cmp     r2, #3 @ if count <= (fix+halfword): just byte copy
    ble     .L_f_copy_max_coalign1
    sub     r2, r2, #1
    ldrb    r3, [r1], #1
    strb    r3, [r0], #1
    b       .L_f_copy_coalign2_assured

gba_memcpy_sram:
  .L_f_copy_max_coalign1:
  1:
    subs    r2, r2, #1
    ldrbge  r3, [r1], #1
    strbge  r3, [r0], #1
    bgt     1b
    bx      lr

  .L_r_copy_gain_align:
    add     r0, r0, r2
    add     r1, r1, r2
    eor     r3, r0, r1
    lsls    r3, r3, #31
    bmi     .L_r_copy_max_coalign1
    bcs     .L_r_copy_max_coalign2
    @ else fallthrough
  .L_r_copy_max_coalign4:
    tst     r0, #3
    bne     .L_r_copy_fixup4
  .L_r_copy_coalign4_assured:
    cmp     r2, #32
    bge     .L_r_copy_block
  .L_r_copy_post_block:
    @ copy 4 words, two at a time
    tst     r2, #0b10000
    ldmdbne r1!, {r3, r12}
    stmdbne r0!, {r3, r12}
    ldmdbne r1!, {r3, r12}
    stmdbne r0!, {r3, r12}
    bics    r2, r2, #0b10000
    bxeq    lr
    @ copy 2 and/or 1 words
    lsls    r3, r2, #29
    ldmdbcs r1!, {r3, r12}
    stmdbcs r0!, {r3, r12}
    ldrmi   r3, [r1, #-4]!
    strmi   r3, [r0, #-4]!
    bxeq    lr
    lsls    r2, r2, #31
    ldrhcs  r3, [r1, #-2]!
    strhcs  r3, [r0, #-2]!
    ldrbmi  r3, [r1, #-1]!
    strbmi  r3, [r0, #-1]!
    bx      lr

  .L_r_copy_block:
    push   {r4-r9}
  1:
    subs    r2, r2, #32
    ldmdbcs r1!, {r3-r9, r12}
    stmdbcs r0!, {r3-r9, r12}
    bgt     1b
    pop     {r4-r9}
    bxeq    lr
    b       .L_r_copy_post_block

  .L_r_copy_fixup4:
    cmp     r2, #7 @ if count <= (fix+word): just byte copy
    ble     .L_r_copy_max_coalign1
    lsls    r3, r0, #31
    submi   r2, r2, #1
    ldrbmi  r3, [r1, #-1]!
    strbmi  r3, [r0, #-1]!
    subcs   r2, r2, #2
    ldrhcs  r3, [r1, #-2]!
    strhcs  r3, [r0, #-2]!
    b       .L_r_copy_coalign4_assured

  .L_r_copy_max_coalign2:
    tst     r0, #1
    bne     .L_r_copy_fixup2
  .L_r_copy_coalign2_assured:
  1:
    subs    r2, r2, #2
    ldrhge  r3, [r1, #-2]!
    strhge  r3, [r0, #-2]!
    bgt     1b
    bxeq    lr
    tst     r2, #1
    ldrbne  r3, [r1, #-1]!
    strbne  r3, [r0, #-1]!
    bx      lr

  .L_r_copy_fixup2:
    cmp     r2, #3 @ if count <= (fix+halfword): just byte copy
    ble     .L_r_copy_max_coalign1
    sub     r2, r2, #1
    ldrb    r3, [r1, #-1]!
    strb    r3, [r0, #-1]!
    b       .L_r_copy_coalign2_assured

  .L_r_copy_max_coalign1:
  1:
    subs    r2, r2, #1
    ldrbge  r3, [r1, #-1]!
    strbge  r3, [r0, #-1]!
    bgt     1b
    bx      lr
.previous

.section ".text.libc.memset"
libc_memset:
    mov    r3, r2
    mov    r2, r1
    mov    r1, r3
    push   {r0, lr}
    bl     aeabi_memset
    pop    {r0, lr}
    bx     lr
.previous

.section ".text.aeabi.memclr.aligned"
aeabi_memclr8:
aeabi_memclr4:
    mov    r2, #0
    mov    r3, #0
    b      .L_memset_check_for_block_work
.previous

.section ".text.aeabi.memset"
aeabi_memclr:
    mov    r2, #0
aeabi_memset8:
aeabi_memset4:
aeabi_memset: @ r0(dest), r1(count), r2(byte)
    @ duplicate the byte across all of r2 and r3
    and    r2, r2, #0xFF
    orr    r2, r2, r2, lsl #8
    orr    r2, r2, r2, lsl #16
    mov    r3, r2
    @ for 'sets' too small to fixup we just byte loop
    cmp    r1, #3
    ble    .L_memset_byte_loop
    @ carry/sign test on the address, then do fixup
    lsls   r12, r0, #31
    submi  r1, r1, #1
    strbmi r2, [r0], #1
    subcs  r1, r1, #2
    strhcs r2, [r0], #2
  .L_memset_check_for_block_work:
    cmp    r1, #32
    bge    .L_memset_block_work
  .L_memset_post_block_work:
    @ set 4 words
    tst    r1, #0b10000
    stmne  r0!, {r2, r3}
    stmne  r0!, {r2, r3}
    @ set 2 and/or 1 words
    lsls   r12, r1, #29
    stmcs  r0!, {r2, r3}
    strmi  r2, [r0], #4
    @ set halfword and/or byte
    lsls   r12, r1, #31
    strhcs r2, [r0], #2
    strbmi r2, [r0], #1
    bx     lr

  .L_memset_block_work:
    push   {r4-r9}
    mov    r4, r2
    mov    r5, r2
    mov    r6, r2
    mov    r7, r2
    mov    r8, r2
    mov    r9, r2
  1:
    subs   r1, r1, #32
    stmge  r0!, {r2-r9}
    bgt    1b
    pop    {r4-r9}
    bxeq   lr
    b      .L_memset_post_block_work

  .L_memset_byte_loop:
  1:
    subs   r1, r1, #1
    strbcs r2, [r0], #1
    bgt    1b
    bx     lr
.previous

.section ".text.aeabi.unaligned.read8"
aeabi_uread8:
    ldrb r1, [r0, #4]
    ldrb r2, [r0, #5]
    orr  r1, r1, r2, lsl #8
    ldrb r2, [r0, #6]
    orr  r1, r1, r2, lsl #16
    ldrb r2, [r0, #7]
    orr  r1, r1, r2, lsl #24
    b    aeabi_uread4
.previous

.section ".text.aeabi.unaligned.read4"
aeabi_uread4:
    @ r1 may already hold output data!
    ldrb r2, [r0]
    ldrb r3, [r0, #1]
    orr  r2, r2, r3, lsl #8
    ldrb r3, [r0, #2]
    orr  r2, r2, r3, lsl #16
    ldrb r3, [r0, #3]
    orr  r2, r2, r3, lsl #24
    mov  r0, r2
    bx   lr
.previous

.section ".text.aeabi.unaligned.write8"
aeabi_uwrite8:
    strb r0, [r2]
    lsr  r3, r0, #8
    strb r3, [r2, #1]
    lsr  r3, r3, #8
    strb r3, [r2, #2]
    lsr  r3, r3, #8
    strb r3, [r2, #3]
    strb r1, [r2, #4]
    lsr  r3, r1, #8
    strb r3, [r2, #5]
    lsr  r3, r3, #8
    strb r3, [r2, #6]
    lsr  r3, r3, #8
    strb r3, [r2, #7]
    bx   lr
.previous

.section ".text.aeabi.unaligned.write4"
aeabi_uwrite4:
    strb r0, [r1]
    lsr  r2, r0, #8
    strb r2, [r1, #1]
    lsr  r2, r2, #8
    strb r2, [r1, #2]
    lsr  r2, r2, #8
    strb r2, [r1, #3]
    bx   lr
.previous

.section ".text.aeabi.idiv"
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
