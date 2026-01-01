#version 330 core

layout (location = 0) in vec3 i_pos;

uniform mat4 u_pvm;

out vec2 v_pos;
out vec3 v_normal;

void main() {
    gl_Position = u_pvm * vec4(i_pos, 1.);
    v_pos = i_pos.xy;
    v_normal = normalize(i_pos);
}
