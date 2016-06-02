#version 150

uniform vec3 u_light;
//uniform sampler2D normal_tex;

in vec3 v_color;
in vec3 v_normal;
in vec3 v_position;
//in vec2 v_tex_coords;

out vec4 f_color;

const vec3 specular_color = vec3(1.0, 1.0, 1.0);
const float shininess = 16.0;
//const float screen_gamma = 2.2;

//mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
//    vec3 dp1 = dFdx(pos);
//    vec3 dp2 = dFdy(pos);
//    vec2 duv1 = dFdx(uv);
//    vec2 duv2 = dFdy(uv);
//    vec3 dp2perp = cross(dp2, normal);
//    vec3 dp1perp = cross(normal, dp1);
//    vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
//    vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;
//    float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
//    return mat3(T * invmax, B * invmax, normal);
//}

void main() {
    vec3 diffuse_color = v_color;
    vec3 ambient_color = diffuse_color * 0.2;

    //vec3 normal_map = texture(normal_tex, v_tex_coords).rgb;
    //mat3 tbn = cotangent_frame(v_normal, v_position, v_tex_coords);
    //vec3 normal = normalize(tbn * -(normal_map * 2.0 - 1.0));
    vec3 normal = normalize(v_normal);
    vec3 light_dir = normalize(u_light);

    float lambertian = max(dot(normal, light_dir), 0.0);
    float specular = 0.0;

    if (lambertian > 0.0) {
        vec3 view_dir = normalize(-v_position);

        // Blinn-Phong shading:
        vec3 half_dir = normalize(light_dir + view_dir);
        float spec_angle = max(dot(half_dir, normal), 0.0);
        specular = pow(spec_angle, shininess);

        // Phong shading:
        //vec3 reflect_dir = reflect(-light_dir, normal);
        //float spec_angle = max(dot(reflect_dir, view_dir), 0.0);
        //specular = pow(spec_angle, shininess / 4.0);
    }

    vec3 linear_color = ambient_color + lambertian * diffuse_color + specular * specular_color;

    // apply gamma correction (assume ambient_color, diffuse_color and spec_color have been
    // linearized, i.e. have no gamma correction in them)
    //vec3 color_gamma_corrected = pow(linear_color, vec3(1.0 / screen_gamma));
    vec3 color_gamma_corrected = linear_color;
    // use the gamma corrected color in the fragment
    f_color = vec4(color_gamma_corrected, 1.0);
}
