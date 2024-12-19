use bevy::{
    asset::{Assets, RenderAssetUsages},
    log,
    math::{Vec2, Vec3, bounding::BoundingCircle},
    prelude::{Commands, Mesh, Mesh2d, Res, ResMut, Transform},
    sprite::{ColorMaterial, MeshMaterial2d},
    utils::default,
};

use crate::{
    components_and_resources::{BoundingCircleComponent, Sensor},
    config::{BODY_RADIUS, BOTTOM_LEFT_X, BOTTOM_LEFT_Y, SENSOR_RADIUS, TOP_RIGHT_X, TOP_RIGHT_Y},
    graphics::colors_and_handles::{AvailableColor, ColorHandles},
};

pub fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    color_handles: Res<ColorHandles>,
) {
    let mut count = 0;
    for x in BOTTOM_LEFT_X..TOP_RIGHT_X {
        for y in BOTTOM_LEFT_Y..TOP_RIGHT_Y {
            spawn_body(
                x as f32,
                y as f32,
                &mut commands,
                &mut meshes,
                &color_handles,
            );
            spawn_sensor(
                x as f32,
                y as f32,
                SENSOR_RADIUS,
                &mut commands,
                &mut meshes,
                &color_handles,
            );
            count += 2;
        }
    }
    log::info!("total of {} entities spawned", count)
}

fn spawn_body(
    x: f32,
    y: f32,
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    color_handles: &Res<ColorHandles>,
) {
    commands.spawn((
        create_circle_outline_components(
            BODY_RADIUS,
            AvailableColor::BLUE,
            color_handles,
            &mut meshes,
        ),
        Transform {
            translation: Vec3::new(x, y, 0.0),
            ..default()
        },
        BoundingCircleComponent(BoundingCircle::new(Vec2::new(x, y), BODY_RADIUS)),
    ));
}

fn spawn_sensor(
    x: f32,
    y: f32,
    radius: f32,
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    color_handles: &Res<ColorHandles>,
) {
    commands.spawn((
        Sensor {},
        create_circle_outline_components(
            SENSOR_RADIUS,
            AvailableColor::GREEN,
            color_handles,
            &mut meshes,
        ),
        Transform {
            translation: Vec3::new(x, y, 0.0),
            ..default()
        },
        BoundingCircleComponent(BoundingCircle::new(Vec2::new(x, y), radius)),
    ));
}

fn create_circle_outline_components(
    radius: f32,
    outline_color: AvailableColor,
    color_handles: &Res<ColorHandles>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> (Mesh2d, MeshMaterial2d<ColorMaterial>) {
    let color = color_handles.handles.get(&outline_color).unwrap().clone();

    // Create a path for the circle outline
    let mut path = Vec::new();
    let segments = 32; // Number of segments to approximate the circle
    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let point = Vec2::new(radius * angle.cos(), radius * angle.sin());
        path.push(point);
    }

    // Create the line strip mesh
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::LineStrip,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        path.iter()
            .map(|p| [p.x, p.y, 0.0])
            .collect::<Vec<[f32; 3]>>(),
    );
    return (Mesh2d(meshes.add(mesh).into()), MeshMaterial2d(color));
}
