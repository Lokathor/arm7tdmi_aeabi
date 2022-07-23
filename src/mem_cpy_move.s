
.global libc_memcpy
.global aeabi_memcpy
.global aeabi_memcpy4
.global aeabi_memcpy8
.global gba_memcpy_sram

.global libc_memmove
.global aeabi_memmove
.global aeabi_memmove4
.global aeabi_memmove8

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

.section ".text.aeabi.copy.forward"
aeabi_memmove8:
aeabi_memmove4:
aeabi_memmove:
    cmp    r0, r1 @ if d > s, reverse copy
    bgt    .L_r_copy_gain_align
    @ else fallthrough

@@@ Forward @@@

aeabi_memcpy:
  .L_f_copy_gain_align:
    eor    r3, r0, r1
    lsls   r3, r3, #31
    bmi    .L_f_copy_max_coalign1
    bcs    .L_f_copy_max_coalign2
    @ else fallthrough

aeabi_memcpy8:
aeabi_memcpy4:
  .L_f_copy_max_coalign4:
    tst    r0, #3
    bne    .L_f_copy_fixup4
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
.previous

.section ".text.aeabi.copy.reverse"
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