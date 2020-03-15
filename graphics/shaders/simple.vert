#version 450

layout(location=0)in vec2 a_pos2D;
layout(location=1)in vec4 a_color;
layout(location=2)in mat4 a_model;

layout(location=0)out vec4 v_color;

layout(set=0,binding=0)
uniform ViewUniforms{
	mat4 u_view_projection;
};

void main(){
	v_color=a_color;
	gl_Position=u_view_projection*a_model*vec4(a_pos2D,0.,1.);
	
}
