#version 150

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

in vec2 position;
in vec2 tex_coords;
in vec4 colour;

out vec2 v_tex_coords;
out vec4 v_colour;

void main() {
    mat4 modelview = view * model;
    //gl_Position = vec4(position, 0.0, 1.0);
    //gl_Position = perspective * vec4(position, 0.0, 1.0);
    gl_Position = perspective * modelview * vec4(position, 0.0, 1.0);
    v_tex_coords = tex_coords;
    v_colour = colour;
}
