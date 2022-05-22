#version 330 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec4 color;

out vec4 vcolor;
uniform float infobox_height;

void main() {
    float hscale = 1.0f;
    float vscale = 1.0f;
    vscale = vscale * (2.0f - infobox_height) / 2.0f;

    float x = pos.x * hscale;
    float y = pos.y * vscale + infobox_height / 2.0f;
    gl_Position = vec4(x, y, 0.0, 1.0);
    
    vcolor = color;
}
