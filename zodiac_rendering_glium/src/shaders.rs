use glium::*;

pub fn create_shader_program(display: &Display) -> Result<Program, ProgramCreationError> {
    let vertex_shader_src = r#"
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
    "#;

    let geometry_shader_src = r#"
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
    "#;

    let fragment_shader_src = r#"
        #version 330 core

        uniform vec2 uResolution;
        uniform sampler2DArray font_buffer;
        float smoothness = 0.002;
        
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
        
        float circle_signed_dist(vec2 position, float radius) 
        {
            return length(position) - radius;
        }
        
        float box_signed_dist(in vec2 position, in vec4 corner_radii, float aspect)
        {
            vec2 bounds = vec2(0.5);
            vec2 quadrant_position = step(vec2(0.5), position);
            int corner_radius_index = int(quadrant_position.x) + int(quadrant_position.y) * 2;
            float corner_radius = corner_radii[corner_radius_index];
        
            vec2 centred_position = position - 0.5;    
            vec2 offset = abs(centred_position) - bounds + corner_radius;
            return min(max(offset.x, offset.y), 0.0) + length(max(offset, 0.0)) - corner_radius;
        }
        
        void main()
        {
            vec3 inner_colour = fs_in.inner_colour.rgb;
            vec3 outer_colour = fs_in.outer_colour.rgb;
            float aspect = uResolution.y / uResolution.x;
            float stroke_width = fs_in.extra_data_1.r / fs_in.dimensions.x;
        
            float alpha = 0.00;
            vec3 current_colour;
            
            if(fs_in.identification.r == 0) 
            {
                float outer_radius = 0.5;
                float dist = circle_signed_dist(fs_in.texture_coord - 0.5, outer_radius);
                float outer = smoothstep(smoothness, -smoothness, dist);
                float inner = smoothstep(-stroke_width + smoothness, -stroke_width - smoothness, dist);
                alpha = smoothstep(0.00, -smoothness, dist);
                current_colour = mix(outer_colour, inner_colour, inner) * outer;
            }
            
            if(fs_in.identification.r == 1) 
            {
                vec4 corner_radii = fs_in.extra_data_2;
                float dist = box_signed_dist(fs_in.texture_coord, corner_radii, aspect);
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
    "#;

    Program::from_source(display, vertex_shader_src, fragment_shader_src, Some(geometry_shader_src))
}
    