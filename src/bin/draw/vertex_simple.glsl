#version 150

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

in vec3 position;
in vec3 color;

out vec3 v_color;

void main() {
    mat4 modelview = view * model;
    gl_Position = perspective * modelview * vec4(position, 1.0);
    v_color = color;
}
