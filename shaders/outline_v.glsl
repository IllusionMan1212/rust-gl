#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
  // scale the vertex positions by their normals to an outline that's slightly bigger than the mesh
  gl_Position = projection * view * model * vec4(aPos + normalize(aNormal) * 0.02, 1.0f);
}

