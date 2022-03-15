
// 15625 for a million
/// compute_enable
/// work_group_count 15625
[[stage(compute), workgroup_size(64)]] // workgroup_size can take 3 arguments -> x*y*z executions (default x, 1, 1) // minimum opengl requirements are (1024, 1024, 64) but (x*y*z < 1024 (not too sure)) no info about wgsl rn
fn main_compute([[builtin(global_invocation_id)]] g_id: vec3<u32>) { // global_invocation_id = local_invocation_id*work_group_id
    // let num_particles = 1000u;
    // let num_particles = 10000u;
    let num_particles = 100000u;
    // let num_particles = 400000u;
    // let num_particles = 1000000u;
    let id = g_id.x;
    if (id >= num_particles) {
        return;
    }

    let look_offset = v3f(1.0, 1.0, 0.0);

    // prepare cursor
    let scale = 2.0/f32(stuff.display_height);
    var curs = v2f(stuff.cursor_x, stuff.cursor_y)*scale;
    curs = curs - 1.0;

    // load particle
    var particle = particle_buffer.buff[id];
    var ppos = particle.p;
    var pvel = particle.v;

    // apply attractor
    let attractor = v4f(curs.x, curs.y, 0.0, 0.3);
    if (length(attractor.xy - ppos.xy) < attractor.w && stuff.mouse_left == 0u) {
        pvel = pvel + normalize(attractor.xyz - ppos)*min(1.0/length(attractor.xyz - ppos), 0.9);
    }

    // update particle
    ppos = ppos + 0.001*pvel;
    pvel = pvel*0.97;
    
    // store particle
    particle.p = ppos;
    particle.v = pvel;
    particle_buffer.buff[id] = particle;

    // store color
    ppos = ppos + look_offset;
    ppos = ppos*f32(stuff.render_height/2u);

    let pos = vec2<i32>(i32(round(ppos.x)), i32(round(ppos.y)));
    var col = textureLoad(screen_texture, pos).xyz;
    // if (col.x < 1.0 && length(pvel) > 0.02) {
    // if (col.x < 1.0) {
    //     col = col + 0.2;
    // } else if (col.x < 0.2) {
        // col = col + 0.1;
    // }
    col = v3f(1.0);
    textureStore(screen_texture, pos, v4f(col, 1.0));
}


[[stage(fragment)]]
fn main_fragment([[builtin(position)]] pos: vec4<f32>) -> [[location(0)]] vec4<f32> {
    let render_to_display_ratio = f32(stuff.render_height)/f32(stuff.display_height);
    let i = vec2<i32>(i32(pos.x*render_to_display_ratio), i32(pos.y*render_to_display_ratio));
    if (i.x >= i32(stuff.render_width)) {return v4f(0.0);};
    // let index = i.x + i.y*stuff.render_width;

    var col = textureLoad(screen_texture, i);

    // slowly remove color
    if (col.x > 0.) {
        textureStore(screen_texture, i, col - v4f(v3f(0.15), 0.0));
    }

    return col;
}