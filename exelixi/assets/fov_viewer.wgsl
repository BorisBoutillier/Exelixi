struct FovInfo {
    angle: f32,
    n_sectors: u32,
    color: vec4<f32>,
    sector_alpha: f32,
};
@group(1) @binding(0)
var<uniform> fov: FovInfo;

@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let none = vec4<f32>(0.);
    let edge_color = fov.color;
    let sector_color = vec4<f32>(edge_color.x, edge_color.y, edge_color.z, fov.sector_alpha);
    let min_angle = -fov.angle / 2.0;
    let max_angle = fov.angle / 2.0;
    let lx = uv.x - 0.5;
    let ly = uv.y - 0.5;
    let in_fov_range = (lx * lx + ly * ly) < 0.5 * 0.5;
    let angle = atan2(ly, lx);
    // Is the point in FOV
    if in_fov_range && angle > min_angle && angle < max_angle {
        // Is the point on an edge of a sector.
        for (var theta: f32 = -fov.angle / 2.0 ; theta < fov.angle / 2.0 + 0.1 ; theta = theta + fov.angle / f32(fov.n_sectors)) {
            if abs(angle - theta) < 0.01 {
                return edge_color;
            }
        }
        return sector_color;
    }
    return none;
}
