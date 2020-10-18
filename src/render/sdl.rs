use std::collections::HashMap;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{Sdl2ImageContext, InitFlag, LoadTexture};
use sdl2::rect::Rect;
use sdl2::ttf::{Sdl2TtfContext, Font};

use nalgebra::Vector2;

use crate::misc::Vector;

pub struct SDLRenderImpl<'a> {
    sprite_cache: SpriteCache<'a>,
    font_cache: FontCache<'a>,
    context: SDLContext
}

impl SDLRenderImpl<'_> {
    pub fn render(&mut self, sprite_name: &str, sprite_pos: Vector, sprite_dim: Vector, cam_pos: Vector, cam_zoom: f32, cam_screen: Vector2<u32>) {
        // Camera transformation
        let pos_x = (((sprite_pos.x - cam_pos.x) / 5.0 * cam_zoom + 1.0) / 2.0 * cam_screen.x as f32) as i32;
        let pos_y = ((-(sprite_pos.y - cam_pos.y + sprite_dim.y) / 5.0 * cam_zoom * (cam_screen.x as f32 / cam_screen.y as f32) + 1.0) / 2.0 * cam_screen.y as f32) as i32;
        let dim_x = (sprite_dim.x * cam_screen.x as f32 * cam_zoom / 5.0 / 2.0) as u32;
        let dim_y = (sprite_dim.y * cam_screen.y as f32 * cam_zoom / 5.0 / 2.0 * (cam_screen.x as f32 / cam_screen.y as f32)) as u32;

        self.context.canvas.copy(self.sprite_cache.get(sprite_name), None, Rect::new(pos_x, pos_y, dim_x, dim_y)).unwrap();
    }

    pub fn write(&mut self, text: &str, font: &str, text_pos: Vector, text_dim: Vector, cam_pos: Vector, cam_zoom: f32, cam_screen: Vector2<u32>) {
        let (font, color) = self.font_cache.get(font);

        let text_surface = font.render(text).blended(color.clone()).unwrap();
        let (w, h) = text_surface.size();
        let text_texture = self.context.texture_creator.create_texture_from_surface(text_surface).unwrap();

        // Camera transformation
        let text_dim = Vector2::new(text_dim.y * (w as f32 / h as f32), text_dim.y);
        let pos_x = (((text_pos.x - cam_pos.x) / 5.0 * cam_zoom + 1.0) / 2.0 * cam_screen.x as f32) as i32;
        let pos_y = ((-(text_pos.y - cam_pos.y + text_dim.y) / 5.0 * cam_zoom * (cam_screen.x as f32 / cam_screen.y as f32) + 1.0) / 2.0 * cam_screen.y as f32) as i32;
        let dim_x = (text_dim.x * cam_screen.x as f32 * cam_zoom / 5.0 / 2.0) as u32;
        let dim_y = (text_dim.y * cam_screen.y as f32 * cam_zoom / 5.0 / 2.0 * (cam_screen.x as f32 / cam_screen.y as f32)) as u32;

        self.context.canvas.copy(&text_texture, None, Rect::new(pos_x, pos_y, dim_x, dim_y)).unwrap();
    }

    pub fn render_ss(&mut self, sprite_name: &str, sprite_pos: Vector2<i32>, sprite_dim: Vector2<u32>) {
        let pos = Vector2::new(sprite_pos.x, sprite_pos.y);
        let dim = sprite_dim;

        self.context.canvas.copy(self.sprite_cache.get(sprite_name), None, Rect::new(pos.x, pos.y, dim.x, dim.y)).unwrap();
    }

    pub fn write_ss(&mut self, text: &str, font: &str, text_pos: Vector2<i32>, text_dim: Vector2<u32>) -> bool {
        let (font, color) = self.font_cache.get(font);

        match font.render(text).blended(color.clone()) {
            Ok(text_surface) => {
                let (w, h) = text_surface.size();
                let text_texture = self.context.texture_creator.create_texture_from_surface(text_surface).unwrap();

                let dim = Vector2::new((text_dim.y as f32 * (w as f32 / h as f32)) as u32, text_dim.y);
                let pos = Vector2::new(text_pos.x, text_pos.y);

                self.context.canvas.copy(&text_texture, None, Rect::new(pos.x, pos.y, dim.x, dim.y)).unwrap();

                dim.x > text_dim.x
            },
            Err(_) => false
        }
    }

    pub fn pre(&mut self) {
        self.context.canvas.set_draw_color(Color::RGB(50, 50, 60));
        self.context.canvas.clear();
    }

    pub fn post(&mut self) {
        self.context.canvas.present();
    }

    pub fn add_sprite(&mut self, name: &str, file: &str) {
        // The texture can not outlive the creator as it is part of the same struct, so this should be safe
        let texture_creator = unsafe {
            &*(&self.context.texture_creator as *const TextureCreator<WindowContext>)
        };

        self.sprite_cache.insert(name.to_string(), texture_creator.load_texture(file).unwrap());
    }

    pub fn add_font(&mut self, name: &str, file: &str, size: u16, red: u8, green: u8, blue: u8) {
        // The font can not outlive the creator as it is part of the same struct, so this should be safe
        let font_context = unsafe {
            &*(&self.context.font as *const Sdl2TtfContext)
        };

        self.font_cache.insert(name.to_string(), font_context.load_font(file, size).unwrap(), Color::RGB(red, green, blue));
    }

    pub fn init<'a>(sdl_context: &'a Sdl, win_dim: Vector2<u32>) -> Self {
        let sdl_image_context = sdl2::image::init(InitFlag::PNG).unwrap();
        let sdl_font_context = sdl2::ttf::init().unwrap();

        let video = sdl_context.video().unwrap();

        let window = video.window("Vitrellogy", win_dim.x, win_dim.y)
            .position_centered()
            .resizable()
            .build().unwrap();

        let mut canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let texture_creator = canvas.texture_creator();

        let context = SDLContext {
            canvas: canvas,
            _image: sdl_image_context,
            font: sdl_font_context,
            texture_creator: texture_creator
        };

        Self {
            sprite_cache: SpriteCache::new(),
            font_cache: FontCache::new(),
            context: context,
        }
    }
}

struct SDLContext {
    canvas: Canvas<Window>,
    _image: Sdl2ImageContext,
    font: Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
}

struct SpriteCache<'a> {
    cache: HashMap<String, Texture<'a>>
}

impl<'a> SpriteCache<'a> {
    fn new() -> Self {
        SpriteCache {
            cache: HashMap::new()
        }
    }

    fn insert(&mut self, name: String, texture: Texture<'a>) {
        self.cache.insert(name, texture);
    }

    fn get(&self, name: &str) -> &Texture<'a> {
        match self.cache.contains_key(name) {
            true => &self.cache[name],
            false => panic!("sprite \"{}\" not found in cache", name)
        }
    }
}

struct FontCache<'a> {
    cache: HashMap<String, (Font<'a, 'static>, Color)>
}

impl<'a> FontCache<'a> {
    fn new() -> Self {
        FontCache {
            cache: HashMap::new()
        }
    }

    fn insert(&mut self, name: String, font: Font<'a, 'static>, color: Color) {
        self.cache.insert(name, (font, color));
    }

    fn get(&self, name: &str) -> &(Font<'a, 'static>, Color) {
        match self.cache.contains_key(name) {
            true => &self.cache[name],
            false => panic!("font \"{}\" not found in cache", name)
        }
    }
}
