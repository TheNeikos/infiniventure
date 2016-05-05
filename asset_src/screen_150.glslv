#version 150 core
in vec3 a_pos;
out vec2 v_TexCoord;

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
}

// vim ft=glsl

