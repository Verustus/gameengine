#version 430

layout(location = 2) in vec2 vertex_texture_coords;
layout(location = 0) out vec4 color;

uniform sampler2D texture_2d;
uniform Material {
    vec3 color_override;
};

void main() {
    vec4 texture_color = texture(texture_2d, vertex_texture_coords);

    if (color_override.r < 0.0) {
        color = vec4(texture_color.rgb * texture_color.a, texture_color.a);
    } else {
        color = vec4(color_override * texture_color.a, texture_color.a);
    }
}