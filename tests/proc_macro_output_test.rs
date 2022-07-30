
// We only manually turn this on during hand testing.
//
// Because the test needs to replace builtin symbols,
// it doesn't work during the standard cross-rs based
// testing that the normal assembly code goes though.

#[cfg(FALSE)]
arm7tdmi_aeabi::generate_fns!(section_prefix = ".iwram");
