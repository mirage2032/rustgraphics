#version 330 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;

out vec3 fragColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    mat4 modelViewProjection = projection * view * model;
    // Transform vertex position
    gl_Position = modelViewProjection * vec4(position, 1.0);
    fragColor = color;
}