// shader.vert
#version 450

layout(location=0) in vec3 ipos;
layout(location=0) out vec3 vcolour;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 uviewproj;
};

layout(set=1, binding=0)
uniform Light {
    vec3 upos;
    vec3 ucolour;
};

float scale = 0.25;

void main() {
    vec3 vpos = ipos * scale + upos;
    gl_Position = uviewproj * vec4(vpos, 1);

    vcolour = ucolour;
}
