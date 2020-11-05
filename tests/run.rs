use invader::InvaderBuilder;
use invader::misc::persist::SpriteSheet;

#[test]
fn run() {
    let engine = InvaderBuilder::new()
        .set_stage("tests/assets/stage.mst")
        .add_sprite_sheet("tests/assets/sprite_sheet.mss")
        .build();
    engine.run();
}
