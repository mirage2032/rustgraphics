#version 460 core
#define MAX_POINT_LIGHTS 5
#define MAX_SPOT_LIGHTS 5

in vec3 Normal;
in vec3 FragPos;
in vec3 ViewPos;

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

struct Light {
    bool is_directional;
    DirectionalLight directional_light;
    int point_count;
    PointLight point_lights[MAX_POINT_LIGHTS];
    int spot_count;
    SpotLight spot_lights[MAX_SPOT_LIGHTS];
};

layout (std140, binding = 5) uniform Lights {
    Light light;
};

uniform struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
} material;

out vec4 FragColor;

// Function to calculate point light contribution
vec3 CalculatePointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir) {
    vec3 lightDir = normalize(light.position - fragPos); // Direction from fragment to light
    // Diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // Specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    // Attenuation
    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));
    // Final light intensity
    vec3 ambient = light.color * material.ambient * attenuation * light.intensity;
    vec3 diffuse = light.color * diff * material.diffuse * attenuation * light.intensity;
    vec3 specular = light.color * spec * material.specular * attenuation * light.intensity;
    return ambient + diffuse + specular;
}

// Function to calculate directional light contribution
vec3 CalculateDirectionalLight(DirectionalLight light, vec3 normal, vec3 viewDir) {
    vec3 lightDir = normalize(-light.direction); // Direction from light source
    // Diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // Specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    // Final light intensity
    vec3 ambient = light.color * light.intensity * material.ambient;
    vec3 diffuse = light.color * diff * material.diffuse;
    vec3 specular = light.color * spec * material.specular;
    return ambient + diffuse + specular;
}

// Function to calculate spotlight contribution
vec3 CalculateSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir) {
    vec3 lightDir = normalize(light.position - fragPos);
    float theta = dot(lightDir, normalize(-light.direction));
    float epsilon = light.cut_off - light.outer_cut_off;
    float intensity = clamp((theta - light.outer_cut_off) / epsilon, 0.0, 1.0);

    // Diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // Specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    // Attenuation
    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));
    // Final light intensity
    vec3 ambient = light.color * light.intensity * material.ambient;
    vec3 diffuse = light.color * diff * material.diffuse;
    vec3 specular = light.color * spec * material.specular;
    ambient *= attenuation * intensity;
    diffuse *= attenuation * intensity;
    specular *= attenuation * intensity;
    return ambient + diffuse + specular;
}

vec3 CalculateLights(vec3 normal, vec3 fragPos, vec3 viewDir){
    vec3 result = vec3(0.0);

    // Calculate directional light
    if (light.is_directional) {
        result += CalculateDirectionalLight(light.directional_light, normal, viewDir);
    }

    // Calculate point lights
    for (int i = 0; i < light.point_count; ++i) {
        result += CalculatePointLight(light.point_lights[i], normal, fragPos, viewDir);
    }

    // Calculate spot lights
    for (int i = 0; i < light.spot_count; ++i) {
        result += CalculateSpotLight(light.spot_lights[i], normal, fragPos, viewDir);
    }

    return result;
}

void main() {
    vec3 normal = normalize(Normal);
    vec3 viewDir = normalize(-FragPos); // Assuming the camera is at the origin in view space
    vec3 result = CalculateLights(normal, FragPos, viewDir);

    // Output final color
    FragColor = vec4(result, 1.0);
}
