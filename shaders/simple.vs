#version 460

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texture_coords;
//layout(location = 2) in vec3 normal;

layout(location = 2) out vec2 vertex_texture_coords;
/*layout(location = 1) out vec3 v_normal;

layout(set = 0, binding = 1) uniform Camera {
    mat4 perspective;
    vec3 view;
} camera;*/

void main() {
    vertex_texture_coords = texture_coords;

    /*/vec3 finalPosition = camera.view * position;

    gl_Position = camera.perspective * vec4(finalPosition, 1.0);/*/
    gl_Position = vec4(position, 1.0);
}