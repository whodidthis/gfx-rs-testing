#version 150

in vec2 a_Pos;
in vec2 a_Normal;
out vec2 v_Normal;

uniform float u_Width;

void main() {
    v_Normal = a_Normal;
    vec4 delta = vec4(a_Normal * u_Width, 0.0, 0.0);
    vec4 pos = vec4(a_Pos, 0.0, 1.0);
    gl_Position = pos + delta;
}
