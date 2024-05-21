#version 330 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

uniform Material material;

out vec4 FragColor;

void main() {
    float intensity = clamp(material.shininess / 100.0, 0.0, 1.0);
    vec3 adjustedColor = mix(material.diffuse * 0.5, material.diffuse, intensity);
    FragColor = vec4(adjustedColor, 1.0);
}