#version 330 core

uniform vec2 uResolution;
uniform sampler2DArray font_buffer;
float smoothness = 1.0;

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

float median(float r, float g, float b)
{
    return max(min(r, g), min(max(r, g), b));
}

float circle_signed_dist(vec2 position, in vec2 dimensions, float radius) 
{
    return length(position * dimensions) - (radius * dimensions.x);
}

float box_signed_dist(in vec2 position, in vec2 dimensions, in vec4 corner_radii)
{
    vec2 bounds = vec2(0.5);
    vec2 quadrant_position = step(vec2(0.5), position);
    int corner_radius_index = int(quadrant_position.x) + int(quadrant_position.y) * 2;
    float corner_radius = corner_radii[corner_radius_index];

    vec2 centred_position = position - 0.5;
    centred_position = centred_position * dimensions;
    bounds = bounds * dimensions;

    vec2 offset = abs(centred_position) - bounds + corner_radius;
    return min(max(offset.x, offset.y), 0.0) + length(max(offset, 0.0)) - corner_radius;
}

void main()
{
    vec3 inner_colour = fs_in.inner_colour.rgb;
    vec3 outer_colour = fs_in.outer_colour.rgb;
    float stroke_width = fs_in.extra_data_1.r;
    
    float alpha = 0.00;
    vec3 current_colour;
    
    if(fs_in.identification.r == 0) 
    {
        float outer_radius = 0.5;
        float dist = circle_signed_dist(fs_in.texture_coord - 0.5, fs_in.dimensions, outer_radius);
        float outer = smoothstep(smoothness, -smoothness, dist);
        float inner = smoothstep(-stroke_width + smoothness, -stroke_width - smoothness, dist);
        alpha = smoothstep(0.00, -smoothness, dist);
        current_colour = mix(outer_colour, inner_colour, inner) * outer;
    }
    
    if(fs_in.identification.r == 1) 
    {
        float stroke_width = fs_in.extra_data_1.r;

        vec4 corner_radii = fs_in.extra_data_2;
        float dist = box_signed_dist(fs_in.texture_coord, fs_in.dimensions, corner_radii);
        float outer = smoothstep(smoothness, -smoothness, dist);
        float inner = smoothstep(-stroke_width + smoothness, -stroke_width - smoothness, dist);
        alpha = smoothstep(0.00, -smoothness, dist);
        current_colour = mix(outer_colour, inner_colour, inner) * outer;
    }

    if(fs_in.identification.r == 2) 
    {
        vec3 sample = texture(font_buffer, vec3(fs_in.texture_coord, fs_in.identification.g)).rgb;
        float dist = median(sample.r, sample.g, sample.b);
        float width = fwidth(dist);
        alpha = smoothstep(0.5 - width, 0.5 + width, dist);
        current_colour = outer_colour;
    }

    Color = vec4(current_colour, alpha);
}