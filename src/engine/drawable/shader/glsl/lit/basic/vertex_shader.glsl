#version 310 es
precision highp float;

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coords;

uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 projection_mat;

out vec3 FragPos;
out vec3 ViewPos;
out vec3 Normal;
out vec2 TexCoords;

void main() {
    Normal = mat3(transpose(inverse(model_mat))) * normal;  // Transform the normal to world space TODO:: might want to do this on the CPU
    vec4 worldPosition = model_mat * vec4(position, 1.0);
    ViewPos = vec3(inverse(view_mat)[3]);
    FragPos = vec3(worldPosition); // Transform the position to world space
    TexCoords = tex_coords;

    gl_Position = projection_mat * view_mat * worldPosition;
}