// shader.frag
#version 450

layout(location=0) in vec3 vcolour;
layout(location=0) out vec4 fcolour;

void main() {
    fcolour = vec4(vcolour, 1.0);
}