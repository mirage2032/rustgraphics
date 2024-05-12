#version 330 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 3) in vec3 transform;

out vec3 fragColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

mat4 translateLocal(mat4 matrix, vec3 translation) {
    // Convert translation vector to local space
    vec3 localTranslation = (matrix * vec4(translation, 0.0)).xyz;
    
    // Apply translation
    matrix[3] += vec4(localTranslation, 0.0);
    
    return matrix;
}

void main() {
    mat4 modelViewProjection = projection * view * translateLocal(model,transform);
    // Transform vertex position
    gl_Position = modelViewProjection * vec4(position, 1.0);
    fragColor = normal;
}