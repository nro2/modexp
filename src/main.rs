use std::str::FromStr;

fn main() {
    let mut input = Vec::new();

    for arg in std::env::args().skip(1) {
        input.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if input.len() != 3 {
        error();
    }

    let d = modexp(input[0], input[1], input[2]);

    println!("{}", d);
}

fn modexp(x: u64, y: u64, m: u64) -> u64 {
    assert!(
        x < u64::from(u32::max_value())
            && y < u64::from(u32::max_value())
            && m < u64::from(u32::max_value()),
        "One of the values is larger than u32 max value!"
    );

    if x == 0 {
        return 0;
    }
    if y == 0 {
        return 1;
    }
    let mut z = modexp(x, y / 2, m);
    z = (z * z) % m;

    if y % 2 == 1 {
        z = (z * x) % m;
    }
    z
}

fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");
    std::process::exit(1);
}

#[test]
fn test_modexp() {
    assert_eq!(modexp(2, 20, 17), 16);
}

#[test]
fn test_y_zero() {
    assert_eq!(modexp(2, 0, 17), 1)
}

#[test]
fn test_x_zero() {
    assert_eq!(modexp(0, 20, 17), 0)
}

#[test]
#[should_panic]
fn mod_by_zero() {
    modexp(22, 20, 0);
}

#[test]
#[should_panic]
fn large_input() {
    modexp(99999999999, 20, 20);
}
