#version 300 es

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aOffset;

out vec3 v_Color;

void main() {
    v_Color = aColor; 
    float frac = 100.0 / float(gl_InstanceID);
    gl_Position = vec4(aPos / frac + aOffset, 0.0, 1.0);

}