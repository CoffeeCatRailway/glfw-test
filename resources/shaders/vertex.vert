#version 330 core

layout (location = 0) in vec3 i_pos;

uniform mat4 u_pvm;

out vec2 f_pos;
out vec3 f_normal;

void main() {
    gl_Position = u_pvm * vec4(i_pos, 1.);
    f_pos = i_pos.xy;
    f_normal = normalize(i_pos);
}
