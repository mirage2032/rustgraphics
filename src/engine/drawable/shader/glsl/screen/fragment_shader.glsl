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

    FragColor = vec4(mapped, 1.0);
}