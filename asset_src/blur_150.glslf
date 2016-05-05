#version 150 core
in vec2 v_TexCoord;
in vec2 v_blurTexCoord[14];

out vec4 o_color;

uniform sampler2D in_color;
uniform int u_horizontal;

void main() {
    float lower = 0.33333333;
    float upper = 0.66666666;

    if (v_TexCoord.y > lower && v_TexCoord.y < upper) {
        o_color  = texture2D(in_color, v_TexCoord);
    } else {
        float coef = 1.0;

        if (v_TexCoord.y < 0.5) {
            coef = 1.0 - (v_TexCoord.y / lower);
        } else {
            coef = ((v_TexCoord.y - upper) / (1 - upper));
        }

        o_color  = vec4(0.0);
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[ 0]*coef)*0.0044299121055113265;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[ 1]*coef)*0.00895781211794;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[ 2]*coef)*0.0215963866053;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[ 3]*coef)*0.0443683338718;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[ 4]*coef)*0.0776744219933;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[ 5]*coef)*0.115876621105;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[ 6]*coef)*0.147308056121;
        o_color += texture2D(in_color, v_TexCoord                          )*0.159576912161;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[ 7]*coef)*0.147308056121;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[ 8]*coef)*0.115876621105;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[ 9]*coef)*0.0776744219933;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[10]*coef)*0.0443683338718;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[11]*coef)*0.0215963866053;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[12]*coef)*0.00895781211794;
        o_color += texture2D(in_color, v_TexCoord + v_blurTexCoord[13]*coef)*0.0044299121055113265;
    }
}

// vim ft=glsl
