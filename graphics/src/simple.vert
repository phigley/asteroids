#version 330 core

in vec2  pos2D;

layout(std140) uniform ViewUniforms {
	mat4 projection;
};

layout(std140) uniform ModelUniforms {
	mat4 translation;
	vec4 color4D;
};

void main() {

	vec4 oriented_pos = projection*translation*vec4(pos2D, 0.0, 1.0);

    gl_Position = oriented_pos;
}
