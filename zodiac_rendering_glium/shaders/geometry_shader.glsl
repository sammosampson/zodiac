#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

in VS_OUT
{
    vec2 dimensions;
    vec2 clip_space_dimensions;
    vec4 inner_colour;
    vec4 outer_colour;
    flat ivec2 identification;
    vec4 extra_data_1;
    vec4 extra_data_2;
} gm_in[];

out GM_OUT 
{
    vec2 dimensions;
    vec2 texture_coord;
    vec4 inner_colour;
    vec4 outer_colour;
    flat ivec2 identification;
    vec4 extra_data_1;
    vec4 extra_data_2;
} gm_out;

void createVertex(vec2 pos, vec2 scale, vec2 corner, float u, float v) {
    vec2 scaled = scale * corner;
    vec2 transformed = pos + scaled;
    gl_Position = vec4(transformed, 0.0, 1.0);
    gm_out.texture_coord = vec2(u, v);
    gm_out.dimensions = gm_in[0].dimensions;
    gm_out.inner_colour = gm_in[0].inner_colour;
    gm_out.outer_colour = gm_in[0].outer_colour;
    gm_out.identification = gm_in[0].identification;
    gm_out.extra_data_1 = gm_in[0].extra_data_1;
    gm_out.extra_data_2 = gm_in[0].extra_data_2;
    EmitVertex();
}

void main()
{
    vec2 pos = gl_in[0].gl_Position.xy;;
    vec2 size = gm_in[0].clip_space_dimensions; 

    float one = 1.0;
    vec2 bottomLeft = vec2(-one, -one);
    vec2 bottomRight = vec2(one, -one);
    vec2 topLeft = vec2(-one, one);
    vec2 topRight = vec2(one, one);
    
    createVertex(pos, size, bottomLeft, 0.0, 1.0);
    createVertex(pos, size, bottomRight, 1.0, 1.0);
    createVertex(pos, size, topLeft, 0.0, 0.0);
    createVertex(pos, size, topRight, 1.0, 0.0);

    EndPrimitive();
}