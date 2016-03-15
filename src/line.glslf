#version 150

in vec2 v_Normal;
out vec4 o_Color;

uniform vec3 u_Color;

void main() {
    o_Color = vec4(u_Color, 0.5);
    o_Color.a = 0.5;
}
