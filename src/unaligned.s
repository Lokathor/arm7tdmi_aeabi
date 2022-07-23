
.global aeabi_uread4
.global aeabi_uwrite4
.global aeabi_uread8
.global aeabi_uwrite8

.section ".text.aeabi.uread"
aeabi_uread8:
    ldrb r1, [r0, #4]
    ldrb r2, [r0, #5]
    orr  r1, r1, r2, lsl #8
    ldrb r2, [r0, #6]
    orr  r1, r1, r2, lsl #16
    ldrb r2, [r0, #7]
    orr  r1, r1, r2, lsl #24
    @ fallthrough
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
