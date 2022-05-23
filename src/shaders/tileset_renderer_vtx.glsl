#version 330 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 tcoords;

out vec2 vtcoords;
uniform vec4 render_area;

void main() {
    float xscale = (render_area[2] - render_area[0]) / 2.0;
    float yscale = (render_area[3] - render_area[1]) / 2.0;

    float x = render_area[0] + (pos[0] + 1.0) * xscale;
    float y = render_area[1] + (pos[1] + 1.0) * yscale;

    gl_Position = vec4(x, y, 0.0, 1.0);

    vtcoords = tcoords;
}
