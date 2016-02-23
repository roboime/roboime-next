enum Action {
    Move(f32, f32),
    Kick(f32, f32),
    Pass(u8),
}

fn main() {
    let a = Move(3.0, 4i as f32);
    match a {
        Move(_px, _py) => (),
        Kick(_px, _py) => (),
        Pass(_ri) => (),
    }
}
