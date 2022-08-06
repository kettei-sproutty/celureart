use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
pub struct Glc {
    #[allow(dead_code)]
    app: App,
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(GravityScale(0.3))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

#[wasm_bindgen]
impl Glc {
    pub fn new(canvas_id: &str) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

        let mut app = App::new();

        app.insert_resource(WindowDescriptor {
            #[cfg(target_arch = "wasm32")]
            canvas: Some(format!("#{canvas_id}")),
            decorations: false,
            width: canvas.width() as f32,
            height: canvas.height() as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .update();

        Self { app }
    }
}
