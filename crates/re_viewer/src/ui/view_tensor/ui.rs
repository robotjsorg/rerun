use std::{collections::BTreeMap, fmt::Display};

use eframe::emath::Align2;
use egui::{epaint::TextShape, Color32, ColorImage, NumExt as _, Vec2};
use half::f16;
use ndarray::{Axis, Ix2};

use re_log_types::{field_types, ClassicTensor, TensorDataType};
use re_tensor_ops::dimension_mapping::DimensionMapping;

use crate::ui::data_ui::image::format_tensor_shape;

use super::dimension_mapping_ui;

// ---

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ViewTensorState {
    /// How we select which dimensions to project the tensor onto.
    dimension_mapping: DimensionMapping,

    /// Selected value of every dimension (iff they are in [`DimensionMapping::selectors`]).
    selector_values: BTreeMap<usize, u64>,

    /// How we map values to colors.
    color_mapping: ColorMapping,

    /// Scaling, filtering, aspect ratio, etc for the rendered texture.
    texture_settings: TextureSettings,

    /// Last viewed tensor, copied each frame.
    /// Used for the selection view.
    #[serde(skip)]
    tensor: Option<ClassicTensor>,
}

impl ViewTensorState {
    pub fn create(tensor: &ClassicTensor) -> ViewTensorState {
        Self {
            selector_values: Default::default(),
            dimension_mapping: DimensionMapping::create(tensor.shape()),
            color_mapping: ColorMapping::default(),
            texture_settings: TextureSettings::default(),
            tensor: Some(tensor.clone()),
        }
    }

    pub(crate) fn ui(&mut self, ctx: &mut crate::misc::ViewerContext<'_>, ui: &mut egui::Ui) {
        if let Some(tensor) = &self.tensor {
            ui.collapsing("Dimension Mapping", |ui| {
                ui.label(format!("dtype: {}", tensor.dtype()));
                ui.label(format!("shape: {}", format_tensor_shape(tensor.shape())));
                ui.add_space(12.0);

                dimension_mapping_ui(ui, &mut self.dimension_mapping, tensor.shape());

                let default_mapping = DimensionMapping::create(tensor.shape());
                if ui
                    .add_enabled(
                        self.dimension_mapping != default_mapping,
                        egui::Button::new("Auto-map"),
                    )
                    .on_disabled_hover_text("The default is already set up")
                    .clicked()
                {
                    self.dimension_mapping = DimensionMapping::create(tensor.shape());
                }
            });
        }

        self.texture_settings.show(ui);

        color_mapping_ui(ctx, ui, &mut self.color_mapping, self.tensor.as_ref());
    }
}

pub(crate) fn view_tensor(
    ctx: &mut crate::misc::ViewerContext<'_>,
    ui: &mut egui::Ui,
    state: &mut ViewTensorState,
    tensor: &ClassicTensor,
) {
    crate::profile_function!();

    state.tensor = Some(tensor.clone());

    if !state.dimension_mapping.is_valid(tensor.num_dim()) {
        state.dimension_mapping = DimensionMapping::create(tensor.shape());
    }

    selectors_ui(ui, state, tensor);

    let tensor_shape = tensor.shape();

    let tensor_stats = ctx.cache.tensor_stats(tensor);
    let range = tensor_stats.range;
    let color_mapping = &state.color_mapping;

    match tensor.dtype {
        TensorDataType::U8 => match re_tensor_ops::as_ndarray::<u8>(tensor) {
            Ok(tensor) => {
                let color_from_value = |value: u8| {
                    // We always use the full range for u8
                    color_mapping.color_from_normalized(value as f32 / 255.0)
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },

        TensorDataType::U16 => match re_tensor_ops::as_ndarray::<u16>(tensor) {
            Ok(tensor) => {
                let color_from_value = |value: u16| {
                    let (tensor_min, tensor_max) = range.unwrap_or((0.0, u16::MAX as f64)); // the cache should provide the range
                    color_mapping.color_from_normalized(egui::remap(
                        value as f32,
                        tensor_min as f32..=tensor_max as f32,
                        0.0..=1.0,
                    ))
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },

        TensorDataType::U32 => match re_tensor_ops::as_ndarray::<u32>(tensor) {
            Ok(tensor) => {
                let (tensor_min, tensor_max) = range.unwrap_or((0.0, u32::MAX as f64)); // the cache should provide the range

                let color_from_value = |value: u32| {
                    color_mapping.color_from_normalized(egui::remap(
                        value as f64,
                        tensor_min..=tensor_max,
                        0.0..=1.0,
                    ) as f32)
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },

        TensorDataType::U64 => match re_tensor_ops::as_ndarray::<u64>(tensor) {
            Ok(tensor) => {
                let color_from_value = |value: u64| {
                    let (tensor_min, tensor_max) = range.unwrap_or((0.0, u64::MAX as f64)); // the cache should provide the range
                    color_mapping.color_from_normalized(egui::remap(
                        value as f64,
                        tensor_min..=tensor_max,
                        0.0..=1.0,
                    ) as f32)
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },

        TensorDataType::I8 => match re_tensor_ops::as_ndarray::<i8>(tensor) {
            Ok(tensor) => {
                let color_from_value = |value: i8| {
                    // We always use the full range for i8:
                    let (tensor_min, tensor_max) = (i8::MIN as f32, i8::MAX as f32);
                    color_mapping.color_from_normalized(egui::remap(
                        value as f32,
                        tensor_min..=tensor_max,
                        0.0..=1.0,
                    ))
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },

        TensorDataType::I16 => match re_tensor_ops::as_ndarray::<i16>(tensor) {
            Ok(tensor) => {
                let color_from_value = |value: i16| {
                    let (tensor_min, tensor_max) =
                        range.unwrap_or((i16::MIN as f64, i16::MAX as f64)); // the cache should provide the range
                    color_mapping.color_from_normalized(egui::remap(
                        value as f32,
                        tensor_min as f32..=tensor_max as f32,
                        0.0..=1.0,
                    ))
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },

        TensorDataType::I32 => match re_tensor_ops::as_ndarray::<i32>(tensor) {
            Ok(tensor) => {
                let color_from_value = |value: i32| {
                    let (tensor_min, tensor_max) =
                        range.unwrap_or((i32::MIN as f64, i32::MAX as f64)); // the cache should provide the range
                    color_mapping.color_from_normalized(egui::remap(
                        value as f64,
                        tensor_min..=tensor_max,
                        0.0..=1.0,
                    ) as f32)
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },

        TensorDataType::I64 => match re_tensor_ops::as_ndarray::<i64>(tensor) {
            Ok(tensor) => {
                let color_from_value = |value: i64| {
                    let (tensor_min, tensor_max) =
                        range.unwrap_or((i64::MIN as f64, i64::MAX as f64)); // the cache should provide the range
                    color_mapping.color_from_normalized(egui::remap(
                        value as f64,
                        tensor_min..=tensor_max,
                        0.0..=1.0,
                    ) as f32)
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },

        TensorDataType::F16 => match re_tensor_ops::as_ndarray::<f16>(tensor) {
            Ok(tensor) => {
                let color_from_value = |value: f16| {
                    let (tensor_min, tensor_max) = range.unwrap_or((0.0, 1.0)); // the cache should provide the range
                    color_mapping.color_from_normalized(egui::remap(
                        value.to_f32(),
                        tensor_min as f32..=tensor_max as f32,
                        0.0..=1.0,
                    ))
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },

        TensorDataType::F32 => match re_tensor_ops::as_ndarray::<f32>(tensor) {
            Ok(tensor) => {
                let color_from_value = |value: f32| {
                    let (tensor_min, tensor_max) = range.unwrap_or((0.0, 1.0)); // the cache should provide the range
                    color_mapping.color_from_normalized(egui::remap(
                        value,
                        tensor_min as f32..=tensor_max as f32,
                        0.0..=1.0,
                    ))
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },

        TensorDataType::F64 => match re_tensor_ops::as_ndarray::<f64>(tensor) {
            Ok(tensor) => {
                let color_from_value = |value: f64| {
                    let (tensor_min, tensor_max) = range.unwrap_or((0.0, 1.0)); // the cache should provide the range
                    color_mapping.color_from_normalized(egui::remap(
                        value,
                        tensor_min..=tensor_max,
                        0.0..=1.0,
                    ) as f32)
                };

                let slice = selected_tensor_slice(state, &tensor);
                slice_ui(ctx, ui, state, tensor_shape, slice, color_from_value);
            }
            Err(err) => {
                ui.label(ctx.re_ui.error_text(err.to_string()));
            }
        },
    }
}

// ----------------------------------------------------------------------------

/// How we map values to colors.
#[derive(Copy, Clone, Debug, serde::Deserialize, serde::Serialize)]
struct ColorMapping {
    turbo: bool,
    gamma: f32,
}

impl Default for ColorMapping {
    fn default() -> Self {
        Self {
            turbo: true,
            gamma: 1.0,
        }
    }
}

impl ColorMapping {
    pub fn color_from_normalized(&self, f: f32) -> Color32 {
        let f = f.powf(self.gamma);

        if self.turbo {
            let [r, g, b] = crate::misc::color_map::turbo_color_map(f);
            Color32::from_rgb(r, g, b)
        } else {
            let lum = (f * 255.0 + 0.5) as u8;
            Color32::from_gray(lum)
        }
    }
}

fn color_mapping_ui(
    ctx: &mut crate::misc::ViewerContext<'_>,
    ui: &mut egui::Ui,
    color_mapping: &mut ColorMapping,
    tensor: Option<&ClassicTensor>,
) {
    ui.group(|ui| {
        ui.strong("Color map");

        ui.horizontal(|ui| {
            ui.radio_value(&mut color_mapping.turbo, false, "Grayscale");
            ui.radio_value(&mut color_mapping.turbo, true, "Turbo");
        });

        let mut brightness = 1.0 / color_mapping.gamma;
        ui.add(
            egui::Slider::new(&mut brightness, 0.1..=10.0)
                .logarithmic(true)
                .text("Brightness"),
        );
        color_mapping.gamma = 1.0 / brightness;

        if let Some(tensor) = &tensor {
            let tensor_stats = ctx.cache.tensor_stats(tensor);
            if let Some((min, max)) = tensor_stats.range {
                ui.monospace(format!("Data range: [{min} - {max}]"));
            }
        }
    });
}

// ----------------------------------------------------------------------------

/// Should we scale the rendered texture, and if so, how?
#[derive(Copy, Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum TextureScaling {
    /// No scaling, texture size will match the tensor's width/height dimensions.
    None,
    /// Scale the texture for the largest possible fit in the UI container.
    Fit,
}

impl Default for TextureScaling {
    fn default() -> Self {
        Self::Fit
    }
}

impl Display for TextureScaling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureScaling::None => "None".fmt(f),
            TextureScaling::Fit => "Fit".fmt(f),
        }
    }
}

/// Scaling, filtering, aspect ratio, etc for the rendered texture.
#[derive(Copy, Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
struct TextureSettings {
    /// Should the aspect ratio of the tensor be kept when scaling?
    keep_aspect_ratio: bool,

    /// Should we scale the texture when rendering?
    scaling: TextureScaling,

    /// Specifies the sampling filter used to render the texture.
    options: egui::TextureOptions,
}

impl Default for TextureSettings {
    fn default() -> Self {
        Self {
            keep_aspect_ratio: true,
            scaling: TextureScaling::default(),
            options: egui::TextureOptions {
                // This is best for low-res depth-images and the like
                magnification: egui::TextureFilter::Nearest,
                minification: egui::TextureFilter::Linear,
            },
        }
    }
}

// helpers
impl TextureSettings {
    fn paint_image(
        &self,
        ui: &mut egui::Ui,
        margin: Vec2,
        image: ColorImage,
    ) -> (egui::Painter, egui::Rect) {
        let img_size = egui::vec2(image.size[0] as _, image.size[1] as _);
        let img_size = Vec2::max(Vec2::splat(1.0), img_size); // better safe than sorry
        let desired_size = match self.scaling {
            TextureScaling::None => img_size + margin,
            TextureScaling::Fit => {
                let desired_size = ui.available_size() - margin;
                if self.keep_aspect_ratio {
                    let scale = (desired_size / img_size).min_elem();
                    img_size * scale
                } else {
                    desired_size
                }
            }
        };

        // TODO(cmc): don't recreate texture unless necessary
        let texture = ui.ctx().load_texture("tensor_slice", image, self.options);

        let (response, painter) = ui.allocate_painter(desired_size, egui::Sense::hover());
        let rect = response.rect;
        let image_rect = egui::Rect::from_min_max(rect.min + margin, rect.max);

        let mut mesh = egui::Mesh::with_texture(texture.id());
        let uv = egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0));
        mesh.add_rect_with_uv(image_rect, uv, Color32::WHITE);

        painter.add(mesh);

        (painter, image_rect)
    }
}

// ui
impl TextureSettings {
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Texture scaling")
                    .selected_text(self.scaling.to_string())
                    .show_ui(ui, |ui| {
                        let mut selectable_value = |ui: &mut egui::Ui, e| {
                            ui.selectable_value(&mut self.scaling, e, e.to_string())
                        };
                        selectable_value(ui, TextureScaling::None);
                        selectable_value(ui, TextureScaling::Fit);
                    });
                ui.checkbox(&mut self.keep_aspect_ratio, "Keep aspect ratio");
            });

            texture_filter_ui(
                ui,
                "Texture magnification filter",
                &mut self.options.magnification,
            );
        });
    }
}

fn texture_filter_ui(ui: &mut egui::Ui, label: &str, filter: &mut egui::TextureFilter) {
    fn tf_to_string(tf: egui::TextureFilter) -> &'static str {
        match tf {
            egui::TextureFilter::Nearest => "Nearest",
            egui::TextureFilter::Linear => "Linear",
        }
    }

    egui::ComboBox::from_label(label)
        .selected_text(tf_to_string(*filter))
        .show_ui(ui, |ui| {
            let mut selectable_value =
                |ui: &mut egui::Ui, e| ui.selectable_value(filter, e, tf_to_string(e));
            selectable_value(ui, egui::TextureFilter::Linear);
            selectable_value(ui, egui::TextureFilter::Nearest);
        });
}

// ----------------------------------------------------------------------------

fn selected_tensor_slice<'a, T: Copy>(
    state: &ViewTensorState,
    tensor: &'a ndarray::ArrayViewD<'_, T>,
) -> ndarray::ArrayViewD<'a, T> {
    let dim_mapping = &state.dimension_mapping;

    assert!(dim_mapping.is_valid(tensor.ndim()));

    // TODO(andreas) - shouldn't just give up here
    if dim_mapping.width.is_none() || dim_mapping.height.is_none() {
        return tensor.view();
    }

    let axis = dim_mapping
        .height
        .into_iter()
        .chain(dim_mapping.width.into_iter())
        .chain(dim_mapping.selectors.iter().copied())
        .collect::<Vec<_>>();
    let mut slice = tensor.view().permuted_axes(axis);

    for dim_idx in &dim_mapping.selectors {
        let selector_value = state
            .selector_values
            .get(dim_idx)
            .copied()
            .unwrap_or_default() as usize;
        assert!(
            selector_value < slice.shape()[2],
            "Bad tensor slicing. Trying to select slice index {selector_value} of dim=2. tensor shape: {:?}, dim_mapping: {dim_mapping:#?}",
            tensor.shape()
        );

        // 0 and 1 are width/height, the rest are rearranged by dimension_mapping.selectors
        // This call removes Axis(2), so the next iteration of the loop does the right thing again.
        slice.index_axis_inplace(Axis(2), selector_value);
    }
    if dim_mapping.invert_height {
        slice.invert_axis(Axis(0));
    }
    if dim_mapping.invert_width {
        slice.invert_axis(Axis(1));
    }

    slice
}

fn slice_ui<T: Copy>(
    ctx: &mut crate::misc::ViewerContext<'_>,
    ui: &mut egui::Ui,
    view_state: &ViewTensorState,
    tensor_shape: &[field_types::TensorDimension],
    slice: ndarray::ArrayViewD<'_, T>,
    color_from_value: impl Fn(T) -> Color32,
) {
    crate::profile_function!();

    let ndims = slice.ndim();
    if let Ok(slice) = slice.into_dimensionality::<Ix2>() {
        let dimension_labels = {
            let dm = &view_state.dimension_mapping;
            [
                (
                    dimension_name(tensor_shape, dm.width.unwrap()),
                    dm.invert_width,
                ),
                (
                    dimension_name(tensor_shape, dm.height.unwrap()),
                    dm.invert_height,
                ),
            ]
        };

        let image = into_image(&slice, color_from_value);
        image_ui(ui, view_state, image, dimension_labels);
    } else {
        ui.label(ctx.re_ui.error_text(format!(
            "Only 2D slices supported at the moment, but slice ndim {ndims}"
        )));
    }
}

fn dimension_name(shape: &[field_types::TensorDimension], dim_idx: usize) -> String {
    let dim = &shape[dim_idx];
    dim.name.as_ref().map_or_else(
        || format!("Dimension {dim_idx} (size={})", dim.size),
        |name| format!("{name} (size={})", dim.size),
    )
}

fn into_image<T: Copy>(
    slice: &ndarray::ArrayView2<'_, T>,
    color_from_value: impl Fn(T) -> Color32,
) -> ColorImage {
    crate::profile_function!();

    use ndarray::Dimension as _;
    let (height, width) = slice.raw_dim().into_pattern();
    let mut image = egui::ColorImage::new([width, height], Color32::DEBUG_COLOR);

    let image_view =
        ndarray::ArrayViewMut2::from_shape(slice.raw_dim(), image.pixels.as_mut_slice())
            .expect("Mismatched length.");

    crate::profile_scope!("color_mapper");
    ndarray::Zip::from(image_view)
        .and(slice)
        .for_each(|pixel, value| {
            *pixel = color_from_value(*value);
        });

    image
}

fn image_ui(
    ui: &mut egui::Ui,
    view_state: &ViewTensorState,
    image: ColorImage,
    dimension_labels: [(String, bool); 2],
) {
    crate::profile_function!();

    egui::ScrollArea::both().show(ui, |ui| {
        let font_id = egui::TextStyle::Body.resolve(ui.style());
        let margin = Vec2::splat(font_id.size + 2.0);

        let (painter, image_rect) = view_state.texture_settings.paint_image(ui, margin, image);

        let [(width_name, invert_width), (height_name, invert_height)] = dimension_labels;
        let text_color = ui.visuals().text_color();

        // Label for X axis, on top:
        if invert_width {
            painter.text(
                image_rect.left_top(),
                Align2::LEFT_BOTTOM,
                format!("{width_name} ⬅"),
                font_id.clone(),
                text_color,
            );
        } else {
            painter.text(
                image_rect.right_top(),
                Align2::RIGHT_BOTTOM,
                format!("➡ {width_name}"),
                font_id.clone(),
                text_color,
            );
        }

        // Label for Y axis, on the left:
        if invert_height {
            let galley = painter.layout_no_wrap(format!("➡ {height_name}"), font_id, text_color);
            painter.add(TextShape {
                pos: image_rect.left_top() - egui::vec2(galley.size().y, -galley.size().x),
                galley,
                angle: -std::f32::consts::TAU / 4.0,
                underline: Default::default(),
                override_text_color: None,
            });
        } else {
            let galley = painter.layout_no_wrap(format!("{height_name} ⬅"), font_id, text_color);
            painter.add(TextShape {
                pos: image_rect.left_bottom() - egui::vec2(galley.size().y, 0.0),
                galley,
                angle: -std::f32::consts::TAU / 4.0,
                underline: Default::default(),
                override_text_color: None,
            });
        }
    });
}

fn selectors_ui(ui: &mut egui::Ui, state: &mut ViewTensorState, tensor: &ClassicTensor) {
    for &dim_idx in &state.dimension_mapping.selectors {
        let dim = &tensor.shape()[dim_idx];
        let size = dim.size;

        let selector_value = state
            .selector_values
            .entry(dim_idx)
            .or_insert_with(|| size / 2); // start in the middle

        if size > 0 {
            *selector_value = selector_value.at_most(size - 1);
        }

        if size > 1 {
            ui.horizontal(|ui| {
                let name = dim.name.as_ref().map_or_else(
                    || format!("dimension {dim_idx}"),
                    |name| format!("{name:?}"),
                );

                ui.weak(format!("Slice selector for {}:", name));

                // If the range is big (say, 2048) then we would need
                // a slider that is 2048 pixels wide to get the good precision.
                // So we add a high-precision drag-value instead:
                ui.add(
                    egui::DragValue::new(selector_value)
                        .clamp_range(0..=size - 1)
                        .speed(0.5),
                )
                .on_hover_text("Drag to precisely control the slice index");

                // Make the slider as big as needed:
                const MIN_SLIDER_WIDTH: f32 = 64.0;
                if ui.available_width() >= MIN_SLIDER_WIDTH {
                    ui.spacing_mut().slider_width = (size as f32 * 2.0)
                        .at_least(MIN_SLIDER_WIDTH)
                        .at_most(ui.available_width());
                    ui.add(egui::Slider::new(selector_value, 0..=size - 1).show_value(false));
                }
            });
        }
    }
}
