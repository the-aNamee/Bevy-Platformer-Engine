@group(0) @binding(0) var u_aspect_ratio: f32;


const TARGET_ASPECT_RATIO: f32 = 16.0 / 9.0;



@fragment
fn main(@location(0) in_uv: vec2<f32>) -> @location(0) vec4<f32> {
    let aspect_ratio = u_aspect_ratio;

    // Get normalized UV cords.
    let uv = in_uv * 2.0 - vec2<f32>(1.0, 1.0);

    let scale = if aspect_ratio > TARGET_ASPECT_RATIO {
        vec2<f32>(1.0, aspect_ratio / screen_aspect)
    } else {
        vec2<f32>(screen_aspect / aspect_ratio, 1.0)
    };

    // Letterbox
    if (abs(uv.y) > scale.y || abs(uv.x) > scale.x) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Black.
    }

    return vec4<f32>(1.0, 1.0, 1.0, 0.0);
}