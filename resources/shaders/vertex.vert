#version 330 core

layout (location = 0) in vec3 i_pos;

out vec2 v_pos;

void main() {
    gl_Position = vec4(i_pos, 1.);
    v_pos = i_pos.xy;
}
