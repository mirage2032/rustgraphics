#version 310 es
#extension GL_EXT_geometry_shader : enable
#extension GL_OES_geometry_shader : enable
precision highp float;

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in vec3 Normal[];

out vec3 fragCol; // Output normal to fragment shader

void main() {
    // Calculate the normal of the triangle
    vec3 n0 = Normal[0];
    vec3 n1 = Normal[1];
    vec3 n2 = Normal[2];
    vec3 face_normal = normalize(n0 + n1 + n2);
    fragCol = face_normal;

    // Emit the vertices
    for (int i = 0; i < gl_in.length(); ++i) {
        gl_Position = gl_in[i].gl_Position;
        EmitVertex();
    }

    EndPrimitive();
}