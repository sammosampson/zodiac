#version 330 core
precision mediump float;

in GM_OUT 
{
    vec2 dimensions;
    vec2 texture_coord;
    vec4 inner_colour;
    vec4 outer_colour;
    flat ivec2 identification;
    vec4 extra_data_1;
    vec4 extra_data_2;
} fs_in;

out vec4 Color;

void main()
{
    vec3 inner_colour = fs_in.inner_colour.rgb;
     Color = vec4(inner_colour, 1.0);
}