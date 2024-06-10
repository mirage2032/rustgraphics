#version 330 core

out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D diffuse_texture;
uniform sampler2D depth_texture;

const float exposure = 1.0;

void main()
{
    vec3 hdrColor = texture(diffuse_texture, TexCoord).rgb;
    vec3 mapped = hdrColor / (hdrColor + vec3(1.0));
//    mapped = vec3(1.0) - exp(-mapped * exposure);

//    float near = 0.1; // Adjust this value to match your near plane distance
//    float far = 100.0; // Adjust this value to match your far plane distance
//
//    float depthVal = texture(depth_texture,TexCoord).r;
//    float linearDepth = (2.0 * near) / (far + near - depthVal * (far - near));
//    vec3 depthCol = vec3(1.0-linearDepth);

    FragColor = vec4(hdrColor, 1.0);
}