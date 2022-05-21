#version 330 core
in vec2 vtcoords;
out vec4 fcolor;

uniform sampler2D texture0;

void main() {
    fcolor = texture(texture0, vtcoords);
}
