#version 330 core

out vec4 out_color;
in vec3 ourColor;

uniform bool is_red;

void main() {
    if(is_red){
        out_color = vec4(1.0, 0.0, 0.0, 1.0);
    } else {
        out_color = vec4(ourColor, 1.0);
    }
}
