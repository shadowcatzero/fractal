// Vertex shader

struct VertexOutput {
    @builtin(position) vertex_pos: vec4<f32>,
    @location(0) world_pos: vec2<f32>,
};

struct View {
    scale: vec2<f32>,
    // x_dec: i32,
    // y_dec: i32,
    // prec: u32,
}

@group(0) @binding(0)
var<uniform> view: View;
// @group(0) @binding(1)
// var<storage> vx: array<u32>;
// @group(0) @binding(2)
// var<storage> vy: array<u32>;

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
    out.world_pos *= view.scale;
    return out;
}

// const PREC = 2;

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

// @fragment
// fn fs_main(
//     in: VertexOutput,
// ) -> @location(0) vec4<f32> {
//     let dec = i32(1) << 13;
//     let c = vec2<i32>(in.world_pos * f32(dec));
//     let cx = c.x;
//     let cy = c.y;
//     var x = 0;
//     var y = 0;
//     var i = 0u;
//     let thresh = 2 * dec;
//     let thresh2 = thresh * thresh;
//     let max = 50u + u32(in.zoom);
//     loop {
//         let x2 = x * x;
//         let y2 = y * y;
//         if x2 + y2 > thresh2 || i >= max {
//             break;
//         }
//         y = (2 * x * y) / dec + c.y;
//         x = (x2 - y2) / dec + c.x;
//         i += 1u;
//     }
//     var color = vec3<f32>(0.0, 0.0, 0.0);
//     if i != max {
//         let pi = 3.1415;
//         let hue = f32(i) / 30.0;
//         color.r = cos(hue);
//         color.g = cos(hue - 2.0 * pi / 3.0);
//         color.b = cos(hue - 4.0 * pi / 3.0);
//     }
//     return vec4(color, 1.0);
// }
