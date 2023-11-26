struct VertexInput {
    @builtin(vertex_index)
    in_vertex_index: u32,
    @location(0)
    pos: vec3f,
    @location(1)
    color: vec4f,
}

struct VertexOutput {
    @builtin(position)
    pos: vec4f,
    @location(0)
    color: vec4f,
}

@binding(0)
@group(0)
var<uniform> transform: mat4x4f;

@vertex
fn vs_main(
    input: VertexInput
) -> VertexOutput {
    var output: VertexOutput;
    output.pos = transform * vec4f(input.pos, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(
    @location(0)
    color: vec4f
) -> @location(0) vec4f {
    return color;
}
