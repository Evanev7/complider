// shader.frag
#version 450

layout(location=0) in vec2 vtexc;
layout(location=0) out vec4 fcolour;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    fcolour = texture(sampler2D(t_diffuse, s_diffuse), vtexc);
}