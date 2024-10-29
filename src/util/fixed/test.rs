use super::*;

macro_rules! assert_bits_eq {
    ($left:expr, $right:expr, $dec:expr $(,)?) => {
        assert!(
            $left == $right,
            "\n  left: {:032b} = {:?}\n right: {:032b} = {:?}\n  from: {:?}",
            $left.to_bits(),
            $left,
            $right.to_bits(),
            $right,
            $dec,
        )
    };
    ($left:expr, $right:expr, $dec:expr, $arg:tt, $($args:tt)+) => {
        assert!(
            $left == $right,
            concat!("\n  expr: ", $arg, "\n  left: {:032b} = {:?}\n right: {:032b} = {:?}\n  from: {:?}"),
            $($args)+,
            $left.to_bits(),
            $left,
            $right.to_bits(),
            $right,
            $dec,
        )
    }
}

#[test]
fn conversion() {
    fn test(x: f32) {
        let dec = FixedDec::from(x);
        assert_bits_eq!(x, f32::from(&dec), dec)
    }
    test(f32::from_bits(0b00000000_00000000_00000000_00000001));
    test(f32::from_bits(0b00000000_01000000_00000000_00000001));
    test(f32::from_bits(0b00010000_01000000_00000000_00000001));
    test(f32::from_bits(0b10000000_00000000_00000000_00000001));
    test(f32::from_bits(0b10000100_00010000_00010000_00100001));
    test(f32::from_bits(0b10000100_00000000_00000000_00000000));
    test(f32::from_bits(0b00111111_11111111_11111111_11111111));
    test(f32::from_bits(0b10111111_11111111_11111111_11111111));
    test(0.75 + 0.125);
    test(1.75);
    test(1.0 / 16.0);
    test(3.75);
    test(-3.75);
    test(1000000000.75);
    test(-1000000000.75);
    assert!(FixedDec::from(0.0).is_zero());
    test(-1.0);
}

#[test]
fn add_sub() {
    fn test_add(x: f32, y: f32) {
        let a = x + y;
        let dec = FixedDec::from(x) + FixedDec::from(y);
        assert_bits_eq!(a, f32::from(&dec), dec, "{} + {}", x, y);
    }
    test_add(0.25, 0.75);
    test_add(1.25, 0.125);
    test_add(1.25, -0.125);
    test_add(100.25, -0.125);
    test_add(-1.25, 0.125);
    test_add(-100.25, -0.125);
    test_add(100.25, -0.125);
    // test_add(0.25, -0.00000000125);
    test_add(0.25, -0.0000125);
    test_add(100000000000000.0, -100000000000.0);
}

#[test]
fn mul() {
    fn test_mul(x: f32, y: f32) {
        let a = x * y;
        let dec = FixedDec::from(x) * FixedDec::from(y);
        assert_bits_eq!(a, f32::from(&dec), dec, "{:?} * {:?}", x, y);
    }
    test_mul(0.0, 0.0);
    test_mul(1.0, 1.0);
    test_mul(1.0, 0.0);
    test_mul(2.0, 1.0);
    test_mul(2.0, 0.5);
    test_mul(20.0, 0.245);
    test_mul(0.03819, 0.0183488);
    test_mul(30492.39, 9130.391);

    test_mul(2.0, -1.0);
    test_mul(0.0, -1.0);
    test_mul(-1.0, 0.0);
    test_mul(1.0, -1.0);
    test_mul(2.0, -1.20904);
    test_mul(-249.0, -1.20904);
    test_mul(-30492.39, 9130.391);
}
