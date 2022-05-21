#version 330 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 tcoords;

out vec2 vtcoords;
uniform float aspect_ratio;
uniform float infobox_height;

void main() {
    float hscale = 1.0f;
    float vscale = 1.0f / aspect_ratio;
    if (aspect_ratio < 1.0f) {
        vscale = 1.0f;
        hscale = aspect_ratio;
    }

    vscale = vscale * (2.0f - infobox_height) / 2.0f;
    float x = pos.x * hscale;
    float y = pos.y * vscale + infobox_height / 2.0f;
    gl_Position = vec4(x, y, 0.0, 1.0);
    vtcoords = tcoords;
}
