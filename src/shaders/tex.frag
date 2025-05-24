// tex.frag
#version 410 core
in vec3 v_color;
in vec2 v_texcoord;
out vec4 FragColor;
uniform sampler2D tex;

void main() {
        FragColor = texture(tex, v_texcoord);
}