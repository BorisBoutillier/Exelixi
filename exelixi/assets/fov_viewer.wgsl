#import bevy_sprite::mesh2d_view_bind_group
#import bevy_sprite::mesh2d_struct

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
};
struct FovInfo {
    angle: f32;
    n_sectors: u32;
    color: vec4<f32>;
    sector_alpha: f32; 
};
[[group(0), binding(0)]]
var<uniform> view: View;

[[group(1), binding(0)]]
var<uniform> fov: FovInfo;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh2d;

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.uv = vertex.uv;
    out.clip_position = view.view_proj * world_position;
    return out;
}
[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let none = vec4<f32>(0.);
    let edge_color = fov.color;
    let sector_color = vec4<f32>(edge_color.x,edge_color.y,edge_color.z,fov.sector_alpha);
    let min_angle = -fov.angle/2.0;
    let max_angle = fov.angle/2.0;
    let lx = in.uv.x - 0.5;
    let ly = in.uv.y - 0.5;
    let in_fov_range = (lx*lx + ly*ly) < 0.5*0.5;
        let angle = atan2(ly,lx);
    // Is the point in FOV
    if ( in_fov_range  && angle>min_angle && angle<max_angle ){
        // Is the point on an edge of a sector.
        for (var theta:f32 = -fov.angle/2.0 ; theta < fov.angle/2.0 + 0.1 ; theta = theta + fov.angle/f32(fov.n_sectors)) {
            if (abs(angle-theta)<0.01) {
                return edge_color;
            }
        }
        return sector_color;
    }
    return none;
}
