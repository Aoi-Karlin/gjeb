// Every frame, before the world is drawn: the lightmap gamma this mod asks for, if any.
// The game fills the renderer's gamma from the brightness slider; writing it here again is what
// lets the player go past that slider's 100%.
fn gjeb_frame(game: &mut crate::game::Game<'_>) {
    let Some(gamma) = gjeb::override_gamma() else {
        return;
    };
    if let Some(renderer) = game.renderer.as_mut() {
        renderer.gamma = gamma;
    }
}
