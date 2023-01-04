#[derive(Bundle)]
struct PlayerBundle {
    xp: Xp,
    name: Name,
    health: Health,
    _p: Player,
    // can include bundle
    #[bundle]
    sprite: SpriteSheetBundle,
}
