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
    vec3 halfwayDir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(normal, halfwayDir), 0.0), material.shininess);

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
    vec3 diffuse = light.color * diff * material.diffuse * light.intensity;
    vec3 specular = light.color * spec * material.specular * light.intensity;

    return ambient + diffuse + specular;
}

vec3 CalculateSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir) {
    // Calculate light direction
    vec3 lightDir = normalize(light.position - fragPos);

    // Calculate spotlight direction
    vec3 spotDir = normalize(-light.direction);

    // Calculate cosine of the angle between the light direction and the spotlight direction
    float theta = dot(lightDir, spotDir);

    // Convert cutoff angles from degrees to radians
    float outer_cut_off_rad = radians(light.outer_cut_off);
    float cut_off_rad = radians(light.cut_off);

    // Check if the fragment is inside the spotlight cone
    float intensity = smoothstep(cos(outer_cut_off_rad), cos(cut_off_rad), theta);

    // Calculate diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);

    // Calculate specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    vec3 halfwayDir = normalize(lightDir + viewDir); // Calculate halfway direction
    float spec = pow(max(dot(normal, halfwayDir), 0.0), material.shininess);

    // Calculate attenuation
    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    // Calculate final light intensity
    vec3 ambient = light.color * material.ambient * light.intensity * attenuation * intensity;
    vec3 diffuse = light.color * material.diffuse * diff * light.intensity * attenuation * intensity;
    vec3 specular = light.color * material.specular * spec * light.intensity * attenuation * intensity;

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
