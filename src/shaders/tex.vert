#version 410 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;
layout(location = 2) in vec2 texcoord;

out vec3 v_color;
out vec2 v_texcoord;

void main() {
    gl_Position = vec4(position, 1.0);
    v_color = color;
    v_texcoord = texcoord;
}