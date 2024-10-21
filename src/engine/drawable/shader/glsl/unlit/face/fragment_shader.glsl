#version 310 es
precision highp float;

in vec3 fragCol;

out vec4 FragColor;

void main() {
    FragColor = vec4(fragCol, 1.0);
}