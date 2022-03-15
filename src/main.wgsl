
struct Stuff {
    render_width: u32;
    render_height: u32;
    display_width: u32;
    display_height: u32;
    windowless: u32;

    time: f32;
    cursor_x: f32;
    cursor_y: f32;

    scroll: f32;
    mouse_left: u32;
    mouse_right: u32;
    mouse_middle: u32;
};
[[group(0), binding(0)]]
var<uniform> stuff: Stuff;

let PI = 3.14159265359;
let PHI = 1.61803398874989484820459;
type v2f = vec2<f32>;
type v3f = vec3<f32>;
type v4f = vec4<f32>;


struct Particle {
    p: v3f;
    v: v3f;
};
struct ParticleBuffer {
    buff: array<Particle>;
};
[[group(0), binding(1)]]
var<storage, read_write> particle_buffer: ParticleBuffer;


struct Buf {
    buff: array<u32>;
};
[[group(0), binding(2)]]
var<storage, read_write> screen_buffer: Buf;


[[group(0), binding(3)]]
var screen_texture: texture_storage_2d<rgba32float, read_write>;

/// import src/vertex.wgsl
/// import src/compute.wgsl
