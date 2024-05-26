#version 460 core

in vec3 fragNormal;

struct DirectionalLight {
    float intensity;
    vec3 color;
    vec3 direction;
};

struct PointLight {
    float intensity;
    vec3 color;
    vec3 position;
    float constant;
    float linear;
    float quadratic;
};

struct SpotLight {
    float intensity;
    vec3 color;
    vec3 position;
    vec3 direction;
    float constant;
    float linear;
    float quadratic;
    float cut_off;
    float outer_cut_off;
};

layout (std430, binding = 0) buffer Light {
    bool is_directional;
    DirectionalLight directional_light;
    int num_point_lights;
    PointLight point_light[4];
    int num_spot_lights;
    SpotLight spot_light[4];
} light;

uniform struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
} material;

out vec4 FragColor;

void main() {
    FragColor = vec4(fragNormal, 1.0);
}