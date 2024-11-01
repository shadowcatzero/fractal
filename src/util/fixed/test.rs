use super::*;

const EPSILON: f32 = 0.000001;

macro_rules! assert_eq_f32 {
    ($left:expr, $right:expr, $dec:expr $(,)?) => {
        assert_eq_bits!($left, $left.to_bits(), $right, $right.to_bits(), $dec)
    };
    ($left:expr, $right:expr, $dec:expr, $arg:tt, $($args:tt)+) => {
        assert_eq_bits!($left, $left.to_bits(), $right, $right.to_bits(), $dec, $arg, $($args)+)
    };
}

macro_rules! assert_eq_bits {
    ($left:expr, $left_bits:expr, $right:expr, $right_bits:expr, $dec:expr $(,)?) => {
        assert!(
            ($left - $right).abs() < EPSILON,
            "\n  left: {:032b} = {:?}\n right: {:032b} = {:?}\n  from: {:?}",
            $left_bits,
            $left,
            $right_bits,
            $right,
            $dec,
        )
    };
    ($left:expr, $left_bits:expr, $right:expr, $right_bits:expr, $dec:expr, $arg:tt, $($args:tt)+) => {
        assert!(
            ($left - $right).abs() < EPSILON,
            concat!("\n  expr: ", $arg, "\n  left: {:032b} = {:?}\n right: {:032b} = {:?}\n  from: {:?}"),
            $($args)+,
            $left_bits,
            $left,
            $right_bits,
            $right,
            $dec,
        )
    };
}

#[test]
fn conversion() {
    fn test(x: f32) {
        let dec = FixedDec::from(x);
        assert_eq_f32!(x, f32::from(&dec), dec)
    }
    test(0.0);
    test(-0.0);
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
    fn test(x: f32, y: f32) {
        let a = x + y;
        let dec = FixedDec::from(x) + FixedDec::from(y);
        assert_eq_f32!(a, f32::from(&dec), dec, "{} + {}", x, y);
    }
    test(0.25, 0.75);
    test(1.25, 0.125);
    test(1.25, -0.125);
    test(100.25, -0.125);
    test(-1.25, 0.125);
    test(-100.25, -0.125);
    test(100.25, -0.125);
    test(0.25, -0.00000000125);
    test(0.25, -0.0000125);
    test(100000000000000.0, -100000000000.0);
    test(0.0000310, -0.0042042);
    test(-0.0000310, 0.0002042);
    let x = -0.0016598016;
    let y = 0.0028538946;
    let mut a = FixedDec::from(x);
    a.set_whole_len(1);
    let b = FixedDec::from(y);
    let res = a + b;
    assert_eq_f32!(x + y, f32::from(&res), res, "{} + {}", x, y);
}

#[test]
fn mul() {
    fn test(x: f32, y: f32) {
        let a = x * y;
        let dec = FixedDec::from(x) * FixedDec::from(y);
        assert_eq_f32!(a, f32::from(&dec), dec, "{:?} * {:?}", x, y);
    }
    test(0.0, 0.0);
    test(1.0, 1.0);
    test(1.0, 0.0);
    test(2.0, 1.0);
    test(2.0, 0.5);
    test(20.0, 0.245);
    test(0.03819, 0.0183488);
    test(30492.39, 9130.391);

    test(2.0, -1.0);
    test(0.0, -1.0);
    test(-1.0, 0.0);
    test(1.0, -1.0);
    test(2.0, -1.20904);
    test(-249.0, -1.20904);
    test(-30492.39, 9130.391);
    test(-249.0, 0.000031421);
}

#[test]
fn shr() {
    fn test(x: i32, y: i32) {
        let a = (x as f32) / 2f32.powi(y);
        let dec = FixedDec::from(x) >> y;
        assert_eq_f32!(a, f32::from(&dec), dec, "{:?} * {:?}", x, y);
    }
    test(1, 3);
    test(1, -3);
    test(1, 33);
    test(1, -33);
}
