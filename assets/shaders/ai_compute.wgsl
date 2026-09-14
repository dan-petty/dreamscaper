// assets/shaders/ai_compute.wgsl
// GPU Compute Shader for semi-autonomous AI unit steering and pathfinding

struct Agent {
    position: vec2<f32>,
    velocity: vec2<f32>,
    target: vec2<f32>,
    state: u32,
    morale: f32,
    _pad: vec2<f32>,
};

struct SimParams {
    delta_time: f32,
    max_speed: f32,
    avoidance_radius: f32,
    agent_count: u32,
};

@group(0) @binding(0)
var<storage, read> agents_in: array<Agent>;

@group(0) @binding(1)
var<storage, read_write> agents_out: array<Agent>;

@group(0) @binding(2)
var<uniform> params: SimParams;

@compute @workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= params.agent_count) {
        return;
    }

    var agent = agents_in[index];

    // Calculate steering towards target
    let to_target = agent.target - agent.position;
    let dist = length(to_target);
    var desired_velocity = vec2<f32>(0.0, 0.0);

    if (dist > 0.001) {
        desired_velocity = normalize(to_target) * params.max_speed;
    }

    // Separation & obstacle avoidance steering
    var separation = vec2<f32>(0.0, 0.0);
    for (var i = 0u; i < params.agent_count; i = i + 1u) {
        if (i == index) {
            continue;
        }
        let other = agents_in[i];
        let diff = agent.position - other.position;
        let d = length(diff);
        if (d > 0.0001 && d < params.avoidance_radius) {
            separation = separation + (normalize(diff) / d);
        }
    }

    // Blend steering vectors
    let steer = (desired_velocity - agent.velocity) * 0.5 + separation * 1.5;
    agent.velocity = agent.velocity + steer * params.delta_time;

    // Clamp to max speed
    let current_speed = length(agent.velocity);
    if (current_speed > params.max_speed) {
        agent.velocity = normalize(agent.velocity) * params.max_speed;
    }

    // Integrate position
    agent.position = agent.position + agent.velocity * params.delta_time;

    // Dynamic state transition based on conditions
    if (dist < 0.5) {
        agent.state = 1u; // Arrived / Idle
    } else {
        agent.state = 2u; // Moving / Autonomous Nav
    }

    agents_out[index] = agent;
}
