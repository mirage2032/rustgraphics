#version 310 es
precision highp float;

out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D color_tex;
uniform sampler2D depth_stencil_tex;

//const float exposure = 1.0;
float near = 0.1;
float far = 300.0;

void main()
{
    vec3 hdrColor = texture(color_tex, TexCoord).rgb;
    float depthValue = texture(depth_stencil_tex, TexCoord).r;

    // Normalize depth value
    float normalizedDepth = (2.0 * near) / (far + near - depthValue * (far - near));

    // Convert normalized depth to a color
    vec3 depthColor = vec3(normalizedDepth);
//    vec3 mapped = hdrColor / (hdrColor + vec3(1.0));

    FragColor = vec4(hdrColor, 1.0);
}