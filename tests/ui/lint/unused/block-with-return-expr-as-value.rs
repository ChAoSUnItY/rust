//@ check-pass
#[allow(unreachable_code)]
#[warn(unused_braces)]

fn main() {
    return { return };
    if return { return } { return } else { return }
    match return { return } {
        _ => { return }
    }
    return { return () };
    if return { return () } { return () } else { return () }
    match return { return () } {
        _ => { return () }
    }
}
