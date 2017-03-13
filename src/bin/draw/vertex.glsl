#version 150

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

in vec3 position;
in vec3 normal;
//in vec2 texpos;
in vec3 color;

out vec3 v_color;
out vec3 v_normal;
out vec3 v_position;
//out vec2 v_tex_coords;

void main() {
    mat4 modelview = view * model;
    gl_Position = perspective * modelview * vec4(position, 1.0);
    v_color = color;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    v_position = gl_Position.xyz / gl_Position.w;
    //v_tex_coords = texpos;
}
