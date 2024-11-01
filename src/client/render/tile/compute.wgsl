
@fragment
fn fs_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let dec = i32(1) << 13;
    let c = vec2<i32>(in.world_pos * f32(dec));
    let cx = c.x;
    let cy = c.y;
    var x = 0;
    var y = 0;
    var i = 0u;
    let thresh = 2 * dec;
    let thresh2 = thresh * thresh;
    let max = 50u;
    loop {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > thresh2 || i >= max {
            break;
        }
        y = (2 * x * y) / dec + c.y;
        x = (x2 - y2) / dec + c.x;
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
