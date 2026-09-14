// assets/shaders/tile_dynamic.wgsl
// Custom tile shader reacting to time of day, moisture, and environmental conditions

struct ViewUniform {
    view_proj: mat4x4<f32>,
    world_position: vec3<f32>,
};

struct TileEnvironment {
    time_of_day_normalized: f32, // 0.0 to 1.0 (dawn -> noon -> dusk -> midnight)
    ambient_light: vec3<f32>,
    moisture_level: f32,
    elevation: f32,
};

@group(0) @binding(0)
var<uniform> view: ViewUniform;

@group(1) @binding(0)
var<uniform> env: TileEnvironment;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) world_pos: vec3<f32>,
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = view.view_proj * vec4<f32>(in.position, 1.0);
    out.uv = in.uv;
    out.color = in.color;
    out.world_pos = in.position;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Dynamic daylight cycle color modulation
    var daylight_tint = vec3<f32>(1.0, 0.95, 0.85); // Noon warmth
    if (env.time_of_day_normalized < 0.25 || env.time_of_day_normalized > 0.75) {
        // Night / Dawn coolness
        daylight_tint = vec3<f32>(0.2, 0.25, 0.45);
    } else if (env.time_of_day_normalized > 0.65) {
        // Sunset golden hour
        daylight_tint = vec3<f32>(0.95, 0.6, 0.35);
    }

    // Moisture darkening effect
    let moisture_mod = 1.0 - (env.moisture_level * 0.2);

    let final_rgb = in.color.rgb * env.ambient_light * daylight_tint * moisture_mod;
    return vec4<f32>(final_rgb, in.color.a);
}
