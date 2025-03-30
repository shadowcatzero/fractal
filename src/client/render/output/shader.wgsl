struct View {
    stretch: vec2<f32>,
    pos: vec2<f32>,
    rendered_chunks: vec2<u32>,
    snapshot: u32,
}

@group(0) @binding(0)
var<storage> view: View;
@group(0) @binding(1)
var chunks: texture_2d_array<u32>;

@group(0) @binding(2)
var tex: texture_2d<f32>;
@group(0) @binding(3)
var sam: sampler;
@group(0) @binding(4)
var ss_t: texture_2d<f32>;
@group(0) @binding(5)
var ss_s: sampler;

struct VertexOutput {
    @builtin(position) vertex_pos: vec4<f32>,
    @location(0) world_pos: vec2<f32>,
    @location(1) tex_pos: vec2<f32>,
    @location(2) ss_pos: vec2<f32>,
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
    out.world_pos = pos / 2.0;
    out.world_pos.y *= -1.0;
    out.world_pos *= view.stretch;
    out.world_pos += view.pos;

    let pos2 = vec2<f32>(
        f32(vi % 2u),
        f32(vi / 2u),
    );
    out.tex_pos = pos2;
    out.tex_pos.y = 1.0 - out.tex_pos.y;
    let pos3 = vec2(pos.x, -pos.y);
    out.ss_pos = pos3 * view.stretch + view.pos;
    out.ss_pos = (out.ss_pos + 1.0) / 2.0;

    return out;
}

@fragment
fn fs_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    // let a = textureLoad(chunks, vec2<u32>(0), 0, 0);
    // let rc = vec2<i32>(view.rendered_chunks);
    // let rcf = vec2<f32>(rc);
    // let cposi = vec2<i32>(floor(in.world_pos));
    // let cposu = vec2<i32>(
    //     rem_euclid(cposi.x, rc.x),
    //     rem_euclid(cposi.y, rc.y)
    // );
    // let cposf = vec2<f32>(cposu);
    // return vec4(cposf / rcf, 0.0, 1.0);
    let cur = textureSample(tex, sam, in.tex_pos);
    let snp_bounds = all(in.ss_pos >= vec2(0.0)) && all(in.ss_pos <= vec2(1.0));
    if all(cur.rgb == vec3(0.0)) && snp_bounds {
        let snp = textureSample(ss_t, ss_s, in.ss_pos).rgb;
        return vec4(snp * 0.3, 1.0);
    } else {
        return cur;
    }
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
