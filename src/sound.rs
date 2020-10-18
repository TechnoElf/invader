pub mod imp;

use specs::*;

use invader_macro::DefaultConstructor;
use crate::sound::imp::SoundImp;

event_queue! {
    SoundRequestQueue: pub enum SoundRequest {
        ChangeMusic(MusicID, LayerID)
    }
}

#[derive(DefaultConstructor)]
pub struct SoundSys {
    imp: SoundImp<'static>,
}

impl<'a> System<'a> for SoundSys {
    type SystemData = Write<'a, SoundRequestQueue>;

    fn run(&mut self, data: Self::SystemData) {
        let mut requests = data;

        for request in requests.iter() {
            match request {
                SoundRequest::ChangeMusic(music, layer) => self.imp.play_music(music.clone(), layer.clone())
            }
        }
        requests.clear();
    }
}

pub type SoundID = u16;
pub type MusicID = u16;
pub type LayerID = u8;
