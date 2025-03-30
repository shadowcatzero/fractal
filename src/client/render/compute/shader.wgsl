override CHUNK_POW: u32 = 10;
const LEN: u32 = REPLACE_LENu;
const ILEN: i32 = i32(LEN);
const LEN2: u32 = LEN * 2;
override WGX: u32 = 8;
override WGY: u32 = 8;

struct View {
    reset: u32,
    level: i32,
    dims: vec2<u32>,
    scale: FixedDec,
    corner_x: FixedDec,
    corner_y: FixedDec,
}

@group(0) @binding(0)
var<storage> view: View;
@group(0) @binding(1)
var<storage, read_write> work: array<u32>;
@group(0) @binding(2)
var output: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(WGX, WGY, 1)
fn main(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    if id.x > view.dims.x - 1 || id.y > view.dims.y - 1 {
        return;
    }
    // TODO: actually use width
    let varwidth = LEN + 2;
    let workwidth = varwidth * 2 + 1;
    let worki = (id.x * view.dims.y + id.y) * workwidth;
    let xidx = worki + 1;
    let yidx = xidx + varwidth;

    // let dec = view.corner_x.dec;
    // var rel_x = FixedDec(POS, dec, array<u32, LEN>());
    // rel_x.parts[0] = id.x;
    // rel_x = shr(rel_x, view.level);
    // var rel_y = FixedDec(POS, dec, array<u32, LEN>());
    // rel_y.parts[0] = id.y;
    // rel_y = shr(rel_y, view.level);
    // let cx = add(view.corner_x, rel_x);
    // let cy = add(view.corner_y, rel_y);
    let fdims = vec2<f32>(view.dims);
    var stretch: vec2<f32>;
    if fdims.x < fdims.y {
        stretch = vec2(fdims.x / fdims.y, 1.0);
    } else {
        stretch = vec2(1.0, fdims.y / fdims.x);
    }
    let fpos = (vec2<f32>(id.xy) / fdims - 0.5) * stretch;
    let cx = add(mul(from_f32(fpos.x), view.scale), view.corner_x);
    let cy = add(mul(from_f32(fpos.y), view.scale), view.corner_y);
    var x: FixedDec;
    var y: FixedDec;
    var i = work[worki];
    if bool(view.reset) {
        x = zero();
        y = zero();
        i = 0;
    } else {
        x = FixedDec(work[xidx + 0], bitcast<i32>(work[xidx + 1]), array<u32, LEN>());
        y = FixedDec(work[yidx + 0], bitcast<i32>(work[yidx + 1]), array<u32, LEN>());
        for (var j = 0u; j < LEN; j += 1u) {
            x.parts[j] = work[xidx + 2 + j];
            y.parts[j] = work[yidx + 2 + j];
        }
    }
    let max = i + 1;
    let thresh = from_f32(2.0 * 2.0);
    loop {
        let x2 = mul(x, x);
        let y2 = mul(y, y);
        if gt(add(x2, y2), thresh) || i >= max {
            break;
        }
        let xy = mul(x, y);
        y = add(add(xy, xy), cy);
        x = add(sub(x2, y2), cx);
        i += 1u;
    }
    work[worki] = i;
    work[xidx + 0] = x.sign; work[xidx + 1] = bitcast<u32>(x.dec);
    work[yidx + 0] = y.sign; work[yidx + 1] = bitcast<u32>(y.dec);
    for (var j = 0u; j < LEN; j += 1u) {
        work[xidx + 2 + j] = x.parts[j];
        work[yidx + 2 + j] = y.parts[j];
    }
    var color = vec3<f32>(0.0, 0.0, 0.0);
    if i != max {
        let pi = 3.1415;
        let hue = f32(i) / 30.0;
        color.r = cos(hue);
        color.g = cos(hue - 2.0 * pi / 3.0);
        color.b = cos(hue - 4.0 * pi / 3.0);
    }
    textureStore(output, id.xy, vec4(color, 1.0));
}

