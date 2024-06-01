#version 460 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coords;

uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 projection_mat;

out vec3 FragPos;
out vec3 Normal;
out vec2 TexCoords;
out vec3 ViewPos;

void main() {
    Normal = mat3(transpose(inverse(model_mat))) * normal;  // Transform the normal to world space
    vec4 worldPosition = model_mat * vec4(position, 1.0);
    FragPos = vec3(worldPosition); // Transform the position to world space
    TexCoords = tex_coords;
    ViewPos = vec3(view_mat * worldPosition); // Calculate the view space position

    gl_Position = projection_mat * view_mat * worldPosition;
}