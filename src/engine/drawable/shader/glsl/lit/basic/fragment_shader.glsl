#version 310 es
precision highp float;

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

const float MIN_SHININESS = 1.0;

// Function to calculate point light contribution
vec3 CalculatePointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir) {
    vec3 lightDir = normalize(light.position - fragPos); // Direction from fragment to light

    // Diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);

    // Specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    vec3 halfwayDir = normalize(lightDir + viewDir);
    float shininess = max(material.shininess, MIN_SHININESS); // Ensure shininess is at least 1
    float spec = pow(max(dot(normal, halfwayDir), 0.0), shininess);

    // Attenuation
    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    // Final light intensity
    vec3 ambient = light.color * material.ambient * attenuation * light.intensity;
    vec3 diffuse = light.color * diff * material.diffuse * attenuation * light.intensity;
    vec3 specular = light.color * spec * material.specular * attenuation * light.intensity;

    return ambient + diffuse + specular;
}

// Function to calculate spot light contribution
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

    // Calculate specular shading using the halfway vector
    vec3 halfwayDir = normalize(lightDir + viewDir); // Correct normalization

    // Calculate the specular strength
    float specularStrength = 0.0;
    float shininess = max(material.shininess, MIN_SHININESS); // Ensure shininess is at least 1
    if (diff > 0.0) {
        specularStrength = pow(max(dot(normal, halfwayDir), 0.0), shininess);
    }

    // Calculate attenuation
    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    // Calculate final light intensity
    vec3 ambient = light.color * material.ambient * light.intensity * attenuation * intensity;
    vec3 diffuse = light.color * material.diffuse * diff * light.intensity * attenuation * intensity;
    vec3 specular = light.color * material.specular * specularStrength * light.intensity * attenuation * intensity;

    return ambient + diffuse + specular;
}


// Function to calculate directional light contribution
vec3 CalculateDirectionalLight(DirectionalLight light, vec3 normal, vec3 viewDir) {
    vec3 lightDir = normalize(-light.direction); // Direction from light source

    // Diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);

    // Specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float shininess = max(material.shininess, MIN_SHININESS); // Ensure shininess is at least 0.01
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);

    // Final light intensity
    vec3 ambient = light.color * light.intensity * material.ambient;
    vec3 diffuse = light.color * diff * material.diffuse * light.intensity;
    vec3 specular = light.color * spec * material.specular * light.intensity;

    return ambient + diffuse + specular;
}


vec3 CalculateLights(vec3 normal, vec3 fragPos, vec3 viewDir){
    vec3 result = vec3(0.0);

    // Calculate directional light
    if (light.is_directional) {
        float angle = dot(normal, -light.directional_light.direction);
        float smooth_angle = smoothstep(0.0, 0.3, angle);
        result += CalculateDirectionalLight(light.directional_light, normal, viewDir) * smooth_angle;
    }

    // Calculate point lights
    for (int i = 0; i < light.point_count; ++i) {
        float angle = dot(normal, normalize(light.point_lights[i].position - fragPos));
        float smooth_angle = smoothstep(0.0, 0.1, angle);
        result += CalculatePointLight(light.point_lights[i], normal, fragPos, viewDir) * smooth_angle;
    }

    // Calculate spot lights
    for (int i = 0; i < light.spot_count; ++i) {
        float angle = dot(normal, normalize(light.spot_lights[i].position - fragPos));
        float smooth_angle = smoothstep(-0.1, 0.1, angle);
        result += CalculateSpotLight(light.spot_lights[i], normal, fragPos, viewDir) * smooth_angle;
    }

    return result;
}

void main() {
    vec3 normal = normalize(Normal);
    vec3 viewDir = normalize(ViewPos-FragPos); // Assuming the camera is at the origin in view space
    vec3 result = CalculateLights(normal, FragPos, viewDir);

    // Output final color
    FragColor = vec4(result, 1.0);
}
