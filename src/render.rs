pub mod sdl;

use serde::{Serialize, Deserialize};

use nalgebra::Vector2;

use specs::{prelude::*, Component, DenseVecStorage};

use invader_macro::DefaultConstructor;
use crate::physics::TransformCom;
use crate::render::sdl::SDLRenderImpl;
use crate::input::{InputEventQueue, InputEvent};
use crate::input::key::{Key, KeysRes};
use crate::misc::{Convertable, Vector};

#[derive(Debug, DefaultConstructor)]
pub struct CameraRes {
    pub pos: Vector,
    pub zoom: f32,
    pub screen: Vector2<u32>
}

impl Default for CameraRes {
    fn default() -> Self {
        Self {
            pos: Vector::new(0.0, 0.0),
            zoom: 1.0,
            screen: Vector2::new(800, 600)
        }
    }
}

pub struct RenderSys<'a> {
    renderer: SDLRenderImpl<'a>
}

impl<'a, 'b> System<'a> for RenderSys<'b> {
    type SystemData = (Write<'a, UIEventQueue>,
        Read<'a, CameraRes>,
        Read<'a, InputEventQueue>,
        Read<'a, KeysRes>,
        ReadStorage<'a, TransformCom>,
        ReadStorage<'a, SpriteCom>,
        ReadStorage<'a, TextCom>,
        ReadStorage<'a, ButtonUICom>,
        ReadStorage<'a, TextUICom>,
        WriteStorage<'a, TextFieldUICom>,
        ReadStorage<'a, StartVerticalGroupCom>,
        ReadStorage<'a, StartHorizontalGroupCom>,
        ReadStorage<'a, EndGroupCom>,
        ReadStorage<'a, ConstraintCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut events, camera, input_events, keys, transforms, sprites, texts, buttons, text_labels, mut text_fields, v_group_start, h_group_start, group_end, constraints) = data;

        self.renderer.pre();

        for (sprite, transform) in (&sprites, &transforms).join() {
            self.renderer.render(&sprite.name, transform.pos, sprite.dim, camera.pos, camera.zoom, camera.screen);
        }

        for (text, transform) in (&texts, &transforms).join() {
            self.renderer.write(&text.text, &text.font, transform.pos, text.dim, camera.pos, camera.zoom, camera.screen);
        }

        events.clear();
        let mut container: Vec<(Vector2<u32>, Vector2<u32>, Vector2<u32>, bool)> = Vec::new();
        container.push((Vector2::new(0, 0), camera.screen, Vector2::new(0, 0), true));
        for (constraint, button, text_label, text_fields, vgs, hgs, ge) in (&constraints, (&buttons).maybe(), (&text_labels).maybe(), (&mut text_fields).maybe(), (&v_group_start).maybe(), (&h_group_start).maybe(), (&group_end).maybe()).join() {
            let size = Vector2::new(constraint.x_size.as_pixels(container.last().unwrap().1.x), constraint.y_size.as_pixels(container.last().unwrap().1.y));
            let pos = Vector2::new(constraint.x_pos.as_pixels(size.x, container.last().unwrap().1.x) + container.last().unwrap().0.x, constraint.y_pos.as_pixels(size.y, container.last().unwrap().1.y) + container.last().unwrap().0.y);

            match vgs {
                Some(_) => {
                    container.push((pos, size, Vector2::new(0, 0), true));
                    continue;
                },
                None => ()
            }

            match hgs {
                Some(_) => {
                    container.push((pos, size, Vector2::new(0, 0), false));
                    continue;
                },
                None => ()
            }

            match ge {
                Some(_) => {
                    container.pop();
                    continue;
                },
                None => ()
            }
            
            container.last_mut().unwrap().2 += pos;

            match button {
                Some(button) => {
                    let mut pressed = false;
                    for event in input_events.iter() {
                        match event {
                            InputEvent::MouseDown(m) if container.last_mut().unwrap().2.x < m.x && m.x < container.last_mut().unwrap().2.x + size.x && container.last_mut().unwrap().2.y < m.y && m.y < container.last_mut().unwrap().2.y + size.y => pressed = true,
                            _ => ()
                        }
                    }

                    if pressed {
                        events.push(UIEvent::ButtonPressed { id: button.element_name.clone() });
                        self.renderer.render_ss(&button.sprite_pressed, container.last_mut().unwrap().2.convert(), size);
                    } else {
                        self.renderer.render_ss(&button.sprite, container.last_mut().unwrap().2.convert(), size);
                    }
                },
                None => ()
            }

            match text_label {
                Some(text_label) => {
                    self.renderer.write_ss(&text_label.text, &text_label.font, container.last_mut().unwrap().2.convert(), size);
                },
                None => ()
            }

            match text_fields {
                Some(text_field) => {
                    self.renderer.render_ss(&text_field.background, container.last_mut().unwrap().2.convert(), size);
                    if self.renderer.write_ss(&text_field.text, &text_field.font, container.last_mut().unwrap().2.convert(), size) {
                        text_field.text.pop();
                    }

                    for event in input_events.iter() {
                        match event {
                            InputEvent::MouseDown(m) if container.last_mut().unwrap().2.x < m.x && m.x < container.last_mut().unwrap().2.x + size.x && container.last_mut().unwrap().2.y < m.y && m.y < container.last_mut().unwrap().2.y + size.y => {
                                text_field.captured = true;
                            },
                            InputEvent::MouseDown(_) => {
                                text_field.captured = false;
                            },
                            InputEvent::KeyDown(k) if text_field.captured => {
                                match k {
                                    Key::Backspace => { text_field.text.pop(); },
                                    _ => { k.to_char(keys.pressed(Key::Shift)).map(|c| text_field.text.push(c)); }
                                }

                                events.push(UIEvent::TextChanged { id: text_field.element_name.clone(), text: text_field.text.clone() });
                            }
                            _ => ()
                        }
                    }
                },
                None => ()
            }

            if container.last_mut().unwrap().3 {
                container.last_mut().unwrap().2.y += size.y;
                container.last_mut().unwrap().2.x -= pos.x;
            } else {
                container.last_mut().unwrap().2.x += size.x;
                container.last_mut().unwrap().2.y -= pos.y;
            }
        }

        self.renderer.post();
    }
}

impl<'a> RenderSys<'a> {
    pub fn new(render: SDLRenderImpl<'a>) -> Self {
        Self {
            renderer: render
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct SpriteCom {
    pub name: String,
    pub dim: Vector
}

#[allow(dead_code)]
impl SpriteCom {
    pub fn new(name: &str, dim: Vector) -> Self {
        Self {
            name: name.to_string(),
            dim: dim
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
#[storage(DenseVecStorage)]
pub struct TextCom {
    pub text: String,
    pub font: String,
    pub dim: Vector
}

#[allow(dead_code)]
impl TextCom {
    pub fn new(text: &str, font: &str, dim: Vector) -> Self {
        Self {
            text: text.to_string(),
            font: font.to_string(),
            dim: dim
        }
    }
}

event_queue! {
    UIEventQueue: pub enum UIEvent {
        ButtonPressed { id: String },
        TextChanged { id: String, text: String }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct TextUICom {
    pub text: String,
    pub font: String
}

impl TextUICom {
    pub fn new(text: &str, font: &str) -> Self {
        Self {
            text: text.to_string(),
            font: font.to_string()
        }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct ButtonUICom {
    pub sprite: String,
    pub sprite_pressed: String,
    pub element_name: String
}

impl ButtonUICom {
    pub fn new(sprite: &str, sprite_pressed: &str, element_name: &str) -> Self {
        Self {
            sprite: sprite.to_string(),
            sprite_pressed: sprite_pressed.to_string(),
            element_name: element_name.to_string()
        }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct TextFieldUICom {
    pub background: String,
    pub text: String,
    pub font: String,
    pub element_name: String,
    pub captured: bool
}

impl TextFieldUICom {
    pub fn new(background: &str, text: &str, font: &str, element_name: &str) -> Self {
        Self {
            background: background.to_string(),
            text: text.to_string(),
            font: font.to_string(),
            element_name: element_name.to_string(),
            captured: false
        }
    }
}

#[derive(Component, Debug, DefaultConstructor)]
#[storage(DenseVecStorage)]
pub struct ConstraintCom {
    x_pos: PositionConstraint,
    y_pos: PositionConstraint,
    x_size: SizeConstraint,
    y_size: SizeConstraint
}

#[derive(Component, Debug, DefaultConstructor)]
#[storage(DenseVecStorage)]
pub struct StartVerticalGroupCom;

#[derive(Component, Debug, DefaultConstructor)]
#[storage(DenseVecStorage)]
pub struct StartHorizontalGroupCom;

#[derive(Component, Debug, DefaultConstructor)]
#[storage(DenseVecStorage)]
pub struct EndGroupCom;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum PositionConstraint {
    Start,
    Center,
    End,
    StartPixelOffset(u32)
}

impl PositionConstraint {
    fn as_pixels(&self, size: u32, container_size: u32) -> u32 {
        match self {
            PositionConstraint::Start => 0,
            PositionConstraint::Center => (container_size / 2) - (size / 2),
            PositionConstraint::End => container_size - size,
            PositionConstraint::StartPixelOffset(offset) => offset.clone()
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum SizeConstraint {
    Proportion(f32),
    Pixels(u32),
    NegativePixels(u32),
    Fill
}

impl SizeConstraint {
    fn as_pixels(&self, container_size: u32) -> u32 {
        match self {
            SizeConstraint::Proportion(proportion) => (proportion * container_size as f32).floor() as u32,
            SizeConstraint::Pixels(pixels) => pixels.clone(),
            SizeConstraint::NegativePixels(pixels) => container_size - pixels.clone(),
            SizeConstraint::Fill => container_size
        }
    }
}

pub fn register(world: &mut World) {
    world.insert(CameraRes::default());
    world.register::<SpriteCom>();
    world.register::<TextCom>();
    world.register::<ConstraintCom>();
    world.register::<TextUICom>();
    world.register::<ButtonUICom>();
    world.register::<TextFieldUICom>();
    world.register::<StartVerticalGroupCom>();
    world.register::<StartHorizontalGroupCom>();
    world.register::<EndGroupCom>();
}
