use super::*;

#[derive(Resource, Default)]
pub(crate) struct UTextLabelAtlasCache {
    pub(super) pages_by_font: HashMap<UTextLabelAtlasKey, Vec<UTextLabelAtlasPage>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct UTextLabelAtlasKey {
    pub(super) font_data_id: u64,
    pub(super) font_size_bits: u32,
    pub(super) index: u32,
    pub(super) variations_hash: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct UTextLabelGlyphKey {
    pub(super) glyph_id: u16,
    pub(super) x_bin_bits: u32,
    pub(super) y_bin_bits: u32,
}

pub(super) struct UTextLabelAtlasPage {
    pub(super) texture: Handle<Image>,
    pub(super) texture_atlas: Handle<TextureAtlasLayout>,
    pub(super) builder: DynamicTextureAtlasBuilder,
    pub(super) glyphs: HashMap<UTextLabelGlyphKey, UTextLabelGlyphAtlasInfo>,
}

#[derive(Clone)]
pub(super) struct UTextLabelGlyphAtlasInfo {
    pub(super) texture: Handle<Image>,
    pub(super) texture_atlas: Handle<TextureAtlasLayout>,
    pub(super) glyph_index: usize,
    pub(super) offset: IVec2,
}

pub(super) fn extract_mask_alpha(data: &[u8], width: usize, height: usize) -> Vec<u8> {
    let pixel_count = width * height;
    match data.len() {
        len if len == pixel_count => data.to_vec(),
        len if len == pixel_count * 4 => data.chunks_exact(4).map(|px| px[3]).collect(),
        len if len == pixel_count * 3 => data
            .chunks_exact(3)
            .map(|px| ((u16::from(px[0]) + u16::from(px[1]) + u16::from(px[2])) / 3) as u8)
            .collect(),
        _ => vec![0; pixel_count],
    }
}

pub(super) fn create_text_atlas_page(
    images: &mut Assets<Image>,
    texture_atlases: &mut Assets<TextureAtlasLayout>,
    minimum_size: u32,
) -> UTextLabelAtlasPage {
    let size = minimum_size.max(TEXT_SDF_ATLAS_SIZE).next_power_of_two();
    let mut atlas_image = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );
    atlas_image.sampler = ImageSampler::linear();
    let texture = images.add(atlas_image);
    let texture_atlas = texture_atlases.add(TextureAtlasLayout::new_empty(UVec2::splat(size)));

    UTextLabelAtlasPage {
        texture,
        texture_atlas,
        builder: DynamicTextureAtlasBuilder::new(UVec2::splat(size), 2),
        glyphs: HashMap::default(),
    }
}
