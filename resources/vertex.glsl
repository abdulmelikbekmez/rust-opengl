#version 460 core

layout (location = 0) in vec3 a_Pos;
layout (location = 1) in vec3 a_Color;
layout (location = 2) in mat4 a_Model;

out vec3 ourColor;

uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * a_Model * vec4(a_Pos, 1.0);
    ourColor = a_Color;
}
