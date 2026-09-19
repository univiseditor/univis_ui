use super::*;

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

pub(super) fn build_sdf_glyph_image(mask: &[u8], width: u32, height: u32) -> Image {
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
