use bevy::{
    asset::{Assets, Handle},
    prelude::{Color, FromWorld, Resource, World},
    sprite::ColorMaterial,
    utils::hashbrown::HashMap,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AvailableColor {
    PADDLE,
    BACKGROUND,
    BALL,
    BRICK,
    WALL,
    TEXT,
    SCORE,
    DARKGREEN,
    GREEN,
    LIGHTGREEN,
    DARKBLUE,
    BLUE,
    LIGHTBLUE,
    DARKRED,
    RED,
    LIGHTRED,
    DARKYELLOW,
    YELLOW,
    LIGHTYELLOW,
    BLACK,
    BARRIER,
}
#[derive(Resource)]
pub struct ColorHandles {
    pub handles: HashMap<AvailableColor, Handle<ColorMaterial>>,
    pub colors: HashMap<AvailableColor, Color>,
}

impl FromWorld for ColorHandles {
    fn from_world(world: &mut World) -> Self {
        let mut colors = HashMap::new();
        colors.insert(AvailableColor::PADDLE, Color::srgb(0.3, 0.3, 0.7));
        colors.insert(AvailableColor::BACKGROUND, Color::srgb(0.9, 0.9, 0.9));
        colors.insert(AvailableColor::BALL, Color::srgb(1.0, 0.5, 0.5));
        colors.insert(AvailableColor::BRICK, Color::srgb(0.5, 0.5, 1.0));
        colors.insert(AvailableColor::WALL, Color::srgb(0.8, 0.8, 0.8));
        colors.insert(AvailableColor::TEXT, Color::srgb(0.5, 0.5, 1.0));
        colors.insert(AvailableColor::SCORE, Color::srgb(1.0, 0.5, 0.5));
        colors.insert(AvailableColor::DARKGREEN, Color::srgb(0.0, 0.5, 0.0));
        colors.insert(AvailableColor::GREEN, Color::srgb(0.0, 1.0, 0.0));
        colors.insert(AvailableColor::LIGHTGREEN, Color::srgb(0.5, 1.0, 0.5));
        colors.insert(AvailableColor::DARKBLUE, Color::srgb(0.0, 0.0, 0.5));
        colors.insert(AvailableColor::BLUE, Color::srgb(0.0, 0.0, 1.0));
        colors.insert(AvailableColor::LIGHTBLUE, Color::srgb(0.5, 0.5, 1.0));
        colors.insert(AvailableColor::DARKRED, Color::srgb(0.5, 0.0, 0.0));
        colors.insert(AvailableColor::RED, Color::srgb(1.0, 0.0, 0.0));
        colors.insert(AvailableColor::LIGHTRED, Color::srgb(1.0, 0.5, 0.5));
        colors.insert(AvailableColor::DARKYELLOW, Color::srgb(0.5, 0.5, 0.0));
        colors.insert(AvailableColor::YELLOW, Color::srgb(1.0, 1.0, 0.0));
        colors.insert(AvailableColor::LIGHTYELLOW, Color::srgb(1.0, 1.0, 0.5));
        colors.insert(AvailableColor::BLACK, Color::srgb(0.0, 0.0, 0.0));
        colors.insert(AvailableColor::BARRIER, Color::srgba(0.1, 0.1, 0.1, 1.0));
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        let mut handles = HashMap::new();
        for (color, color_value) in colors.iter() {
            let handle = materials.add(*color_value);
            handles.insert(*color, handle);
        }
        Self { handles, colors }
    }
}
