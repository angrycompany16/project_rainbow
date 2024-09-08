use bevy::{math::{vec2, vec3}, prelude::*, render::{camera::RenderTarget, render_resource::*, view::RenderLayers}};

// NOTE:
// Low res layer is 0
// High res layer is 1

pub struct PixelCameraPlugin {
    pub pixel_scale_factor: u32,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        let canvas_size = Extent3d {
            width: self.screen_width / self.pixel_scale_factor + 2,
            height: self.screen_height / self.pixel_scale_factor + 2,
            ..default()
        };
        
        let mut canvas = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: canvas_size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };

        canvas.resize(canvas_size);

        let mut images = app.world_mut().get_resource_mut::<Assets<Image>>().expect("Could not find the image asset loader");
        let image_handle = images.add(canvas);

        // Spawn low-res camera
        app
            .insert_resource(Msaa::Off)
            .add_systems(Update, update_low_res_camera)
            .world_mut().spawn(Camera2dBundle {
                    camera: Camera {
                        order: -1,
                        target: RenderTarget::Image(image_handle.clone()),
                        ..default()
                    },
                    ..default()
                })
                .insert(LowResCamera)
                .insert(RenderLayers::layer(0))
        ;

        // Spawn high-res camera
        app
            .add_systems(Update, move_camera)
            .world_mut().spawn(Camera2dBundle {
                    camera: Camera {
                        clear_color: ClearColorConfig::Custom(Color::linear_rgb(0.05, 0.05, 0.05)),
                        ..default()
                    },
                    ..default()
                })
                .insert(RenderLayers::layer(1))
                .insert(HighResCamera);
    

        app.world_mut().spawn(
            SpriteBundle {
                texture: image_handle,
                transform: Transform {
                    scale: vec3(self.pixel_scale_factor as f32, self.pixel_scale_factor as f32, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(RenderLayers::layer(1))
            .insert(Canvas);

        app.
            insert_resource(PixelScaleFactor(self.pixel_scale_factor))
        ;
    }
}

#[derive(Resource)]
pub struct PixelScaleFactor(u32);

#[derive(Component)]
pub struct LowResCamera;

#[derive(Component)]
pub struct HighResCamera;

#[derive(Component)]
pub struct Canvas;

fn move_camera(
    mut high_res_camera_q: Query<&mut Transform, With<HighResCamera>>,
    
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction: Vec2 = vec2(0.0, 0.0);
    
    if input.pressed(KeyCode::ArrowUp) { direction.y += 1.0; } 
    if input.pressed(KeyCode::ArrowDown) { direction.y -= 1.0; }
    if input.pressed(KeyCode::ArrowRight) { direction.x += 1.0 } 
    if input.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0 }
    
    if direction.length_squared() > 0.0 { direction = direction.normalize() }

    let mut high_res_cam_transform = high_res_camera_q.get_single_mut().expect("More than one high res camera in the scene");
    
    high_res_cam_transform.translation += time.delta_seconds() * direction.extend(0.0) * 100.0;
}

fn update_low_res_camera(
    mut rendering_query: ParamSet<(
        Query<&mut Transform, With<HighResCamera>>,
        Query<&mut Transform, With<LowResCamera>>,
        Query<&mut Transform, With<Canvas>>,
    )>,

    pixel_scale_factor: Res<PixelScaleFactor>,
) {
    let mut high_res_position = Vec3::splat(0.0);
    for high_res_cam_transform in rendering_query.p0().iter() {
        high_res_position = high_res_cam_transform.translation;
    }

    let mut diff_x: f32 = 0.0;
    let mut diff_y: f32 = 0.0;
    for mut canvas_transform in rendering_query.p2().iter_mut() {
        diff_x = high_res_position.x - canvas_transform.translation.x;
        diff_y = high_res_position.y - canvas_transform.translation.y;

        if diff_x.abs() >= pixel_scale_factor.0 as f32 { 
            canvas_transform.translation.x += diff_x; 
        }

        if diff_y.abs() >= pixel_scale_factor.0 as f32 { 
            canvas_transform.translation.y += diff_y; 
        } 
    }

    for mut low_res_camera_transform in rendering_query.p1().iter_mut() {
        if diff_x >= pixel_scale_factor.0 as f32 { 
            low_res_camera_transform.translation.x += (diff_x / (pixel_scale_factor.0 as f32)).floor(); 
        } else if diff_x <= -(pixel_scale_factor.0 as f32) {
            low_res_camera_transform.translation.x += (diff_x / (pixel_scale_factor.0 as f32)).ceil(); 
        }

        if diff_y >= pixel_scale_factor.0 as f32 {
            low_res_camera_transform.translation.y += (diff_y / (pixel_scale_factor.0 as f32)).floor();
        } else if diff_y <= -(pixel_scale_factor.0 as f32) {
            low_res_camera_transform.translation.y += (diff_y / (pixel_scale_factor.0 as f32)).ceil(); 
        } 
    }
}