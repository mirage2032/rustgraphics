#version 330 core

uniform vec3 u_diff_color;

out vec4 FragColor;

void main() {
    FragColor = vec4(u_diff_color, 1.0);
}