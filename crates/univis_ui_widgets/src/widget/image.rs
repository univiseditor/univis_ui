use bevy::prelude::*;
use univis_ui_engine::layout::geometry::UVal;
use univis_ui_engine::layout::image::UImage;
use univis_ui_engine::layout::univis_node::UNode;

fn image_uses_native_intrinsic_size(value: UVal) -> bool {
    matches!(
        value,
        UVal::Auto | UVal::Content | UVal::MinContent | UVal::MaxContent
    )
}

fn resolve_image_dimension(value: UVal, native_axis: f32) -> UVal {
    if image_uses_native_intrinsic_size(value) && native_axis > 0.0 {
        UVal::Px(native_axis)
    } else {
        value
    }
}

pub fn sync_image_geometry(
    mut query: Query<(&UImage, &mut UNode)>,
    // Read image assets to resolve native texture dimensions when needed.
    images: Res<Assets<Image>>,
) {
    for (ui_image, mut node) in query.iter_mut() {
        // 1. Sync the optional corner radius.
        if let Some(r) = ui_image.radius {
            if node.border_radius != r {
                node.border_radius = r;
            }
        }

        // 2. Resolve intrinsic-size modes from the native texture size.
        let needs_native_size = image_uses_native_intrinsic_size(ui_image.width)
            || image_uses_native_intrinsic_size(ui_image.height);

        let mut native_size = Vec2::ZERO;
        if needs_native_size {
            if let Some(img) = images.get(&ui_image.texture) {
                // Native texture size is treated as logical UI units when projected into layout.
                let size = img.size_f32(); // Bevy exposes the image size as `Vec2`.
                native_size = size;
            }
        }

        // 3. Apply width.
        let target_width = resolve_image_dimension(ui_image.width, native_size.x);

        if node.width != target_width {
            node.width = target_width;
        }

        // 4. Apply height.
        let target_height = resolve_image_dimension(ui_image.height, native_size.y);

        if node.height != target_height {
            node.height = target_height;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::RenderAssetUsages;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

    fn test_image(width: u32, height: u32) -> Image {
        Image::new_fill(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[255, 255, 255, 255],
            TextureFormat::Rgba8Unorm,
            RenderAssetUsages::default(),
        )
    }

    #[test]
    fn intrinsic_image_modes_resolve_to_native_texture_size() {
        let mut app = App::new();
        app.init_resource::<Assets<Image>>();
        app.add_systems(Update, sync_image_geometry);

        let handle = app
            .world_mut()
            .resource_mut::<Assets<Image>>()
            .add(test_image(64, 24));
        let entity = app.world_mut().spawn(UImage {
            texture: handle,
            width: UVal::MaxContent,
            height: UVal::MinContent,
            ..default()
        });
        let entity = entity.id();

        app.update();

        let node = app
            .world()
            .entity(entity)
            .get::<UNode>()
            .expect("image should have a UNode");
        assert_eq!(node.width, UVal::Px(64.0));
        assert_eq!(node.height, UVal::Px(24.0));
    }

    #[test]
    fn image_geometry_retries_after_texture_becomes_available() {
        let mut app = App::new();
        app.init_resource::<Assets<Image>>();
        app.add_systems(Update, sync_image_geometry);

        let handle = app.world().resource::<Assets<Image>>().reserve_handle();
        let entity = app.world_mut().spawn(UImage {
            texture: handle.clone(),
            width: UVal::Auto,
            height: UVal::Content,
            ..default()
        });
        let entity = entity.id();

        app.update();

        {
            let node = app
                .world()
                .entity(entity)
                .get::<UNode>()
                .expect("image should have a UNode");
            assert_eq!(node.width, UVal::Auto);
            assert_eq!(node.height, UVal::Content);
        }

        let _ = app
            .world_mut()
            .resource_mut::<Assets<Image>>()
            .insert(handle.id(), test_image(32, 18));

        app.update();

        let node = app
            .world()
            .entity(entity)
            .get::<UNode>()
            .expect("image should have a UNode");
        assert_eq!(node.width, UVal::Px(32.0));
        assert_eq!(node.height, UVal::Px(18.0));
    }
}
