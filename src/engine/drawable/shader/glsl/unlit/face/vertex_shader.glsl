#version 310 es
precision highp float;

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

out vec3 Normal;

uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 projection_mat;

void main() {
    gl_Position = projection_mat * view_mat * model_mat * vec4(position, 1.0);
    // Transform vertex position
    Normal = normal;
}