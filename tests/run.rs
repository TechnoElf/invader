use invader::InvaderBuilder;

#[test]
fn run() {
    let engine = InvaderBuilder::new()
        .set_stage("tests/assets/stage.mst")
        .add_sprite("r", "tests/assets/32x32-w-r.png")
        .add_sprite("g", "tests/assets/32x32-w-g.png")
        .add_sprite("b", "tests/assets/32x32-w-b.png")
        .build();
    engine.run();
}
