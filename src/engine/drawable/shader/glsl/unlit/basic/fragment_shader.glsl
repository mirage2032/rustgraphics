#version 450 core

in vec3 fragNormal;
out vec4 FragColor;

void main() {
    FragColor = vec4(fragNormal, 1.0);
}