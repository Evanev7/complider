// shader.vert
#version 450

layout(location=0) in vec3 ipos;
layout(location=1) in vec2 itexc;
layout(location=2) in vec3 inorm;

layout(location=0) out vec2 vtexc;
layout(location=1) out vec3 vnorm;
layout(location=2) out vec3 vpos;

layout(location = 5) in vec4 imod0;
layout(location = 6) in vec4 imod1;
layout(location = 7) in vec4 imod2;
layout(location = 8) in vec4 imod3;

layout(set=1, binding=0)
uniform Uniforms {
    mat4 uviewproj;
};

void main() {
    mat4 imod = mat4(
        imod0,
        imod1,
        imod2,
        imod3
    );
    vtexc = itexc;
    mat3 normal_matrix = mat3(transpose(inverse(imod)));
    vnorm = normal_matrix * inorm;
    vec4 model_space = imod * vec4(ipos, 1.0);
    vpos = model_space.xyz;
    gl_Position = uviewproj * imod * vec4(ipos, 1.0);
}