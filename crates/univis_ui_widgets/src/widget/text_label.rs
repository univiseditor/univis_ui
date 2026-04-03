//! Text rendering widgets for Univis UI.
//!
//! [`UTextLabel`] measures text through Bevy's text pipeline, then renders the
//! final result through Univis SDF materials so the same widget can stay sharp
//! in screen and world roots.

use crate::internal_prelude::*;
use bevy::asset::{Asset, AssetEvent, AssetId, Assets, RenderAssetUsages, embedded_asset};
use bevy::color::LinearRgba;
use bevy::ecs::relationship::Relationship;
use bevy::image::{DynamicTextureAtlasBuilder, ImageSampler, TextureAtlasLayout};
use bevy::mesh::{Indices, Mesh, Mesh2d, PrimitiveTopology};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, Extent3d, TextureDimension, TextureFormat};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin, MeshMaterial2d};
use bevy::text::{
    ComputedTextBlock, CosmicFontSystem, FontHinting, LineBreak, LineHeight, TextBounds, TextFont,
    TextLayout, TextPipeline,
};
use std::collections::{HashMap, HashSet};
use unicode_bidi::BidiInfo;
use unicode_segmentation::UnicodeSegmentation;
use univis_ui_engine::internal::IntrinsicSize;
use univis_ui_engine::layout::core::layout_cache::LayoutCache;

const DEFAULT_TEXT_RENDER_SCALE: f32 = 8.0;
const DEFAULT_TEXT_EDGE_SOFTNESS: f32 = 0.75;
const TEXT_SDF_PADDING: u32 = 12;
const TEXT_SDF_ATLAS_SIZE: u32 = 1024;
const DISTANCE_FIELD_INF: f32 = 1.0e20;
const DEFAULT_ELLIPSIS: &str = "...";
const LTR_ISOLATE_START: char = '\u{2066}';
const RTL_ISOLATE_START: char = '\u{2067}';
const ISOLATE_END: char = '\u{2069}';

/// Overflow policy used by [`UTextLabel`].
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Default, Debug, Clone, PartialEq)]
pub enum UTextOverflow {
    /// Render the full text even if it extends past the available bounds.
    Visible,
    /// Clip the rendered text to the available bounds.
    Clip,
    /// Replace overflowing tail content with an ellipsis when possible.
    #[default]
    Ellipsis,
}

/// Controls which side of the text is shortened when ellipsis is applied.
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Default, Debug, Clone, PartialEq)]
pub enum UTextTruncateSide {
    /// Use the widget's default behavior.
    #[default]
    Auto,
    /// Remove leading content.
    Start,
    /// Remove trailing content.
    End,
    /// Remove inner content and keep both edges.
    Middle,
}

/// A text-rendering widget backed by Bevy text measurement and Univis SDF rendering.
///
/// `UTextLabel` measures text using Bevy's text pipeline, then renders the
/// result through a mesh/material path that stays sharp in screen and world roots.
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use univis_ui_widgets::prelude::*;
///
/// fn spawn_label(commands: &mut Commands) {
///     commands.spawn(UTextLabel {
///         text: "Hello Univis".into(),
///         font_size: 28.0,
///         color: Color::WHITE,
///         ..default()
///     });
/// }
/// ```
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout, Visibility, ComputedTextBlock, UTextLabelLayoutCache)]
pub struct UTextLabel {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub font: Handle<Font>,
    pub justify: Justify,
    pub linebreak: LineBreak,
    /// If `true`, the label updates its host [`UNode`] width and height from measured text.
    pub autosize: bool,
    /// SDF raster scale used to increase glyph sharpness while keeping the same final size.
    pub render_scale: f32,
    /// What happens when the available bounds are too small for the full text.
    pub overflow: UTextOverflow,
    /// Which side is shortened when ellipsis is active.
    pub truncate_side: UTextTruncateSide,
    /// Maximum line count before clipping or ellipsis is applied.
    pub max_lines: Option<usize>,
}

/// Cached measurement output for [`UTextLabel`].
///
/// This is maintained by the text systems and is mainly useful for advanced
/// tooling or diagnostics.
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct UTextLabelLayoutCache {
    pub measured_size: Vec2,
    pub min_content_size: Vec2,
    pub max_content_size: Vec2,
    pub displayed_text: String,
    pub line_count: usize,
    pub overflowed: bool,
    pub dirty: bool,
}

/// Internal marker for generated text-render child entities.
#[derive(Component)]
#[doc(hidden)]
pub struct TextChildMarker;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TextRenderPageMarker {
    texture_id: AssetId<Image>,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct UTextLabelSdfMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(0)]
    clip_radius: Vec4,
    #[uniform(0)]
    clip_center: Vec2,
    #[uniform(0)]
    clip_size: Vec2,
    #[uniform(0)]
    edge_softness: f32,
    #[uniform(0)]
    use_clip: u32,
    #[uniform(0)]
    _pad: Vec2,
    #[texture(1)]
    #[sampler(2)]
    texture: Handle<Image>,
}

impl Material2d for UTextLabelSdfMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://univis_ui_widgets/widget/shaders/text_label_sdf.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Resource, Default)]
struct UTextLabelAtlasCache {
    pages_by_font: HashMap<UTextLabelAtlasKey, Vec<UTextLabelAtlasPage>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct UTextLabelAtlasKey {
    font_id: AssetId<Font>,
    font_size_bits: u32,
    font_weight: u16,
    flags_bits: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct UTextLabelGlyphKey {
    glyph_id: u16,
    x_bin_bits: u32,
    y_bin_bits: u32,
}

struct UTextLabelAtlasPage {
    texture: Handle<Image>,
    texture_atlas: Handle<TextureAtlasLayout>,
    builder: DynamicTextureAtlasBuilder,
    glyphs: HashMap<UTextLabelGlyphKey, UTextLabelGlyphAtlasInfo>,
}

#[derive(Clone)]
struct UTextLabelGlyphAtlasInfo {
    texture: Handle<Image>,
    texture_atlas: Handle<TextureAtlasLayout>,
    glyph_index: usize,
    offset: IVec2,
}

#[derive(Clone, Debug)]
struct TextGlyphQuad {
    center: Vec2,
    size: Vec2,
    uv_min: Vec2,
    uv_max: Vec2,
}

#[derive(Clone)]
struct TextPageBatch {
    texture: Handle<Image>,
    quads: Vec<TextGlyphQuad>,
}

#[derive(Clone, Copy, Debug)]
struct LocalClipRect {
    min: Vec2,
    max: Vec2,
}

#[derive(Clone, Copy, Debug, Default)]
struct MaterialClipInfo {
    center: Vec2,
    size: Vec2,
    radius: Vec4,
    use_clip: u32,
}

#[derive(Clone, Debug, Default)]
struct MeasuredTextInfo {
    size: Vec2,
    min_content_size: Vec2,
    max_content_size: Vec2,
    line_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TextBaseDirection {
    Ltr,
    Rtl,
}

impl Default for UTextLabel {
    fn default() -> Self {
        Self {
            text: "Label".to_string(),
            font_size: 16.0,
            color: Color::WHITE,
            font: Handle::default(),
            justify: Justify::Left,
            linebreak: LineBreak::NoWrap,
            autosize: true,
            render_scale: DEFAULT_TEXT_RENDER_SCALE,
            overflow: UTextOverflow::Ellipsis,
            truncate_side: UTextTruncateSide::Auto,
            max_lines: None,
        }
    }
}

impl UTextLabel {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            ..default()
        }
    }

    pub fn with_render_scale(mut self, render_scale: f32) -> Self {
        self.render_scale = render_scale;
        self
    }

    pub fn with_overflow(mut self, overflow: UTextOverflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn with_truncate_side(mut self, truncate_side: UTextTruncateSide) -> Self {
        self.truncate_side = truncate_side;
        self
    }

    pub fn with_max_lines(mut self, max_lines: usize) -> Self {
        self.max_lines = Some(max_lines);
        self
    }
}

fn resolved_render_scale(render_scale: f32) -> f32 {
    if render_scale.is_finite() && render_scale >= 1.0 {
        render_scale
    } else {
        1.0
    }
}

fn label_text_layout(label: &UTextLabel) -> TextLayout {
    TextLayout {
        justify: label.justify,
        linebreak: label.linebreak,
    }
}

fn label_text_font(label: &UTextLabel) -> TextFont {
    TextFont {
        font: label.font.clone(),
        font_size: label.font_size,
        ..default()
    }
}

fn constrained_node_dimension(spec: &UVal, computed: f32, padding: f32) -> Option<f32> {
    match spec {
        UVal::Px(value) => Some((*value - padding).max(0.0)),
        _ if computed > 0.0 => Some((computed - padding).max(0.0)),
        _ => None,
    }
}

fn node_uses_intrinsic_dimension(spec: &UVal) -> bool {
    spec.uses_intrinsic_measurement()
}

fn parent_label_bounds(
    entity: Entity,
    label: &UTextLabel,
    node: &UNode,
    parents_query: &Query<&ChildOf>,
    parent_query: &Query<(&UNode, &ComputedSize)>,
) -> TextBounds {
    if label.overflow == UTextOverflow::Visible {
        return TextBounds {
            width: None,
            height: None,
        };
    }

    let Ok(parent) = parents_query.get(entity) else {
        return TextBounds {
            width: None,
            height: None,
        };
    };
    let Ok((parent_node, parent_size)) = parent_query.get(parent.get()) else {
        return TextBounds {
            width: None,
            height: None,
        };
    };

    let width = if node_uses_intrinsic_dimension(&parent_node.width) {
        None
    } else {
        Some(
            (parent_size.width - parent_node.padding.width_sum() - node.margin.width_sum())
                .max(0.0),
        )
    };

    let height = if node_uses_intrinsic_dimension(&parent_node.height) {
        None
    } else {
        Some(
            (parent_size.height - parent_node.padding.height_sum() - node.margin.height_sum())
                .max(0.0),
        )
    };

    TextBounds { width, height }
}

fn parent_label_bounds_from_ids(
    parent_entity: Option<Entity>,
    overflow: UTextOverflow,
    margin: USides,
    parent_query: &Query<(&UNode, &ComputedSize)>,
) -> TextBounds {
    if overflow == UTextOverflow::Visible {
        return TextBounds {
            width: None,
            height: None,
        };
    }

    let Some(parent_entity) = parent_entity else {
        return TextBounds {
            width: None,
            height: None,
        };
    };
    let Ok((parent_node, parent_size)) = parent_query.get(parent_entity) else {
        return TextBounds {
            width: None,
            height: None,
        };
    };

    let width = if node_uses_intrinsic_dimension(&parent_node.width) {
        None
    } else {
        Some((parent_size.width - parent_node.padding.width_sum() - margin.width_sum()).max(0.0))
    };

    let height = if node_uses_intrinsic_dimension(&parent_node.height) {
        None
    } else {
        Some((parent_size.height - parent_node.padding.height_sum() - margin.height_sum()).max(0.0))
    };

    TextBounds { width, height }
}

fn clamp_outer_size_to_bounds(outer_size: Vec2, bounds: TextBounds) -> Vec2 {
    Vec2::new(
        bounds
            .width
            .map_or(outer_size.x, |width| outer_size.x.min(width)),
        bounds
            .height
            .map_or(outer_size.y, |height| outer_size.y.min(height)),
    )
}

fn text_horizontal_offset(
    label: &UTextLabel,
    node: &UNode,
    computed_size: &ComputedSize,
    text_size: Vec2,
) -> f32 {
    let content_width = (computed_size.width - node.padding.width_sum()).max(0.0);

    match label.justify {
        Justify::Left => (-content_width * 0.5) + (text_size.x * 0.5),
        Justify::Center | Justify::Justified => 0.0,
        Justify::Right => (content_width * 0.5) - (text_size.x * 0.5),
    }
}

fn measured_text_outer_size(node: &UNode, layout_cache: &UTextLabelLayoutCache) -> Vec2 {
    Vec2::new(
        layout_cache.measured_size.x.max(0.0) + node.padding.width_sum(),
        layout_cache.measured_size.y.max(0.0) + node.padding.height_sum(),
    )
}

fn measured_text_outer_bounds(node: &UNode, layout_cache: &UTextLabelLayoutCache) -> (Vec2, Vec2) {
    (
        Vec2::new(
            layout_cache.min_content_size.x.max(0.0) + node.padding.width_sum(),
            layout_cache.min_content_size.y.max(0.0) + node.padding.height_sum(),
        ),
        Vec2::new(
            layout_cache.max_content_size.x.max(0.0) + node.padding.width_sum(),
            layout_cache.max_content_size.y.max(0.0) + node.padding.height_sum(),
        ),
    )
}

fn desired_text_label_intrinsic_size(
    node: &UNode,
    layout_cache: &UTextLabelLayoutCache,
    current: IntrinsicSize,
) -> IntrinsicSize {
    let (min_outer_size, max_outer_size) = measured_text_outer_bounds(node, layout_cache);

    IntrinsicSize {
        width: if node_uses_intrinsic_dimension(&node.width) {
            match node.width {
                UVal::MinContent => min_outer_size.x,
                _ => max_outer_size.x,
            }
        } else {
            current.width
        },
        height: if node_uses_intrinsic_dimension(&node.height) {
            match node.height {
                UVal::MinContent => min_outer_size.y,
                _ => max_outer_size.y,
            }
        } else {
            current.height
        },
        min_width: if node_uses_intrinsic_dimension(&node.width) {
            min_outer_size.x
        } else {
            current.min_width
        },
        max_width: if node_uses_intrinsic_dimension(&node.width) {
            max_outer_size.x
        } else {
            current.max_width
        },
        min_height: if node_uses_intrinsic_dimension(&node.height) {
            min_outer_size.y
        } else {
            current.min_height
        },
        max_height: if node_uses_intrinsic_dimension(&node.height) {
            max_outer_size.y
        } else {
            current.max_height
        },
    }
}

fn label_measure_bounds(
    label: &UTextLabel,
    node: &UNode,
    computed_size: Option<&ComputedSize>,
    parent_bounds: TextBounds,
) -> TextBounds {
    let computed_width = computed_size.map_or(0.0, |size| size.width);
    let computed_height = computed_size.map_or(0.0, |size| size.height);

    let constrain_width = !label.autosize
        && (label.linebreak != LineBreak::NoWrap
            || label.overflow != UTextOverflow::Visible
            || label.max_lines.is_some());
    let constrain_height = !label.autosize;

    let width = if constrain_width {
        constrained_node_dimension(&node.width, computed_width, node.padding.width_sum())
    } else {
        None
    };

    let height = if constrain_height {
        constrained_node_dimension(&node.height, computed_height, node.padding.height_sum())
    } else {
        None
    };

    TextBounds {
        width: width.or(if label.autosize {
            parent_bounds.width
        } else {
            None
        }),
        height: height.or(if label.autosize {
            parent_bounds.height
        } else {
            None
        }),
    }
}

fn measure_layout_for_text(
    entity: Entity,
    text: &str,
    text_font: &TextFont,
    text_layout: &TextLayout,
    text_color: Color,
    bounds: TextBounds,
    fonts: &Assets<Font>,
    text_pipeline: &mut TextPipeline,
    computed: &mut ComputedTextBlock,
    font_system: &mut CosmicFontSystem,
) -> Result<MeasuredTextInfo, ()> {
    let mut measure = text_pipeline
        .create_text_measure(
            entity,
            fonts,
            std::iter::once((
                entity,
                0,
                text,
                text_font,
                text_color,
                LineHeight::default(),
            )),
            1.0,
            text_layout,
            computed,
            font_system,
            FontHinting::Disabled,
        )
        .map_err(|_| ())?;

    let size = measure.compute_size(bounds, computed, font_system);
    let line_count = computed.buffer().layout_runs().count();

    Ok(MeasuredTextInfo {
        size,
        min_content_size: measure.min,
        max_content_size: measure.max,
        line_count,
    })
}

fn resolve_final_measured_text(
    entity: Entity,
    label: &UTextLabel,
    text_font: &TextFont,
    text_layout: &TextLayout,
    text_color: Color,
    bounds: TextBounds,
    fonts: &Assets<Font>,
    text_pipeline: &mut TextPipeline,
    computed: &mut ComputedTextBlock,
    font_system: &mut CosmicFontSystem,
    measured: MeasuredTextInfo,
) -> Result<(String, MeasuredTextInfo, bool), ()> {
    let mut final_text = label.text.clone();
    let mut final_measured = measured;

    if !text_fits_constraints(&final_measured, bounds, label)
        && label.overflow == UTextOverflow::Ellipsis
        && has_overflow_constraints(bounds, label)
    {
        let (displayed_text, _) = build_ellipsized_text(
            entity,
            label,
            text_font,
            text_layout,
            text_color,
            bounds,
            fonts,
            text_pipeline,
            computed,
            font_system,
        )?;
        final_text = displayed_text;
        final_measured = measure_layout_for_text(
            entity,
            final_text.as_str(),
            text_font,
            text_layout,
            text_color,
            bounds,
            fonts,
            text_pipeline,
            computed,
            font_system,
        )?;
    }

    let overflowed =
        final_text != label.text || !text_fits_constraints(&final_measured, bounds, label);

    Ok((final_text, final_measured, overflowed))
}

fn text_fits_constraints(
    measured: &MeasuredTextInfo,
    bounds: TextBounds,
    label: &UTextLabel,
) -> bool {
    let width_ok = bounds
        .width
        .map_or(true, |width| measured.size.x <= width + 0.5);
    let height_ok = bounds
        .height
        .map_or(true, |height| measured.size.y <= height + 0.5);
    let lines_ok = label
        .max_lines
        .map_or(true, |max_lines| measured.line_count <= max_lines);

    width_ok && height_ok && lines_ok
}

fn has_overflow_constraints(bounds: TextBounds, label: &UTextLabel) -> bool {
    bounds.width.is_some() || bounds.height.is_some() || label.max_lines.is_some()
}

fn text_char_boundaries(text: &str) -> Vec<usize> {
    let mut boundaries = Vec::with_capacity(text.graphemes(true).count() + 1);
    boundaries.push(0);
    for (idx, grapheme) in text.grapheme_indices(true) {
        boundaries.push(idx + grapheme.len());
    }
    boundaries
}

fn base_direction_for_text(text: &str) -> TextBaseDirection {
    let bidi = BidiInfo::new(text, None);
    bidi.paragraphs
        .first()
        .map(|paragraph| {
            if paragraph.level.is_rtl() {
                TextBaseDirection::Rtl
            } else {
                TextBaseDirection::Ltr
            }
        })
        .unwrap_or(TextBaseDirection::Ltr)
}

fn resolve_truncate_side(
    truncate_side: UTextTruncateSide,
    _base_direction: TextBaseDirection,
) -> UTextTruncateSide {
    match truncate_side {
        UTextTruncateSide::Auto => UTextTruncateSide::End,
        side => side,
    }
}

fn isolate_for_direction(segment: &str, base_direction: TextBaseDirection) -> String {
    if segment.is_empty() {
        return String::new();
    }

    let isolate_start = match base_direction {
        TextBaseDirection::Ltr => LTR_ISOLATE_START,
        TextBaseDirection::Rtl => RTL_ISOLATE_START,
    };

    let mut isolated = String::with_capacity(segment.len() + 2);
    isolated.push(isolate_start);
    isolated.push_str(segment);
    isolated.push(ISOLATE_END);
    isolated
}

fn build_truncate_candidate(
    text: &str,
    boundaries: &[usize],
    truncate_side: UTextTruncateSide,
    kept_graphemes: usize,
    base_direction: TextBaseDirection,
) -> String {
    let total_graphemes = boundaries.len().saturating_sub(1);
    if total_graphemes == 0 {
        return DEFAULT_ELLIPSIS.to_string();
    }

    match truncate_side {
        UTextTruncateSide::End | UTextTruncateSide::Auto => {
            let keep = kept_graphemes.min(total_graphemes);
            let prefix = &text[..boundaries[keep]];
            let isolated_prefix = isolate_for_direction(prefix, base_direction);
            let mut candidate =
                String::with_capacity(isolated_prefix.len() + DEFAULT_ELLIPSIS.len());
            candidate.push_str(&isolated_prefix);
            candidate.push_str(DEFAULT_ELLIPSIS);
            candidate
        }
        UTextTruncateSide::Start => {
            let keep = kept_graphemes.min(total_graphemes);
            let suffix = &text[boundaries[total_graphemes - keep]..];
            let isolated_suffix = isolate_for_direction(suffix, base_direction);
            let mut candidate =
                String::with_capacity(DEFAULT_ELLIPSIS.len() + isolated_suffix.len());
            candidate.push_str(DEFAULT_ELLIPSIS);
            candidate.push_str(&isolated_suffix);
            candidate
        }
        UTextTruncateSide::Middle => {
            let keep = kept_graphemes.min(total_graphemes);
            let keep_prefix = keep.div_ceil(2);
            let keep_suffix = keep / 2;
            let prefix = &text[..boundaries[keep_prefix]];
            let suffix = &text[boundaries[total_graphemes - keep_suffix]..];
            let isolated_prefix = isolate_for_direction(prefix, base_direction);
            let isolated_suffix = isolate_for_direction(suffix, base_direction);
            let mut candidate = String::with_capacity(
                isolated_prefix.len() + DEFAULT_ELLIPSIS.len() + isolated_suffix.len(),
            );
            candidate.push_str(&isolated_prefix);
            candidate.push_str(DEFAULT_ELLIPSIS);
            candidate.push_str(&isolated_suffix);
            candidate
        }
    }
}

fn build_ellipsized_text(
    entity: Entity,
    label: &UTextLabel,
    text_font: &TextFont,
    text_layout: &TextLayout,
    text_color: Color,
    bounds: TextBounds,
    fonts: &Assets<Font>,
    text_pipeline: &mut TextPipeline,
    computed: &mut ComputedTextBlock,
    font_system: &mut CosmicFontSystem,
) -> Result<(String, MeasuredTextInfo), ()> {
    let base_direction = base_direction_for_text(&label.text);
    let truncate_side = resolve_truncate_side(label.truncate_side, base_direction);
    let ellipsis_only = measure_layout_for_text(
        entity,
        DEFAULT_ELLIPSIS,
        text_font,
        text_layout,
        text_color,
        bounds,
        fonts,
        text_pipeline,
        computed,
        font_system,
    )?;

    if !text_fits_constraints(&ellipsis_only, bounds, label) {
        let empty = measure_layout_for_text(
            entity,
            "",
            text_font,
            text_layout,
            text_color,
            bounds,
            fonts,
            text_pipeline,
            computed,
            font_system,
        )?;
        return Ok((String::new(), empty));
    }

    let boundaries = text_char_boundaries(&label.text);
    let total_graphemes = boundaries.len().saturating_sub(1);
    if total_graphemes == 0 {
        return Ok((DEFAULT_ELLIPSIS.to_string(), ellipsis_only));
    }

    let mut best_text = DEFAULT_ELLIPSIS.to_string();
    let mut best_measured = ellipsis_only;
    let mut low = 0usize;
    let mut high = total_graphemes;

    while low < high {
        let mid = (low + high + 1) / 2;
        let candidate =
            build_truncate_candidate(&label.text, &boundaries, truncate_side, mid, base_direction);

        let measured = measure_layout_for_text(
            entity,
            &candidate,
            text_font,
            text_layout,
            text_color,
            bounds,
            fonts,
            text_pipeline,
            computed,
            font_system,
        )?;

        if text_fits_constraints(&measured, bounds, label) {
            best_text = candidate;
            best_measured = measured;
            low = mid;
        } else {
            high = mid.saturating_sub(1);
        }
    }

    Ok((best_text, best_measured))
}

/// Internal system that measures text and updates [`UTextLabelLayoutCache`].
#[doc(hidden)]
pub fn measure_text_label_layout(
    mut font_events: MessageReader<AssetEvent<Font>>,
    fonts: Res<Assets<Font>>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut font_system: ResMut<CosmicFontSystem>,
    parents_query: Query<&ChildOf>,
    parent_query: Query<(&UNode, &ComputedSize)>,
    mut query: Query<(
        Entity,
        Ref<UTextLabel>,
        Ref<UNode>,
        Ref<ComputedSize>,
        &mut ComputedTextBlock,
        &mut UTextLabelLayoutCache,
    )>,
) {
    let mut changed_fonts = HashSet::new();
    for event in font_events.read() {
        match *event {
            AssetEvent::Added { id }
            | AssetEvent::Modified { id }
            | AssetEvent::LoadedWithDependencies { id }
            | AssetEvent::Removed { id }
            | AssetEvent::Unused { id } => {
                changed_fonts.insert(id);
            }
        }
    }

    for (entity, label, node, computed_size, mut computed, mut cache) in query.iter_mut() {
        let font_changed = changed_fonts.contains(&label.font.id());
        if !(label.is_changed()
            || node.is_changed()
            || computed_size.is_changed()
            || cache.dirty
            || font_changed)
        {
            continue;
        }

        let text_font = label_text_font(&label);
        let text_layout = label_text_layout(&label);
        let text_color = label.color;
        let parent_bounds =
            parent_label_bounds(entity, &label, &node, &parents_query, &parent_query);
        let bounds = label_measure_bounds(&label, &node, Some(&computed_size), parent_bounds);

        match measure_layout_for_text(
            entity,
            label.text.as_str(),
            &text_font,
            &text_layout,
            text_color,
            bounds,
            &fonts,
            &mut text_pipeline,
            &mut computed,
            &mut font_system,
        ) {
            Ok(measured) => {
                let Ok((final_text, final_measured, overflowed)) = resolve_final_measured_text(
                    entity,
                    &label,
                    &text_font,
                    &text_layout,
                    text_color,
                    bounds,
                    &fonts,
                    &mut text_pipeline,
                    &mut computed,
                    &mut font_system,
                    measured,
                ) else {
                    cache.measured_size = Vec2::ZERO;
                    cache.min_content_size = Vec2::ZERO;
                    cache.max_content_size = Vec2::ZERO;
                    cache.displayed_text.clear();
                    cache.line_count = 0;
                    cache.overflowed = false;
                    cache.dirty = false;
                    continue;
                };

                cache.min_content_size = final_measured.min_content_size;
                cache.max_content_size = final_measured.max_content_size;
                cache.measured_size = final_measured.size;
                cache.displayed_text = final_text;
                cache.line_count = final_measured.line_count;
                cache.overflowed = overflowed;
                cache.dirty = false;
            }
            Err(_) => {
                cache.measured_size = Vec2::ZERO;
                cache.min_content_size = Vec2::ZERO;
                cache.max_content_size = Vec2::ZERO;
                cache.displayed_text.clear();
                cache.line_count = 0;
                cache.overflowed = false;
                cache.dirty = false;
            }
        }
    }
}

/// Internal system that copies measured text size back into autosized [`UNode`]s.
#[doc(hidden)]
pub fn fit_node_to_text_size(
    parents_query: Query<&ChildOf>,
    mut params: ParamSet<(
        Query<(&UNode, &ComputedSize)>,
        Query<(Entity, &UTextLabel, &mut UNode, &UTextLabelLayoutCache)>,
    )>,
) {
    let label_snapshot: Vec<_> = {
        let labels = params.p1();
        labels
            .iter()
            .map(|(entity, label, node, _)| {
                (
                    entity,
                    parents_query.get(entity).ok().map(|p| p.get()),
                    label.overflow,
                    node.margin,
                )
            })
            .collect()
    };

    let parent_bounds_by_entity: HashMap<Entity, TextBounds> = {
        let parent_bounds_query = params.p0();
        label_snapshot
            .into_iter()
            .map(|(entity, parent_entity, overflow, margin)| {
                (
                    entity,
                    parent_label_bounds_from_ids(
                        parent_entity,
                        overflow,
                        margin,
                        &parent_bounds_query,
                    ),
                )
            })
            .collect()
    };

    for (entity, label, mut node, layout_cache) in params.p1().iter_mut() {
        if !label.autosize {
            continue;
        }

        let outer_size = measured_text_outer_size(&node, layout_cache);
        let parent_bounds = parent_bounds_by_entity
            .get(&entity)
            .copied()
            .unwrap_or(TextBounds {
                width: None,
                height: None,
            });
        let clamped_outer_size = clamp_outer_size_to_bounds(outer_size, parent_bounds);
        let target_width = clamped_outer_size.x;
        let target_height = clamped_outer_size.y;

        let current_w = match node.width {
            UVal::Px(v) => v,
            _ => -1.0,
        };
        let current_h = match node.height {
            UVal::Px(v) => v,
            _ => -1.0,
        };

        if (current_w - target_width).abs() > 0.1 {
            node.width = UVal::Px(target_width);
        }
        if (current_h - target_height).abs() > 0.1 {
            node.height = UVal::Px(target_height);
        }
    }
}

/// Internal system that mirrors text measurement into [`IntrinsicSize`].
#[doc(hidden)]
pub fn sync_text_label_intrinsic_size(
    parents_query: Query<&ChildOf>,
    parent_bounds_query: Query<(&UNode, &ComputedSize)>,
    mut query: Query<
        (
            Entity,
            &UNode,
            &UTextLabel,
            &UTextLabelLayoutCache,
            &mut IntrinsicSize,
        ),
        Or<(
            Added<UTextLabel>,
            Changed<UTextLabel>,
            Changed<UNode>,
            Changed<UTextLabelLayoutCache>,
        )>,
    >,
) {
    for (entity, node, label, layout_cache, mut intrinsic) in query.iter_mut() {
        if layout_cache.dirty {
            continue;
        }

        let mut target = desired_text_label_intrinsic_size(node, layout_cache, *intrinsic);
        if label.autosize {
            let parent_bounds =
                parent_label_bounds(entity, label, node, &parents_query, &parent_bounds_query);
            let clamped_outer_size =
                clamp_outer_size_to_bounds(Vec2::new(target.width, target.height), parent_bounds);
            target.width = clamped_outer_size.x;
            target.height = clamped_outer_size.y;
        }
        if (intrinsic.width - target.width).abs() > 0.1
            || (intrinsic.height - target.height).abs() > 0.1
        {
            *intrinsic = target;
        }
    }
}

fn extract_mask_alpha(data: &[u8], width: usize, height: usize) -> Vec<u8> {
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

fn squared_distance_transform_1d(f: &[f32], d: &mut [f32], v: &mut [usize], z: &mut [f32]) {
    let n = f.len();
    if n == 0 {
        return;
    }

    let mut k = 0usize;
    v[0] = 0;
    z[0] = -DISTANCE_FIELD_INF;
    z[1] = DISTANCE_FIELD_INF;

    for q in 1..n {
        let mut s;
        loop {
            let p = v[k];
            s = ((f[q] + (q * q) as f32) - (f[p] + (p * p) as f32)) / (2.0 * (q - p) as f32);
            if s > z[k] || k == 0 {
                break;
            }
            k -= 1;
        }

        if s <= z[k] && k == 0 {
            z[1] = DISTANCE_FIELD_INF;
            v[0] = q;
            continue;
        }

        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = DISTANCE_FIELD_INF;
    }

    k = 0;
    for q in 0..n {
        while z[k + 1] < q as f32 {
            k += 1;
        }
        let p = v[k];
        d[q] = (q as f32 - p as f32).powi(2) + f[p];
    }
}

fn distance_transform(mask: &[bool], width: usize, height: usize) -> Vec<f32> {
    let mut grid = vec![0.0; width * height];
    for (dst, &is_feature) in grid.iter_mut().zip(mask.iter()) {
        *dst = if is_feature { 0.0 } else { DISTANCE_FIELD_INF };
    }

    let mut tmp = vec![0.0; width * height];
    let mut input = vec![0.0; width.max(height)];
    let mut output = vec![0.0; width.max(height)];
    let mut v = vec![0usize; width.max(height)];
    let mut z = vec![0.0; width.max(height) + 1];

    for x in 0..width {
        for y in 0..height {
            input[y] = grid[y * width + x];
        }
        squared_distance_transform_1d(
            &input[..height],
            &mut output[..height],
            &mut v[..height],
            &mut z[..=height],
        );
        for y in 0..height {
            tmp[y * width + x] = output[y];
        }
    }

    let mut out = vec![0.0; width * height];
    for y in 0..height {
        let row = &tmp[y * width..(y + 1) * width];
        input[..width].copy_from_slice(row);
        squared_distance_transform_1d(
            &input[..width],
            &mut output[..width],
            &mut v[..width],
            &mut z[..=width],
        );
        out[y * width..(y + 1) * width].copy_from_slice(&output[..width]);
    }

    out
}

fn build_sdf_glyph_image(mask: &[u8], width: u32, height: u32) -> Image {
    let padded_width = width + TEXT_SDF_PADDING * 2;
    let padded_height = height + TEXT_SDF_PADDING * 2;
    let padded_len = (padded_width * padded_height) as usize;
    let mut inside_mask = vec![false; padded_len];
    let mut coverage = vec![0.0; padded_len];

    for y in 0..height as usize {
        for x in 0..width as usize {
            let src = y * width as usize + x;
            let dst = (y + TEXT_SDF_PADDING as usize) * padded_width as usize
                + (x + TEXT_SDF_PADDING as usize);
            let alpha = f32::from(mask[src]) / 255.0;
            inside_mask[dst] = alpha >= 0.5;
            coverage[dst] = alpha;
        }
    }

    let outside_mask: Vec<bool> = inside_mask.iter().map(|inside| !inside).collect();
    let distance_to_inside =
        distance_transform(&inside_mask, padded_width as usize, padded_height as usize);
    let distance_to_outside =
        distance_transform(&outside_mask, padded_width as usize, padded_height as usize);
    let spread = TEXT_SDF_PADDING as f32;

    let mut data = Vec::with_capacity(padded_len * 4);
    for idx in 0..padded_len {
        // Preserve the original antialiased coverage so diagonal strokes do not collapse
        // into a hard binary silhouette before we build the distance field.
        let signed_distance = distance_to_outside[idx].sqrt() - distance_to_inside[idx].sqrt()
            + (coverage[idx] - 0.5);
        let alpha = (0.5 + 0.5 * (signed_distance / spread)).clamp(0.0, 1.0);
        let alpha_u8 = (alpha * 255.0).round() as u8;
        data.extend_from_slice(&[255, 255, 255, alpha_u8]);
    }

    let mut image = Image::new(
        Extent3d {
            width: padded_width,
            height: padded_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::linear();
    image
}

fn create_text_atlas_page(
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

fn clip_quad_to_rect(quad: TextGlyphQuad, clip_rect: LocalClipRect) -> Option<TextGlyphQuad> {
    let half = quad.size * 0.5;
    let original_min = quad.center - half;
    let original_max = quad.center + half;

    let clipped_min = Vec2::new(
        original_min.x.max(clip_rect.min.x),
        original_min.y.max(clip_rect.min.y),
    );
    let clipped_max = Vec2::new(
        original_max.x.min(clip_rect.max.x),
        original_max.y.min(clip_rect.max.y),
    );

    if clipped_min.x >= clipped_max.x || clipped_min.y >= clipped_max.y {
        return None;
    }

    let original_width = original_max.x - original_min.x;
    let original_height = original_max.y - original_min.y;
    if original_width <= 0.0 || original_height <= 0.0 {
        return None;
    }

    let left_t = (clipped_min.x - original_min.x) / original_width;
    let right_t = (clipped_max.x - original_min.x) / original_width;
    let bottom_t = (clipped_min.y - original_min.y) / original_height;
    let top_t = (clipped_max.y - original_min.y) / original_height;

    let uv_left = quad.uv_min.x + (quad.uv_max.x - quad.uv_min.x) * left_t;
    let uv_right = quad.uv_min.x + (quad.uv_max.x - quad.uv_min.x) * right_t;
    let uv_bottom = quad.uv_max.y + (quad.uv_min.y - quad.uv_max.y) * bottom_t;
    let uv_top = quad.uv_max.y + (quad.uv_min.y - quad.uv_max.y) * top_t;

    Some(TextGlyphQuad {
        center: (clipped_min + clipped_max) * 0.5,
        size: clipped_max - clipped_min,
        uv_min: Vec2::new(uv_left, uv_top),
        uv_max: Vec2::new(uv_right, uv_bottom),
    })
}

fn label_content_clip_rect(
    label: &UTextLabel,
    node: &UNode,
    computed_size: &ComputedSize,
) -> Option<LocalClipRect> {
    if label.overflow == UTextOverflow::Visible {
        return None;
    }

    if computed_size.width <= 0.0 || computed_size.height <= 0.0 {
        return None;
    }

    let width = (computed_size.width - node.padding.width_sum()).max(0.0);
    let height = (computed_size.height - node.padding.height_sum()).max(0.0);

    Some(LocalClipRect {
        min: Vec2::new(-width * 0.5, -height * 0.5),
        max: Vec2::new(width * 0.5, height * 0.5),
    })
}

fn build_text_page_mesh(quads: &[TextGlyphQuad]) -> Mesh {
    let mut positions = Vec::with_capacity(quads.len() * 4);
    let mut normals = Vec::with_capacity(quads.len() * 4);
    let mut uvs = Vec::with_capacity(quads.len() * 4);
    let mut indices = Vec::with_capacity(quads.len() * 6);

    for (quad_index, quad) in quads.iter().enumerate() {
        let base = (quad_index * 4) as u32;
        let half = quad.size * 0.5;
        let min = quad.center - half;
        let max = quad.center + half;

        positions.extend_from_slice(&[
            [min.x, min.y, 0.0],
            [max.x, min.y, 0.0],
            [min.x, max.y, 0.0],
            [max.x, max.y, 0.0],
        ]);
        normals.extend_from_slice(&[[0.0, 0.0, 1.0]; 4]);
        uvs.extend_from_slice(&[
            [quad.uv_min.x, quad.uv_max.y],
            [quad.uv_max.x, quad.uv_max.y],
            [quad.uv_min.x, quad.uv_min.y],
            [quad.uv_max.x, quad.uv_min.y],
        ]);
        indices.extend_from_slice(&[base, base + 2, base + 1, base + 1, base + 2, base + 3]);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn build_text_material(label: &UTextLabel, texture: Handle<Image>) -> UTextLabelSdfMaterial {
    UTextLabelSdfMaterial {
        color: label.color.into(),
        clip_radius: Vec4::ZERO,
        clip_center: Vec2::ZERO,
        clip_size: Vec2::ZERO,
        edge_softness: DEFAULT_TEXT_EDGE_SOFTNESS,
        use_clip: 0,
        _pad: Vec2::ZERO,
        texture,
    }
}

fn text_world_scale_for_entity(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi>,
) -> f32 {
    let mut current = entity;

    loop {
        if let Ok(root) = root_query.get(current) {
            return root.ui_units_to_world_scale();
        }

        let Ok(parent) = parents_query.get(current) else {
            return 1.0;
        };
        current = parent.parent();
    }
}

fn text_root_stack_for_entity(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_stack_query: &Query<&ResolvedRootStack>,
) -> ResolvedRootStack {
    let mut current = entity;

    loop {
        if let Ok(stack) = root_stack_query.get(current) {
            return *stack;
        }

        let Ok(parent) = parents_query.get(current) else {
            return ResolvedRootStack::default();
        };
        current = parent.parent();
    }
}

fn text_render_transform(world_scale: f32, depth_offset: f32) -> Transform {
    Transform {
        translation: Vec3::new(0.0, 0.0, depth_offset),
        scale: Vec3::splat(world_scale),
        ..default()
    }
}

fn sync_text_label_meshes(
    mut commands: Commands,
    mut atlas_cache: ResMut<UTextLabelAtlasCache>,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<UTextLabelSdfMaterial>>,
    mut font_system: ResMut<CosmicFontSystem>,
    mut swash_cache: ResMut<bevy::text::SwashCache>,
    label_query: Query<
        (
            Entity,
            Ref<UTextLabel>,
            Ref<UNode>,
            Ref<ComputedSize>,
            Ref<ComputedTextBlock>,
            Ref<UTextLabelLayoutCache>,
            Option<&Children>,
        ),
        Or<(
            Added<UTextLabel>,
            Changed<UTextLabel>,
            Changed<UNode>,
            Changed<ComputedSize>,
            Changed<ComputedTextBlock>,
            Changed<UTextLabelLayoutCache>,
        )>,
    >,
    child_query: Query<
        (
            Entity,
            &ChildOf,
            &TextRenderPageMarker,
            &Mesh2d,
            &MeshMaterial2d<UTextLabelSdfMaterial>,
        ),
        With<TextChildMarker>,
    >,
) {
    for (entity, label, node, computed_size, computed, layout_cache, children) in label_query.iter()
    {
        let mut existing_children = HashMap::new();
        if let Some(children) = children {
            for child in children {
                if let Ok((child_entity, child_of, page_marker, mesh_handle, material_handle)) =
                    child_query.get(*child)
                {
                    if child_of.get() == entity {
                        existing_children.insert(
                            page_marker.texture_id,
                            (
                                child_entity,
                                mesh_handle.0.clone(),
                                material_handle.0.clone(),
                            ),
                        );
                    }
                }
            }
        }

        let text_size = layout_cache.measured_size;
        let render_scale = resolved_render_scale(label.render_scale);
        let content_clip = label_content_clip_rect(&label, &node, &computed_size);
        let horizontal_offset = text_horizontal_offset(&label, &node, &computed_size, text_size);
        let mut page_batches: HashMap<AssetId<Image>, TextPageBatch> = HashMap::default();

        if !layout_cache.displayed_text.is_empty() && text_size.x > 0.0 && text_size.y > 0.0 {
            for run in computed.buffer().layout_runs() {
                let line_y = (run.line_y * render_scale).round();

                for layout_glyph in run.glyphs.iter() {
                    let physical_glyph = layout_glyph.physical((0.0, 0.0), render_scale);
                    let cache_key = physical_glyph.cache_key;
                    let atlas_key = UTextLabelAtlasKey {
                        font_id: label.font.id(),
                        font_size_bits: cache_key.font_size_bits,
                        font_weight: cache_key.font_weight.0,
                        flags_bits: cache_key.flags.bits(),
                    };
                    let glyph_key = UTextLabelGlyphKey {
                        glyph_id: cache_key.glyph_id,
                        x_bin_bits: cache_key.x_bin.as_float().to_bits(),
                        y_bin_bits: cache_key.y_bin.as_float().to_bits(),
                    };

                    let atlas_pages = atlas_cache.pages_by_font.entry(atlas_key).or_default();
                    let atlas_info = if let Some(info) = atlas_pages
                        .iter()
                        .find_map(|page| page.glyphs.get(&glyph_key))
                        .cloned()
                    {
                        info
                    } else {
                        let Some(image) = swash_cache
                            .0
                            .get_image_uncached(&mut font_system.0, physical_glyph.cache_key)
                        else {
                            continue;
                        };

                        let width = image.placement.width;
                        let height = image.placement.height;
                        if width == 0 || height == 0 {
                            continue;
                        }

                        let mask = extract_mask_alpha(&image.data, width as usize, height as usize);
                        if mask.iter().all(|alpha| *alpha == 0) {
                            continue;
                        }

                        let glyph_image = build_sdf_glyph_image(&mask, width, height);
                        let glyph_offset = IVec2::new(
                            image.placement.left - TEXT_SDF_PADDING as i32,
                            image.placement.top + TEXT_SDF_PADDING as i32,
                        );

                        let mut stored = None;
                        for page in atlas_pages.iter_mut() {
                            let Some(atlas_layout) = texture_atlases.get_mut(&page.texture_atlas)
                            else {
                                continue;
                            };
                            let Some(atlas_image) = images.get_mut(&page.texture) else {
                                continue;
                            };
                            if let Ok(glyph_index) =
                                page.builder
                                    .add_texture(atlas_layout, &glyph_image, atlas_image)
                            {
                                let info = UTextLabelGlyphAtlasInfo {
                                    texture: page.texture.clone(),
                                    texture_atlas: page.texture_atlas.clone(),
                                    glyph_index,
                                    offset: glyph_offset,
                                };
                                page.glyphs.insert(glyph_key, info.clone());
                                stored = Some(info);
                                break;
                            }
                        }

                        if let Some(info) = stored {
                            info
                        } else {
                            let min_size = glyph_image
                                .texture_descriptor
                                .size
                                .width
                                .max(glyph_image.texture_descriptor.size.height);
                            let mut page =
                                create_text_atlas_page(&mut images, &mut texture_atlases, min_size);
                            let Some(atlas_layout) = texture_atlases.get_mut(&page.texture_atlas)
                            else {
                                continue;
                            };
                            let Some(atlas_image) = images.get_mut(&page.texture) else {
                                continue;
                            };
                            let Ok(glyph_index) =
                                page.builder
                                    .add_texture(atlas_layout, &glyph_image, atlas_image)
                            else {
                                continue;
                            };
                            let info = UTextLabelGlyphAtlasInfo {
                                texture: page.texture.clone(),
                                texture_atlas: page.texture_atlas.clone(),
                                glyph_index,
                                offset: glyph_offset,
                            };
                            page.glyphs.insert(glyph_key, info.clone());
                            atlas_pages.push(page);
                            info
                        }
                    };

                    let Some(atlas_layout) = texture_atlases.get(&atlas_info.texture_atlas) else {
                        continue;
                    };
                    let Some(atlas_image) = images.get(&atlas_info.texture) else {
                        continue;
                    };
                    let glyph_rect = atlas_layout.textures[atlas_info.glyph_index];
                    let glyph_size_pixels =
                        Vec2::new(glyph_rect.width() as f32, glyph_rect.height() as f32);
                    let glyph_size = glyph_size_pixels / render_scale;

                    let position_x = (glyph_size_pixels.x * 0.5
                        + atlas_info.offset.x as f32
                        + physical_glyph.x as f32)
                        / render_scale;
                    let position_y = (line_y + physical_glyph.y as f32
                        - atlas_info.offset.y as f32
                        + glyph_size_pixels.y * 0.5)
                        / render_scale;

                    let local_center = Vec2::new(
                        horizontal_offset + position_x - text_size.x * 0.5,
                        text_size.y * 0.5 - position_y,
                    );
                    let atlas_width = atlas_image.texture_descriptor.size.width as f32;
                    let atlas_height = atlas_image.texture_descriptor.size.height as f32;
                    let uv_min = Vec2::new(
                        glyph_rect.min.x as f32 / atlas_width,
                        glyph_rect.min.y as f32 / atlas_height,
                    );
                    let uv_max = Vec2::new(
                        glyph_rect.max.x as f32 / atlas_width,
                        glyph_rect.max.y as f32 / atlas_height,
                    );

                    let quad = TextGlyphQuad {
                        center: local_center,
                        size: glyph_size,
                        uv_min,
                        uv_max,
                    };
                    let clipped_quad = if let Some(clip_rect) = content_clip {
                        clip_quad_to_rect(quad, clip_rect)
                    } else {
                        Some(quad)
                    };
                    let Some(quad) = clipped_quad else {
                        continue;
                    };

                    page_batches
                        .entry(atlas_info.texture.id())
                        .and_modify(|batch| batch.quads.push(quad.clone()))
                        .or_insert_with(|| TextPageBatch {
                            texture: atlas_info.texture.clone(),
                            quads: vec![quad],
                        });
                }
            }
        }

        for (texture_id, batch) in page_batches {
            let existing = existing_children.remove(&texture_id);
            let mesh = build_text_page_mesh(&batch.quads);

            match existing {
                Some((child_entity, mesh_handle, material_handle)) => {
                    if let Some(existing_mesh) = meshes.get_mut(&mesh_handle) {
                        *existing_mesh = mesh;
                    }
                    if let Some(existing_material) = materials.get_mut(&material_handle) {
                        *existing_material = build_text_material(&label, batch.texture.clone());
                    }
                    commands
                        .entity(child_entity)
                        .insert(TextRenderPageMarker { texture_id });
                }
                None => {
                    commands.entity(entity).with_children(|parent| {
                        parent.spawn((
                            TextRenderPageMarker { texture_id },
                            TextChildMarker,
                            Mesh2d(meshes.add(mesh)),
                            MeshMaterial2d(
                                materials.add(build_text_material(&label, batch.texture.clone())),
                            ),
                            Transform::default(),
                            Visibility::Inherited,
                        ));
                    });
                }
            }
        }

        for (_, (stale_child, _, _)) in existing_children {
            commands.entity(stale_child).despawn();
        }
    }
}

fn active_clipper_for_entity(
    start_entity: Entity,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    root_query: &Query<&ResolvedRootUi>,
) -> MaterialClipInfo {
    let mut current = start_entity;
    let world_scale = text_world_scale_for_entity(start_entity, parents_query, root_query);

    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();

        let Ok((transform, computed_size, node, clip)) = clipper_query.get(current) else {
            continue;
        };

        if !clip.enabled {
            continue;
        }

        let size = Vec2::new(computed_size.width, computed_size.height) * world_scale;
        if size.x <= 0.0 || size.y <= 0.0 {
            continue;
        }

        return MaterialClipInfo {
            center: transform.translation().truncate(),
            size,
            radius: Vec4::new(
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            ) * world_scale,
            use_clip: 1,
        };
    }

    MaterialClipInfo::default()
}

fn sync_text_clipper_materials(
    mut text_query: Query<
        (
            Entity,
            &MeshMaterial2d<UTextLabelSdfMaterial>,
            &mut Transform,
        ),
        With<TextChildMarker>,
    >,
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    root_query: Query<&ResolvedRootUi>,
    root_stack_query: Query<&ResolvedRootStack>,
    mut materials: ResMut<Assets<UTextLabelSdfMaterial>>,
) {
    for (entity, material_handle, mut transform) in text_query.iter_mut() {
        let world_scale = text_world_scale_for_entity(entity, &parents_query, &root_query);
        let root_stack = text_root_stack_for_entity(entity, &parents_query, &root_stack_query);
        let clip = active_clipper_for_entity(entity, &parents_query, &clipper_query, &root_query);
        let Some(material) = materials.get_mut(&material_handle.0) else {
            continue;
        };
        let desired_transform = text_render_transform(world_scale, root_stack.text_child_offset());
        if *transform != desired_transform {
            *transform = desired_transform;
        }
        if material.clip_center != clip.center
            || material.clip_size != clip.size
            || material.clip_radius != clip.radius
            || material.use_clip != clip.use_clip
            || material.edge_softness != DEFAULT_TEXT_EDGE_SOFTNESS
        {
            material.clip_center = clip.center;
            material.clip_size = clip.size;
            material.clip_radius = clip.radius;
            material.use_clip = clip.use_clip;
            material.edge_softness = DEFAULT_TEXT_EDGE_SOFTNESS;
        }
    }
}

fn mark_text_label_layout_dirty(
    mut layout_cache: ResMut<LayoutCache>,
    changed_labels: Query<
        Entity,
        (
            With<UTextLabel>,
            Or<(Added<UTextLabel>, Changed<IntrinsicSize>, Changed<UNode>)>,
        ),
    >,
    children_query: Query<&Children>,
    parents_query: Query<&ChildOf>,
) {
    for entity in changed_labels.iter() {
        layout_cache.mark_dirty_recursive(entity, &children_query);
        layout_cache.mark_dirty_ancestors(entity, &parents_query);
    }
}

/// Registers the `UTextLabel` measurement and SDF rendering pipeline.
pub struct UnivisTextPlugin;

impl Plugin for UnivisTextPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shaders/text_label_sdf.wgsl");

        app.register_type::<UTextLabel>()
            .register_type::<UTextOverflow>()
            .register_type::<UTextTruncateSide>()
            .register_type::<UTextLabelLayoutCache>()
            .init_resource::<UTextLabelAtlasCache>()
            .add_plugins(Material2dPlugin::<UTextLabelSdfMaterial>::default())
            .add_systems(
                PostUpdate,
                measure_text_label_layout
                    .in_set(UnivisPostUpdateSet::WidgetSync)
                    .before(sync_text_label_intrinsic_size),
            )
            .add_systems(
                PostUpdate,
                sync_text_label_intrinsic_size
                    .in_set(UnivisPostUpdateSet::WidgetSync)
                    .before(fit_node_to_text_size),
            )
            .add_systems(
                PostUpdate,
                fit_node_to_text_size
                    .in_set(UnivisPostUpdateSet::WidgetSync)
                    .before(mark_text_label_layout_dirty),
            )
            .add_systems(
                PostUpdate,
                mark_text_label_layout_dirty
                    .in_set(UnivisPostUpdateSet::WidgetSync)
                    .before(sync_text_label_meshes),
            )
            .add_systems(
                PostUpdate,
                sync_text_label_meshes
                    .in_set(UnivisPostUpdateSet::WidgetSync)
                    .before(UnivisPostUpdateSet::LayoutMeasure),
            )
            .add_systems(
                PostUpdate,
                sync_text_clipper_materials.in_set(UnivisPostUpdateSet::RenderSync),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_quad_to_rect_clips_size_and_uvs() {
        let quad = TextGlyphQuad {
            center: Vec2::new(0.0, 0.0),
            size: Vec2::new(10.0, 6.0),
            uv_min: Vec2::new(0.1, 0.2),
            uv_max: Vec2::new(0.5, 0.8),
        };
        let clip_rect = LocalClipRect {
            min: Vec2::new(-2.0, -3.0),
            max: Vec2::new(5.0, 3.0),
        };

        let clipped = clip_quad_to_rect(quad, clip_rect).expect("quad should be clipped");

        assert_eq!(clipped.size, Vec2::new(7.0, 6.0));
        assert_eq!(clipped.center, Vec2::new(1.5, 0.0));
        assert!((clipped.uv_min.x - 0.22).abs() < 0.001);
        assert!((clipped.uv_max.x - 0.5).abs() < 0.001);
    }

    #[test]
    fn clip_quad_to_rect_returns_none_when_outside() {
        let quad = TextGlyphQuad {
            center: Vec2::new(20.0, 0.0),
            size: Vec2::new(4.0, 4.0),
            uv_min: Vec2::ZERO,
            uv_max: Vec2::ONE,
        };
        let clip_rect = LocalClipRect {
            min: Vec2::new(-5.0, -5.0),
            max: Vec2::new(5.0, 5.0),
        };

        assert!(clip_quad_to_rect(quad, clip_rect).is_none());
    }

    #[test]
    fn desired_intrinsic_size_uses_content_dimensions_only() {
        let node = UNode {
            width: UVal::Content,
            height: UVal::Px(48.0),
            padding: USides::axes(6.0, 4.0),
            ..default()
        };
        let layout_cache = UTextLabelLayoutCache {
            min_content_size: Vec2::new(48.0, 14.0),
            max_content_size: Vec2::new(120.0, 22.0),
            ..default()
        };
        let current = IntrinsicSize {
            width: 1.0,
            height: 48.0,
            ..default()
        };

        let desired = desired_text_label_intrinsic_size(&node, &layout_cache, current);

        assert_eq!(desired.width, 132.0);
        assert_eq!(desired.height, 48.0);
        assert_eq!(desired.min_width, 60.0);
        assert_eq!(desired.max_width, 132.0);
    }

    #[test]
    fn desired_intrinsic_size_uses_min_content_dimensions_when_requested() {
        let node = UNode {
            width: UVal::MinContent,
            height: UVal::MinContent,
            padding: USides::axes(6.0, 4.0),
            ..default()
        };
        let layout_cache = UTextLabelLayoutCache {
            min_content_size: Vec2::new(48.0, 14.0),
            max_content_size: Vec2::new(120.0, 22.0),
            ..default()
        };

        let desired =
            desired_text_label_intrinsic_size(&node, &layout_cache, IntrinsicSize::default());

        assert_eq!(desired.width, 60.0);
        assert_eq!(desired.height, 22.0);
        assert_eq!(desired.min_width, 60.0);
        assert_eq!(desired.max_width, 132.0);
        assert_eq!(desired.min_height, 22.0);
        assert_eq!(desired.max_height, 30.0);
    }

    #[test]
    fn measured_text_outer_size_includes_padding_for_empty_text() {
        let node = UNode {
            padding: USides::axes(8.0, 10.0),
            ..default()
        };
        let layout_cache = UTextLabelLayoutCache {
            measured_size: Vec2::ZERO,
            ..default()
        };

        let outer = measured_text_outer_size(&node, &layout_cache);

        assert_eq!(outer, Vec2::new(16.0, 20.0));
    }

    #[test]
    fn grapheme_boundaries_keep_joined_emoji_together() {
        let boundaries = text_char_boundaries("A🧑‍💻B");

        assert_eq!(boundaries.len(), 4);
        assert_eq!(&"A🧑‍💻B"[..boundaries[2]], "A🧑‍💻");
    }

    #[test]
    fn base_direction_detects_rtl_text() {
        assert_eq!(
            base_direction_for_text("مرحبا بالعالم"),
            TextBaseDirection::Rtl
        );
        assert_eq!(
            base_direction_for_text("hello world"),
            TextBaseDirection::Ltr
        );
    }

    #[test]
    fn auto_truncate_side_defaults_to_end_for_ltr_and_rtl() {
        assert_eq!(
            resolve_truncate_side(UTextTruncateSide::Auto, TextBaseDirection::Ltr),
            UTextTruncateSide::End
        );
        assert_eq!(
            resolve_truncate_side(UTextTruncateSide::Auto, TextBaseDirection::Rtl),
            UTextTruncateSide::End
        );
    }

    #[test]
    fn default_text_label_uses_ellipsis_overflow() {
        let label = UTextLabel::default();

        assert_eq!(label.overflow, UTextOverflow::Ellipsis);
    }

    #[test]
    fn default_text_label_uses_auto_truncate_side() {
        let label = UTextLabel::default();

        assert_eq!(label.truncate_side, UTextTruncateSide::Auto);
    }

    #[test]
    fn left_justify_offsets_text_toward_start_edge() {
        let label = UTextLabel {
            justify: Justify::Left,
            ..default()
        };
        let node = UNode {
            padding: USides::axes(10.0, 0.0),
            ..default()
        };
        let computed_size = ComputedSize {
            width: 100.0,
            height: 20.0,
            ..default()
        };

        let offset = text_horizontal_offset(&label, &node, &computed_size, Vec2::new(200.0, 20.0));

        assert_eq!(offset, 60.0);
    }

    #[test]
    fn right_justify_offsets_text_toward_end_edge() {
        let label = UTextLabel {
            justify: Justify::Right,
            ..default()
        };
        let node = UNode {
            padding: USides::axes(10.0, 0.0),
            ..default()
        };
        let computed_size = ComputedSize {
            width: 100.0,
            height: 20.0,
            ..default()
        };

        let offset = text_horizontal_offset(&label, &node, &computed_size, Vec2::new(40.0, 20.0));

        assert_eq!(offset, 20.0);
    }

    #[test]
    fn center_justify_keeps_zero_offset() {
        let label = UTextLabel {
            justify: Justify::Center,
            ..default()
        };
        let node = UNode {
            padding: USides::axes(10.0, 0.0),
            ..default()
        };
        let computed_size = ComputedSize {
            width: 100.0,
            height: 20.0,
            ..default()
        };

        let offset = text_horizontal_offset(&label, &node, &computed_size, Vec2::new(40.0, 20.0));

        assert_eq!(offset, 0.0);
    }

    #[test]
    fn local_clip_rect_exists_for_ellipsis_even_with_autosize() {
        let label = UTextLabel {
            autosize: true,
            overflow: UTextOverflow::Ellipsis,
            ..default()
        };
        let node = UNode {
            padding: USides::axes(10.0, 6.0),
            ..default()
        };
        let computed_size = ComputedSize {
            width: 120.0,
            height: 40.0,
            ..default()
        };

        let clip = label_content_clip_rect(&label, &node, &computed_size).unwrap();

        assert_eq!(clip.min, Vec2::new(-50.0, -14.0));
        assert_eq!(clip.max, Vec2::new(50.0, 14.0));
    }

    #[test]
    fn visible_overflow_disables_local_clip_rect() {
        let label = UTextLabel {
            overflow: UTextOverflow::Visible,
            ..default()
        };
        let node = UNode {
            padding: USides::all(8.0),
            ..default()
        };
        let computed_size = ComputedSize {
            width: 120.0,
            height: 40.0,
            ..default()
        };

        assert!(label_content_clip_rect(&label, &node, &computed_size).is_none());
    }

    #[test]
    fn autosize_measure_bounds_use_parent_constraints_when_available() {
        let label = UTextLabel {
            autosize: true,
            overflow: UTextOverflow::Ellipsis,
            ..default()
        };
        let node = UNode::default();
        let bounds = label_measure_bounds(
            &label,
            &node,
            None,
            TextBounds {
                width: Some(180.0),
                height: Some(44.0),
            },
        );

        assert_eq!(bounds.width, Some(180.0));
        assert_eq!(bounds.height, Some(44.0));
    }

    #[test]
    fn truncate_candidate_end_keeps_prefix() {
        let text = "abcdef";
        let boundaries = text_char_boundaries(text);
        let candidate = build_truncate_candidate(
            text,
            &boundaries,
            UTextTruncateSide::End,
            3,
            TextBaseDirection::Ltr,
        );

        assert_eq!(candidate, format!("{LTR_ISOLATE_START}abc{ISOLATE_END}..."));
    }

    #[test]
    fn truncate_candidate_start_keeps_suffix() {
        let text = "abcdef";
        let boundaries = text_char_boundaries(text);
        let candidate = build_truncate_candidate(
            text,
            &boundaries,
            UTextTruncateSide::Start,
            3,
            TextBaseDirection::Ltr,
        );

        assert_eq!(candidate, format!("...{LTR_ISOLATE_START}def{ISOLATE_END}"));
    }

    #[test]
    fn truncate_candidate_middle_keeps_edges() {
        let text = "abcdef";
        let boundaries = text_char_boundaries(text);
        let candidate = build_truncate_candidate(
            text,
            &boundaries,
            UTextTruncateSide::Middle,
            4,
            TextBaseDirection::Ltr,
        );

        assert_eq!(
            candidate,
            format!("{LTR_ISOLATE_START}ab{ISOLATE_END}...{LTR_ISOLATE_START}ef{ISOLATE_END}")
        );
    }
}
