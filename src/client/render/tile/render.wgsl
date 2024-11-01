const LEN: u32 = 3;
const ILEN: i32 = 3;
const LEN2: u32 = LEN * 2;

struct View {
    stretch: vec2<f32>,
    level: i32,
    scale: FixedDec,
    x: FixedDec,
    y: FixedDec,
}

struct FixedDec {
    sign: u32,
    dec: i32,
    parts: array<u32, LEN>,
}

@group(0) @binding(0)
var<storage> view: View;

struct VertexOutput {
    @builtin(position) vertex_pos: vec4<f32>,
    @location(0) world_pos: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32,
    @builtin(instance_index) ii: u32,
) -> VertexOutput {
    var out: VertexOutput;

    let pos = vec2<f32>(
        f32(vi % 2u),
        f32(vi / 2u),
    ) * 2.0 - 1.0;
    out.vertex_pos = vec4<f32>(pos.x, -pos.y, 0.0, 1.0);
    out.world_pos = pos;
    out.world_pos.y *= -1.0;
    out.world_pos *= view.stretch;
    return out;
}

@fragment
fn fs_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let cx = add(mul(from_f32(in.world_pos.x), view.scale), view.x);
    let cy = add(mul(from_f32(in.world_pos.y), view.scale), view.y);
    var x = zero();
    var y = zero();
    let two = from_f32(2.0);
    let thresh = from_f32(2.0 * 2.0);
    var i = 0u;
    let max = 50u + (1u << u32(view.level));
    loop {
        let x2 = mul(x, x);
        let y2 = mul(y, y);
        if gt(add(x2, y2), thresh) || i >= max {
            break;
        }
        y = add(mul(two, mul(x, y)), cy);
        x = add(sub(x2, y2), cx);
        i += 1u;
    }
    var color = vec3<f32>(0.0, 0.0, 0.0);
    if i != max {
        let pi = 3.1415;
        let hue = f32(i) / 30.0;
        color.r = cos(hue);
        color.g = cos(hue - 2.0 * pi / 3.0);
        color.b = cos(hue - 4.0 * pi / 3.0);
    }
    return vec4(color, 1.0);
}

fn add(lhs: FixedDec, rhs: FixedDec) -> FixedDec {
    var dest = FixedDec();
    dest.sign = lhs.sign;
    dest.dec = max(lhs.dec, rhs.dec);
    var carry = false;
    let rhs_offset = rhs.dec - dest.dec;
    let lhs_offset = lhs.dec - dest.dec;
    var i = ILEN;
    if lhs.sign == rhs.sign {
        while i > 0 {
            i -= 1;
            let a = at(lhs, i + lhs_offset);
            let b = at(rhs, i + rhs_offset);
            let res = a + b + u32(carry);
            dest.parts[i] = res;
            carry = res < a;
        }
        if carry {
            var i = ILEN - 1;
            while i > 0 {
                i -= 1;
                dest.parts[i + 1] = dest.parts[i];
            }
            dest.parts[0] = 1u;
            dest.dec += 1;
        }
    } else {
        while i > 0 {
            i -= 1;
            let a = at(lhs, i + lhs_offset);
            let b = at(rhs, i + rhs_offset);
            let res = a - b - u32(carry);
            dest.parts[i] = res;
            carry = a < res;
        }
        if carry {
            dest.sign = u32(dest.sign == 0);
            var i = 0;
            while i < ILEN {
                dest.parts[i] = ~dest.parts[i];
                i += 1;
            }
        }
    }
    return dest;
}

fn sub(lhs: FixedDec, rhs: FixedDec) -> FixedDec {
    var r = rhs;
    r.sign = u32(r.sign == 0);
    return add(lhs, r);
}

fn at(dec: FixedDec, i: i32) -> u32 {
    if i < 0 || i >= ILEN {
        return 0u;
    }
    var parts = dec.parts;
    return parts[i];
}

const POS: u32 = 0u;
const NEG: u32 = 1u;

fn mul(lhs: FixedDec, rhs: FixedDec) -> FixedDec {
    let sign = u32(lhs.sign != rhs.sign);
    var parts = array<u32, LEN2>();
    var dec = lhs.dec + rhs.dec;
    var lparts = lhs.parts;
    var rparts = rhs.parts;

    var i = LEN;
    while i > 0 {
        i -= 1u;
        let x = lparts[i];
        var carry: u32 = 0;
        var j = LEN;
        while j > 0 {
            j -= 1u;
            let y = rparts[j];

            // widening mul
            let lsb = x * y;
            let a = x & 0xffff;
            let b = x >> 16;
            let c = y & 0xffff;
            let d = y >> 16;
            let ad = a * d + ((a * c) >> 16);
            let bc = b * c;
            let car = ad > (0xffffffff - bc);
            let msb = ((ad + bc) >> 16) + (u32(car) << 16) + b * d;

            let k = i + j + 1;
            let res = parts[k] + lsb;
            let carry1 = res < lsb;
            let res2 = res + carry;
            let carry2 = res2 < res;
            parts[k] = res2;
            carry = u32(carry1 || carry2) + msb;
        }
        parts[i] = carry;
    }

    var new_parts = array<u32, LEN>();
    i = 0u;
    while i < LEN2 && parts[i] == 0 {
        dec -= 1;
        i += 1u;
    }
    var j = 0u;
    while j < LEN && (i + j) < LEN2 {
        new_parts[j] = parts[i + j];
        j += 1u;
    }
    return FixedDec(sign, dec, new_parts);
}

fn gt(x: FixedDec, y: FixedDec) -> bool {
    if x.dec > y.dec {
        return true;
    }
    if y.dec > x.dec {
        return false;
    }
    return x.parts[0] > y.parts[0];
}

fn to_f32(value: FixedDec) -> f32 {
    var parts = value.parts;

    var sign = value.sign * (1u << 31);
    var skip_count = 0;

    while skip_count < ILEN && parts[skip_count] == 0 {
        skip_count += 1;
    }

    if skip_count == ILEN {
        if value.sign == POS {
            return 0.0;
        } else {
            return -0.0;
        }
    }
    let v = parts[skip_count];
    var start = countLeadingZeros(v) + 1;
    let exp_i = (value.dec - skip_count) * 32 - i32(start);
    var frac_sh = 0u;
    var exp = 0u;
    if exp_i >= -127 {
        if exp_i == -127 {
            start -= 1u;
        }
        exp = u32(exp_i + 127);
    } else {
        frac_sh = u32(-(exp_i + 32 * 4 - 1));
        if frac_sh < 23 {
            start -= 1u;
        } else {
            return 0.0;
        }
    };
    var frac: u32;
    if start > 9 {
        let sh = start - 9;
        let next_i = skip_count + 1;
        var v2 = 0u;
        if next_i < ILEN {
            v2 = parts[next_i] >> (32 - sh);
        }
        frac = (v << sh) + v2;
    } else {
        frac = v >> (9 - start);
    };
    frac &= ~(1u << 23);
    let res = (frac >> frac_sh) + (exp << 23) + sign;
    return bitcast<f32>(res);
}

const INV_SIGN_MASK: u32 = (1u << 31) - 1;
const FRAC_BIT: u32 = 1u << 23;
const FRAC_MASK: u32 = FRAC_BIT - 1;

fn from_f32(value: f32) -> FixedDec {
    let raw = bitcast<u32>(value) & INV_SIGN_MASK;
    var exp = i32(raw >> 23) - 127;
    var frac = raw & FRAC_MASK;
    var start = -exp;
    if exp == -127 {
        exp = -126;
        start = -exp;
    } else {
        frac += FRAC_BIT;
        start -= 1;
    }
    let end = -exp + 23;
    let start_i = div_euclid(start, 32);
    let end_i = div_euclid(end - 1, 32);
    var parts = array<u32, LEN>();
    var dec = -start_i;
    if start_i == end_i {
        let val = frac << u32(8 - rem_euclid(start, 32));
        if val != 0 {
            parts[0] = val;
        }
    } else {
        let s = u32(rem_euclid(end, 32));
        let val_high = frac >> s;
        let val_low = frac << (32 - s);
        var i = 0;
        if val_high != 0 {
            parts[0] = val_high;
            i += 1;
        } else {
            dec -= 1;
        }
        if val_low != 0 {
            parts[i] = val_low;
        }
    }
    if parts[0] == 0 && parts[1] == 0 {
        dec = 0;
    }
    return FixedDec(
        u32(value < 0.0),
        dec,
        parts,
    );
}

fn zero() -> FixedDec {
    return FixedDec(0, 0, array<u32, LEN>());
}

fn div_euclid(x: i32, y: i32) -> i32 {
    if x < 0 {
        return -((-x - 1) / y) - 1;
    }
    return x / y;
}

fn rem_euclid(x: i32, y: i32) -> i32 {
    return x - div_euclid(x, y) * y;
}
