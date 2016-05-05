#version 150 core
in vec2 v_TexCoord;
out vec4 o_color;

uniform sampler2D in_depth;
uniform sampler2D in_color;

uniform int enabled;

uniform SSAOParams {
    float sample_rad;
};

uniform mat4 u_model_view_proj;

const int MAX_KERNEL_SIZE = 16;

void main() {
    vec3 Pos = texture(in_depth, v_TexCoord).xyz;
    vec4 screenColor = texture(in_color, v_TexCoord).rgba;

    float AO = 0.0;
    vec3 sample_sphere[MAX_KERNEL_SIZE];
    sample_sphere[ 0] = vec3( 0.5381, 0.1856,-0.4319);
    sample_sphere[ 1] = vec3( 0.1379, 0.2486, 0.4430);
    sample_sphere[ 2] = vec3( 0.3371, 0.5679,-0.0057);
    sample_sphere[ 3] = vec3(-0.6999,-0.0451,-0.0019);
    sample_sphere[ 4] = vec3( 0.0689,-0.1598,-0.8547);
    sample_sphere[ 5] = vec3( 0.0560, 0.0069,-0.1843);
    sample_sphere[ 6] = vec3(-0.0146, 0.1402, 0.0762);
    sample_sphere[ 7] = vec3( 0.0100,-0.1924,-0.0344);
    sample_sphere[ 8] = vec3(-0.3577,-0.5301,-0.4358);
    sample_sphere[ 9] = vec3(-0.3169, 0.1063, 0.0158);
    sample_sphere[10] = vec3( 0.0103,-0.5869, 0.0046);
    sample_sphere[11] = vec3(-0.0897,-0.4940, 0.3287);
    sample_sphere[12] = vec3( 0.7119,-0.0154,-0.0918);
    sample_sphere[13] = vec3(-0.0533, 0.0596,-0.5411);
    sample_sphere[14] = vec3( 0.0352,-0.0631, 0.5460);
    sample_sphere[15] = vec3(-0.4776, 0.2847,-0.0271);

    for (int i = 0; i < MAX_KERNEL_SIZE; i++) {
        vec3 samplePos = Pos + sample_sphere[i];
        vec4 offset = vec4(samplePos, 1.0);
        offset = u_model_view_proj * offset;
        offset.xy /= offset.w;
        offset.xy = offset.xy * 0.5 + vec2(0.5);

        float sampleDepth = texture(in_depth, offset.xy).z;

        if (abs(Pos.z - sampleDepth) < sample_rad) {
            AO += step(sampleDepth, samplePos.z);
        }
    }

    AO = 1.0 - AO/(float(MAX_KERNEL_SIZE) - 1.0);

    if (enabled > 0) {
        o_color = vec4(AO);
    } else {
        o_color = screenColor;
    }
}

// vim ft=glsl
