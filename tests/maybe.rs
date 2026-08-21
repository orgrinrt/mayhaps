use mayhaps::maybe;

#[test]
fn plain_form_binds_the_value() {
    let opt = Some(7u8);
    let mut seen = 0u8;
    maybe!(v from (opt) exists { seen = v; });
    assert_eq!(seen, 7);
}

#[test]
fn plain_form_skips_when_absent() {
    let opt: Option<u8> = None;
    let mut seen = 0u8;
    maybe!(v from (opt) exists { seen = v; });
    assert_eq!(seen, 0);
}

#[test]
fn else_form_takes_the_present_branch() {
    let opt = Some(3u8);
    // Uninitialised on purpose: both arms assign, so this compiling at all is the
    // compiler proving the expansion has no path that assigns neither.
    let which;
    maybe!(v from (opt) exists { let _ = v; which = "present"; } else { which = "absent"; });
    assert_eq!(which, "present");
}

#[test]
fn else_form_takes_the_absent_branch() {
    let opt: Option<u8> = None;
    // Uninitialised on purpose: both arms assign, so this compiling at all is the
    // compiler proving the expansion has no path that assigns neither.
    let which;
    maybe!(v from (opt) exists { let _ = v; which = "present"; } else { which = "absent"; });
    assert_eq!(which, "absent");
}
