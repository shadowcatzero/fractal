struct View {
    ss_stretch: vec2<f32>,
    ss_pos: vec2<f32>,
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
    @location(0) tex_pos: vec2<f32>,
    @location(1) ss_pos: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32,
    @builtin(instance_index) ii: u32,
) -> VertexOutput {
    var out: VertexOutput;

    let tpos = vec2<f32>(
        f32(vi % 2u),
        f32(1 - vi / 2u),
    );
    let vpos = tpos * 2.0 - 1.0;
    out.vertex_pos = vec4<f32>(vpos, 0.0, 1.0);

    out.tex_pos = tpos;
    out.ss_pos = vpos * view.ss_stretch + view.ss_pos;
    out.ss_pos = (out.ss_pos + 1.0) / 2.0;

    return out;
}

@fragment
fn fs_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let cur = textureSample(tex, sam, in.tex_pos);
    let snp_bounds = all(in.ss_pos >= vec2(0.0)) && all(in.ss_pos <= vec2(1.0));
    if all(cur.rgb == vec3(0.0)) && snp_bounds {
        let snp = textureSample(ss_t, ss_s, in.ss_pos).rgb;
        return vec4(snp * 0.3, 1.0);
    } else {
        return cur;
    }
}

