#version 330 core

in vec2 v_pos;
in vec3 v_normal;

uniform vec3 u_color;

out vec4 o_color;

void main() {
//    vec3 posColor = vec3(v_pos * .5 + .5, 0.);
//    o_color = vec4((posColor + u_color) * .5, 1.);
    o_color = vec4((v_normal * .5 + .5 + u_color) * .5, 1.);
}
