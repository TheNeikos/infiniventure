#version 150 core
in vec2 v_TexCoord;

out vec4 o_color;

uniform sampler2D t_color;

void main() {
    vec4 tex = texture(t_color, v_TexCoord);
    o_color = tex;
}

// vim ft=glsl
