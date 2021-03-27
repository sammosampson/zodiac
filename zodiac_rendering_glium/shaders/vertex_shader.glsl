#version 330 core

uniform vec2 uResolution;

layout (location = 0) in ivec2 position;
layout (location = 1) in ivec2 dimensions;
layout (location = 2) in vec4 inner_colour;
layout (location = 3) in vec4 outer_colour;
layout (location = 4) in ivec2 identification;
layout (location = 5) in vec4 extra_data_1;
layout (location = 6) in vec4 extra_data_2;

out VS_OUT
{
    vec2 dimensions;
    vec2 clip_space_dimensions;
    vec4 inner_colour;
    vec4 outer_colour;
    flat ivec2 identification;
    vec4 extra_data_1;
    vec4 extra_data_2;
} vs_out;

vec2 toClipSpace(vec2 resolution, vec2 from)
{
    return vec2(
        from.x / (resolution.x / 2.0) - 1.0,
        1.0 - (from.y / (resolution.y / 2.0))
    );
}

void main()
{
    vec2 screen_position = (position + dimensions / 2);
    vec2 screen_dimensions = dimensions;
    gl_Position = vec4(toClipSpace(uResolution, vec2(screen_position)), 0.0, 1.0);
    vs_out.clip_space_dimensions = screen_dimensions / uResolution.xy;
    vs_out.dimensions = dimensions;
    vs_out.inner_colour = inner_colour;
    vs_out.outer_colour = outer_colour;
    vs_out.identification = identification;
    vs_out.extra_data_1 = extra_data_1;
    vs_out.extra_data_2 = extra_data_2;
}
    