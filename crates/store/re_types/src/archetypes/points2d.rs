// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/points2d.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A 2D point cloud with positions and optional colors, radii, labels, etc.
///
/// ## Examples
///
/// ### Randomly distributed 2D points with varying color and radius
/// ```ignore
/// use rand::{distributions::Uniform, Rng as _};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_points2d_random").spawn()?;
///
///     let mut rng = rand::thread_rng();
///     let dist = Uniform::new(-3., 3.);
///
///     rec.log(
///         "random",
///         &rerun::Points2D::new((0..10).map(|_| (rng.sample(dist), rng.sample(dist))))
///             .with_colors((0..10).map(|_| rerun::Color::from_rgb(rng.gen(), rng.gen(), rng.gen())))
///             .with_radii((0..10).map(|_| rng.gen::<f32>())),
///     )?;
///
///     // TODO(#5521): log VisualBounds2D
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/1200w.png">
///   <img src="https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/full.png" width="640">
/// </picture>
/// </center>
///
/// ### Log points with radii given in UI points
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_points2d_ui_radius").spawn()?;
///
///     // Two blue points with scene unit radii of 0.1 and 0.3.
///     rec.log(
///         "scene_units",
///         &rerun::Points2D::new([(0.0, 0.0), (0.0, 1.0)])
///             // By default, radii are interpreted as world-space units.
///             .with_radii([0.1, 0.3])
///             .with_colors([rerun::Color::from_rgb(0, 0, 255)]),
///     )?;
///
///     // Two red points with ui point radii of 40 and 60.
///     // UI points are independent of zooming in Views, but are sensitive to the application UI scaling.
///     // For 100% ui scaling, UI points are equal to pixels.
///     rec.log(
///         "ui_points",
///         &rerun::Points2D::new([(1.0, 0.0), (1.0, 1.0)])
///             // rerun::Radius::new_ui_points produces a radius that the viewer interprets as given in ui points.
///             .with_radii([
///                 rerun::Radius::new_ui_points(40.0),
///                 rerun::Radius::new_ui_points(60.0),
///             ])
///             .with_colors([rerun::Color::from_rgb(255, 0, 0)]),
///     )?;
///
///     // TODO(#5521): log VisualBounds2D
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/point2d_ui_radius/ce804fc77300d89c348b4ab5960395171497b7ac/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/point2d_ui_radius/ce804fc77300d89c348b4ab5960395171497b7ac/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/point2d_ui_radius/ce804fc77300d89c348b4ab5960395171497b7ac/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/point2d_ui_radius/ce804fc77300d89c348b4ab5960395171497b7ac/1200w.png">
///   <img src="https://static.rerun.io/point2d_ui_radius/ce804fc77300d89c348b4ab5960395171497b7ac/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct Points2D {
    /// All the 2D positions at which the point cloud shows points.
    pub positions: Vec<crate::components::Position2D>,

    /// Optional radii for the points, effectively turning them into circles.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the points.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional text labels for the points.
    ///
    /// If there's a single label present, it will be placed at the center of the entity.
    /// Otherwise, each instance will have its own label.
    pub labels: Option<Vec<crate::components::Text>>,

    /// Optional choice of whether the text labels should be shown by default.
    pub show_labels: Option<crate::components::ShowLabels>,

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<crate::components::DrawOrder>,

    /// Optional class Ids for the points.
    ///
    /// The [`components::ClassId`][crate::components::ClassId] provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,

    /// Optional keypoint IDs for the points, identifying them within a class.
    ///
    /// If keypoint IDs are passed in but no [`components::ClassId`][crate::components::ClassId]s were specified, the [`components::ClassId`][crate::components::ClassId] will
    /// default to 0.
    /// This is useful to identify points within a single classification (which is identified
    /// with `class_id`).
    /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
    /// detected skeleton.
    pub keypoint_ids: Option<Vec<crate::components::KeypointId>>,
}

impl Points2D {
    /// Returns the [`ComponentDescriptor`] for [`Self::positions`].
    #[inline]
    pub fn descriptor_positions() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Points2D".into()),
            component_name: "rerun.components.Position2D".into(),
            archetype_field_name: Some("positions".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::radii`].
    #[inline]
    pub fn descriptor_radii() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Points2D".into()),
            component_name: "rerun.components.Radius".into(),
            archetype_field_name: Some("radii".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::colors`].
    #[inline]
    pub fn descriptor_colors() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Points2D".into()),
            component_name: "rerun.components.Color".into(),
            archetype_field_name: Some("colors".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::labels`].
    #[inline]
    pub fn descriptor_labels() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Points2D".into()),
            component_name: "rerun.components.Text".into(),
            archetype_field_name: Some("labels".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::show_labels`].
    #[inline]
    pub fn descriptor_show_labels() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Points2D".into()),
            component_name: "rerun.components.ShowLabels".into(),
            archetype_field_name: Some("show_labels".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::draw_order`].
    #[inline]
    pub fn descriptor_draw_order() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Points2D".into()),
            component_name: "rerun.components.DrawOrder".into(),
            archetype_field_name: Some("draw_order".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::class_ids`].
    #[inline]
    pub fn descriptor_class_ids() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Points2D".into()),
            component_name: "rerun.components.ClassId".into(),
            archetype_field_name: Some("class_ids".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::keypoint_ids`].
    #[inline]
    pub fn descriptor_keypoint_ids() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Points2D".into()),
            component_name: "rerun.components.KeypointId".into(),
            archetype_field_name: Some("keypoint_ids".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Points2D".into()),
            component_name: "rerun.components.Points2DIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Points2D::descriptor_positions()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Points2D::descriptor_radii(),
            Points2D::descriptor_colors(),
            Points2D::descriptor_indicator(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Points2D::descriptor_labels(),
            Points2D::descriptor_show_labels(),
            Points2D::descriptor_draw_order(),
            Points2D::descriptor_class_ids(),
            Points2D::descriptor_keypoint_ids(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 9usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Points2D::descriptor_positions(),
            Points2D::descriptor_radii(),
            Points2D::descriptor_colors(),
            Points2D::descriptor_indicator(),
            Points2D::descriptor_labels(),
            Points2D::descriptor_show_labels(),
            Points2D::descriptor_draw_order(),
            Points2D::descriptor_class_ids(),
            Points2D::descriptor_keypoint_ids(),
        ]
    });

impl Points2D {
    /// The total number of components in the archetype: 1 required, 3 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 9usize;
}

/// Indicator component for the [`Points2D`] [`::re_types_core::Archetype`]
pub type Points2DIndicator = ::re_types_core::GenericIndicatorComponent<Points2D>;

impl ::re_types_core::Archetype for Points2D {
    type Indicator = Points2DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Points2D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Points 2D"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: Points2DIndicator = Points2DIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let positions = {
            let array = arrays_by_name
                .get("rerun.components.Position2D")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Points2D#positions")?;
            <crate::components::Position2D>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Points2D#positions")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Points2D#positions")?
        };
        let radii = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            Some({
                <crate::components::Radius>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#radii")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#colors")?
            })
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            Some({
                <crate::components::Text>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#labels")?
            })
        } else {
            None
        };
        let show_labels = if let Some(array) = arrays_by_name.get("rerun.components.ShowLabels") {
            <crate::components::ShowLabels>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Points2D#show_labels")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let draw_order = if let Some(array) = arrays_by_name.get("rerun.components.DrawOrder") {
            <crate::components::DrawOrder>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Points2D#draw_order")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("rerun.components.ClassId") {
            Some({
                <crate::components::ClassId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#class_ids")?
            })
        } else {
            None
        };
        let keypoint_ids = if let Some(array) = arrays_by_name.get("rerun.components.KeypointId") {
            Some({
                <crate::components::KeypointId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#keypoint_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#keypoint_ids")?
            })
        } else {
            None
        };
        Ok(Self {
            positions,
            radii,
            colors,
            labels,
            show_labels,
            draw_order,
            class_ids,
            keypoint_ids,
        })
    }
}

impl ::re_types_core::AsComponents for Points2D {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.positions as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(Self::descriptor_positions()),
                }
            }),
            (self
                .radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_radii()),
            }),
            (self
                .colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_colors()),
            }),
            (self
                .labels
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_labels()),
            }),
            (self
                .show_labels
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_show_labels()),
            }),
            (self
                .draw_order
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_draw_order()),
            }),
            (self
                .class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_class_ids()),
            }),
            (self
                .keypoint_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_keypoint_ids()),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Points2D {}

impl Points2D {
    /// Create a new `Points2D`.
    #[inline]
    pub fn new(
        positions: impl IntoIterator<Item = impl Into<crate::components::Position2D>>,
    ) -> Self {
        Self {
            positions: positions.into_iter().map(Into::into).collect(),
            radii: None,
            colors: None,
            labels: None,
            show_labels: None,
            draw_order: None,
            class_ids: None,
            keypoint_ids: None,
        }
    }

    /// Optional radii for the points, effectively turning them into circles.
    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    /// Optional colors for the points.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    /// Optional text labels for the points.
    ///
    /// If there's a single label present, it will be placed at the center of the entity.
    /// Otherwise, each instance will have its own label.
    #[inline]
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    /// Optional choice of whether the text labels should be shown by default.
    #[inline]
    pub fn with_show_labels(
        mut self,
        show_labels: impl Into<crate::components::ShowLabels>,
    ) -> Self {
        self.show_labels = Some(show_labels.into());
        self
    }

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    #[inline]
    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = Some(draw_order.into());
        self
    }

    /// Optional class Ids for the points.
    ///
    /// The [`components::ClassId`][crate::components::ClassId] provides colors and labels if not specified explicitly.
    #[inline]
    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Optional keypoint IDs for the points, identifying them within a class.
    ///
    /// If keypoint IDs are passed in but no [`components::ClassId`][crate::components::ClassId]s were specified, the [`components::ClassId`][crate::components::ClassId] will
    /// default to 0.
    /// This is useful to identify points within a single classification (which is identified
    /// with `class_id`).
    /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
    /// detected skeleton.
    #[inline]
    pub fn with_keypoint_ids(
        mut self,
        keypoint_ids: impl IntoIterator<Item = impl Into<crate::components::KeypointId>>,
    ) -> Self {
        self.keypoint_ids = Some(keypoint_ids.into_iter().map(Into::into).collect());
        self
    }
}

impl ::re_byte_size::SizeBytes for Points2D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.positions.heap_size_bytes()
            + self.radii.heap_size_bytes()
            + self.colors.heap_size_bytes()
            + self.labels.heap_size_bytes()
            + self.show_labels.heap_size_bytes()
            + self.draw_order.heap_size_bytes()
            + self.class_ids.heap_size_bytes()
            + self.keypoint_ids.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::Position2D>>::is_pod()
            && <Option<Vec<crate::components::Radius>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
            && <Option<Vec<crate::components::Text>>>::is_pod()
            && <Option<crate::components::ShowLabels>>::is_pod()
            && <Option<crate::components::DrawOrder>>::is_pod()
            && <Option<Vec<crate::components::ClassId>>>::is_pod()
            && <Option<Vec<crate::components::KeypointId>>>::is_pod()
    }
}
