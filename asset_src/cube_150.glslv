#version 150 core
in ivec3 a_pos;
in ivec2 a_tex_coord;
in int a_face;

out vec2 v_TexCoord;
uniform ivec3 u_faces;
uniform float u_height;
uniform float u_width;

uniform mat4 u_model_view_proj;

void main() {

    int pos = u_faces[a_face/2];
    int id;

    if (a_face % 2 == 0) {
        id = pos >> 16;
    } else {
        id = pos & 0xFFFF;
    }


    vec2 idx = vec2(id % 32, id / 32);
    vec2 dim = vec2(16.0f/u_width, 16.0f/u_height);

    vec2 coords;
    coords.x = (idx.x + a_tex_coord.x) * dim.x;
    coords.y = (idx.y + a_tex_coord.y) * dim.y;

    v_TexCoord = coords;
    gl_Position = u_model_view_proj * vec4(a_pos, 1.0);
}

// vim ft:glsl


