#version 330 core

in vec3 fragNorm;

uniform vec3 u_diff_color;

struct Ambient {
    float intensity;
    vec3 color;
    vec3 direction;
};

uniform Light {
    Ambient ambient;
} light;

out vec4 FragColor;

void main() {
    // calc color based on direction also
    float diff = max(dot(fragNorm, light.ambient.direction), 0.0);
    vec3 color = u_diff_color * diff * light.ambient.color * light.ambient.intensity;
    FragColor = vec4(color, 1.0);
}