// shader.vert
#version 450

layout(location=0) in vec3 ipos;
layout(location=1) in vec2 itexc;
layout(location=0) out vec2 vtexc;

layout(set=1, binding=0)
uniform Uniforms {
    mat4 uviewproj;
};

void main() {
    vtexc = itexc;
    gl_Position = uviewproj * vec4(ipos, 1.0);
}