// shader.frag
#version 450

layout(location=0) in vec2 vtexc;
layout(location=1) in vec3 vnorm;
layout(location=2) in vec3 vpos;
layout(location=0) out vec4 fcolour;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(set=1, binding=0)
uniform Uniforms {
    vec4 uviewpos;
    mat4 uviewproj;
};

layout(set=2, binding=0)
uniform Light {
    vec3 upos;
    vec3 ucolour;
};

void main() {
    vec4   objcolour = texture(sampler2D(t_diffuse, s_diffuse), vtexc);
    
    // Ambient lighting
    float  ambientstrength = 0.1;
    vec3   ambient_colour = ucolour * ambientstrength;
    
    // Diffuse lighting
    vec3   light_dir = normalize(upos - vpos);
    float  diffuse_strength = max(dot(vnorm, light_dir), 0.0);
    vec3   diffuse_colour = ucolour * diffuse_strength;
    
    // Specular highlights
    vec3   view_dir = normalize(uviewpos.xyz - vpos);
    vec3   half_dir = normalize(view_dir + light_dir);
    float  specular_strength = pow(max(dot(vnorm, half_dir), 0.0), 32.0);
    vec3   specular_colour = specular_strength * ucolour;

    vec3   result = (ambient_colour + diffuse_colour + specular_colour) * objcolour.xyz;

    fcolour = vec4(result, objcolour.a);
}