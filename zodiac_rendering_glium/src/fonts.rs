use std::io::Cursor;
use glium::*;
use glium::texture::texture2d_array::*;
use glium::texture::*;
use image::imageops::*;

pub fn create_font_array(display: &Display) -> Result<Texture2dArray, TextureCreationError> {
    let mut glyphs = vec!();
    let glyph_dimensions = (96, 96);    
    
    println!("loading font texture");
    
    let mut font_image = image::load(Cursor::new(&include_bytes!("../images/segoeui-1.png")[..]), image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    
    println!("loaded font texture");
    
    let font_image_dimensions = font_image.dimensions(); 
    let glyph_count = font_image_dimensions.1 / glyph_dimensions.1;
    
    for glyph_index in 0..glyph_count {
        let glyph_image = crop(
            &mut font_image,
            0,
            glyph_index * glyph_dimensions.1,
            glyph_dimensions.0,
            glyph_dimensions.1).to_image();

        let glyph_image_dimensions = glyph_image.dimensions();

        glyphs.push(RawImage2d::from_raw_rgba(glyph_image.into_raw(), glyph_image_dimensions));
    }

    Texture2dArray::new(display, glyphs)  
}