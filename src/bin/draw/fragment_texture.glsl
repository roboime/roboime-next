#version 150

uniform sampler2D tex;

in vec2 v_tex_coords;
in vec4 v_colour;

out vec4 f_colour;

void main() {
    f_colour = v_colour * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);
}
