#version 150 core
in vec3 a_pos;
out vec2 v_TexCoord;
out vec2 v_blurTexCoord[14];

uniform int u_horizontal;

void main() {
    if (gl_VertexID == 0) {
        gl_Position = vec4(-1.0, -1.0, 0.0, 1.0);
        v_TexCoord = vec2(0.0, 0.0);
    }
    if (gl_VertexID == 1) {
        gl_Position = vec4(-1.0, 1.0, 0.0,1.0);
        v_TexCoord = vec2(0.0, 1.0);
    }
    if (gl_VertexID == 2) {
        gl_Position = vec4(1.0, 1.0, 0.0, 1.0);
        v_TexCoord = vec2(1.0, 1.0);
    }
    if (gl_VertexID == 3) {
        gl_Position = vec4(1.0, -1.0, 0.0, 1.0);
        v_TexCoord = vec2(1.0, 0.0);
    }

    if (u_horizontal == 1.0) {
        v_blurTexCoord[ 0] = vec2(-0.028, 0.0);
        v_blurTexCoord[ 1] = vec2(-0.024, 0.0);
        v_blurTexCoord[ 2] = vec2(-0.020, 0.0);
        v_blurTexCoord[ 3] = vec2(-0.016, 0.0);
        v_blurTexCoord[ 4] = vec2(-0.012, 0.0);
        v_blurTexCoord[ 5] = vec2(-0.008, 0.0);
        v_blurTexCoord[ 6] = vec2(-0.004, 0.0);
        v_blurTexCoord[ 7] = vec2( 0.004, 0.0);
        v_blurTexCoord[ 8] = vec2( 0.008, 0.0);
        v_blurTexCoord[ 9] = vec2( 0.012, 0.0);
        v_blurTexCoord[10] = vec2( 0.016, 0.0);
        v_blurTexCoord[11] = vec2( 0.020, 0.0);
        v_blurTexCoord[12] = vec2( 0.024, 0.0);
        v_blurTexCoord[13] = vec2( 0.028, 0.0);
    } else {
        v_blurTexCoord[ 0] = vec2(0.0, -0.028);
        v_blurTexCoord[ 1] = vec2(0.0, -0.024);
        v_blurTexCoord[ 2] = vec2(0.0, -0.020);
        v_blurTexCoord[ 3] = vec2(0.0, -0.016);
        v_blurTexCoord[ 4] = vec2(0.0, -0.012);
        v_blurTexCoord[ 5] = vec2(0.0, -0.008);
        v_blurTexCoord[ 6] = vec2(0.0, -0.004);
        v_blurTexCoord[ 7] = vec2(0.0,  0.004);
        v_blurTexCoord[ 8] = vec2(0.0,  0.008);
        v_blurTexCoord[ 9] = vec2(0.0,  0.012);
        v_blurTexCoord[10] = vec2(0.0,  0.016);
        v_blurTexCoord[11] = vec2(0.0,  0.020);
        v_blurTexCoord[12] = vec2(0.0,  0.024);
        v_blurTexCoord[13] = vec2(0.0,  0.028);
    }
}

// vim ft=glsl

