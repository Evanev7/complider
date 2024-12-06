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
    float angle;
    vec3 ucolour;
};

float scale = 0.25;

mat3 base_rot = mat3(
    1, 0, 0,
    0, 0, 1,
    0, -1, 0
);

void main() {
    mat3 vroom_vroom = mat3(
        -cos(angle), 0, sin(angle),
        0, 1, 0,
        -sin(angle), 0, -cos(angle)
    );
    vec3 vpos = base_rot * vroom_vroom * ipos * scale + upos;
    gl_Position = uviewproj * vec4(vpos, 1);

    vcolour = ucolour;
}
