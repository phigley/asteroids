#version 330 core

out vec4 colorOutput;

layout(std140) uniform ModelUniforms {
	mat4 translation;
	vec4 color4D;
};

void main() {
    colorOutput = color4D;
}