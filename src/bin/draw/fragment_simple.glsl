#version 150

in vec3 v_color;
out vec4 f_color;
//const float gamma = 1.5;

void main() {
    //vec3 c_color = v_color * 0.9;
    //vec3 color = vec3(pow(c_color.x, gamma), pow(c_color.y, gamma), pow(c_color.z, gamma));
    //f_color = vec4(color, 1.0);
    f_color = vec4(v_color, 1.0);
}
