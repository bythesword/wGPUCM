//Vertex shader

struct Camera {
    view_pos : vec4 < f32>,
    view_proj : mat4x4 < f32>,
}
@group(0) @binding(0) var<uniform> camera : Camera;

struct VertexInput {
    @location(0) position : vec3 < f32>,
    //@location(1) color : vec4 < f32>,
};

struct VertexOutput {
    @builtin(position) clip_position : vec4 < f32>,
   // @location(0) color : vec4 < f32>,
};

@vertex
fn vs_main(
model : VertexInput,
) -> VertexOutput {
    //let c=camera;
    var out : VertexOutput;
    //out.color = model.color;
    out.clip_position = camera.view_proj * vec4 < f32 > (model.position, 1.0);
    return out;
}

//Fragment shader

@fragment
fn fs_main(in : VertexOutput) -> @location(0) vec4 < f32> {
    //return vec4<f32>(in.color.rgb, 0.10);
    //return in.color;
    return vec4f(1.0);
}
