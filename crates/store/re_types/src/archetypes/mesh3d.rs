// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/mesh3d.fbs".

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

/// **Archetype**: A 3D triangle mesh as specified by its per-mesh and per-vertex properties.
///
/// See also [`archetypes::Asset3D`][crate::archetypes::Asset3D].
///
/// If there are multiple [`archetypes::InstancePoses3D`][crate::archetypes::InstancePoses3D] instances logged to the same entity as a mesh,
/// an instance of the mesh will be drawn for each transform.
///
/// ## Examples
///
/// ### Simple indexed 3D mesh
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_mesh3d_indexed").spawn()?;
///
///     rec.log(
///         "triangle",
///         &rerun::Mesh3D::new([[0.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
///             .with_vertex_normals([[0.0, 0.0, 1.0]])
///             .with_vertex_colors([0x0000FFFF, 0x00FF00FF, 0xFF0000FF])
///             .with_triangle_indices([[2, 1, 0]]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/mesh3d_indexed/57c70dc992e6dc0bd9c5222ca084f5b6240cea75/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/mesh3d_indexed/57c70dc992e6dc0bd9c5222ca084f5b6240cea75/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/mesh3d_indexed/57c70dc992e6dc0bd9c5222ca084f5b6240cea75/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/mesh3d_indexed/57c70dc992e6dc0bd9c5222ca084f5b6240cea75/1200w.png">
///   <img src="https://static.rerun.io/mesh3d_indexed/57c70dc992e6dc0bd9c5222ca084f5b6240cea75/full.png" width="640">
/// </picture>
/// </center>
///
/// ### 3D mesh with instancing
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_mesh3d_instancing").spawn()?;
///
///     rec.set_time_sequence("frame", 0);
///     rec.log(
///         "shape",
///         &rerun::Mesh3D::new([
///             [1.0, 1.0, 1.0],
///             [-1.0, -1.0, 1.0],
///             [-1.0, 1.0, -1.0],
///             [1.0, -1.0, -1.0],
///         ])
///         .with_triangle_indices([[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]])
///         .with_vertex_colors([0xFF0000FF, 0x00FF00FF, 0x00000FFFF, 0xFFFF00FF]),
///     )?;
///     // This box will not be affected by its parent's instance poses!
///     rec.log(
///         "shape/box",
///         &rerun::Boxes3D::from_half_sizes([[5.0, 5.0, 5.0]]),
///     )?;
///
///     for i in 0..100 {
///         rec.set_time_sequence("frame", i);
///         rec.log(
///             "shape",
///             &rerun::InstancePoses3D::new()
///                 .with_translations([
///                     [2.0, 0.0, 0.0],
///                     [0.0, 2.0, 0.0],
///                     [0.0, -2.0, 0.0],
///                     [-2.0, 0.0, 0.0],
///                 ])
///                 .with_rotation_axis_angles([rerun::RotationAxisAngle::new(
///                     [0.0, 0.0, 1.0],
///                     rerun::Angle::from_degrees(i as f32 * 2.0),
///                 )]),
///         )?;
///     }
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/mesh3d_leaf_transforms3d/c2d0ee033129da53168f5705625a9b033f3a3d61/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/mesh3d_leaf_transforms3d/c2d0ee033129da53168f5705625a9b033f3a3d61/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/mesh3d_leaf_transforms3d/c2d0ee033129da53168f5705625a9b033f3a3d61/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/mesh3d_leaf_transforms3d/c2d0ee033129da53168f5705625a9b033f3a3d61/1200w.png">
///   <img src="https://static.rerun.io/mesh3d_leaf_transforms3d/c2d0ee033129da53168f5705625a9b033f3a3d61/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct Mesh3D {
    /// The positions of each vertex.
    ///
    /// If no `triangle_indices` are specified, then each triplet of positions is interpreted as a triangle.
    pub vertex_positions: Vec<crate::components::Position3D>,

    /// Optional indices for the triangles that make up the mesh.
    pub triangle_indices: Option<Vec<crate::components::TriangleIndices>>,

    /// An optional normal for each vertex.
    pub vertex_normals: Option<Vec<crate::components::Vector3D>>,

    /// An optional color for each vertex.
    pub vertex_colors: Option<Vec<crate::components::Color>>,

    /// An optional uv texture coordinate for each vertex.
    pub vertex_texcoords: Option<Vec<crate::components::Texcoord2D>>,

    /// A color multiplier applied to the whole mesh.
    pub albedo_factor: Option<crate::components::AlbedoFactor>,

    /// Optional albedo texture.
    ///
    /// Used with the [`components::Texcoord2D`][crate::components::Texcoord2D] of the mesh.
    ///
    /// Currently supports only sRGB(A) textures, ignoring alpha.
    /// (meaning that the tensor must have 3 or 4 channels and use the `u8` format)
    pub albedo_texture_buffer: Option<crate::components::ImageBuffer>,

    /// The format of the `albedo_texture_buffer`, if any.
    pub albedo_texture_format: Option<crate::components::ImageFormat>,

    /// Optional class Ids for the vertices.
    ///
    /// The [`components::ClassId`][crate::components::ClassId] provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,
}

impl Mesh3D {
    /// Returns the [`ComponentDescriptor`] for [`Self::vertex_positions`].
    #[inline]
    pub fn descriptor_vertex_positions() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Mesh3D".into()),
            component_name: "rerun.components.Position3D".into(),
            archetype_field_name: Some("vertex_positions".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::triangle_indices`].
    #[inline]
    pub fn descriptor_triangle_indices() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Mesh3D".into()),
            component_name: "rerun.components.TriangleIndices".into(),
            archetype_field_name: Some("triangle_indices".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::vertex_normals`].
    #[inline]
    pub fn descriptor_vertex_normals() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Mesh3D".into()),
            component_name: "rerun.components.Vector3D".into(),
            archetype_field_name: Some("vertex_normals".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::vertex_colors`].
    #[inline]
    pub fn descriptor_vertex_colors() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Mesh3D".into()),
            component_name: "rerun.components.Color".into(),
            archetype_field_name: Some("vertex_colors".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::vertex_texcoords`].
    #[inline]
    pub fn descriptor_vertex_texcoords() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Mesh3D".into()),
            component_name: "rerun.components.Texcoord2D".into(),
            archetype_field_name: Some("vertex_texcoords".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::albedo_factor`].
    #[inline]
    pub fn descriptor_albedo_factor() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Mesh3D".into()),
            component_name: "rerun.components.AlbedoFactor".into(),
            archetype_field_name: Some("albedo_factor".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::albedo_texture_buffer`].
    #[inline]
    pub fn descriptor_albedo_texture_buffer() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Mesh3D".into()),
            component_name: "rerun.components.ImageBuffer".into(),
            archetype_field_name: Some("albedo_texture_buffer".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::albedo_texture_format`].
    #[inline]
    pub fn descriptor_albedo_texture_format() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Mesh3D".into()),
            component_name: "rerun.components.ImageFormat".into(),
            archetype_field_name: Some("albedo_texture_format".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::class_ids`].
    #[inline]
    pub fn descriptor_class_ids() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Mesh3D".into()),
            component_name: "rerun.components.ClassId".into(),
            archetype_field_name: Some("class_ids".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Mesh3D".into()),
            component_name: "rerun.components.Mesh3DIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Mesh3D::descriptor_vertex_positions()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Mesh3D::descriptor_triangle_indices(),
            Mesh3D::descriptor_vertex_normals(),
            Mesh3D::descriptor_indicator(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Mesh3D::descriptor_vertex_colors(),
            Mesh3D::descriptor_vertex_texcoords(),
            Mesh3D::descriptor_albedo_factor(),
            Mesh3D::descriptor_albedo_texture_buffer(),
            Mesh3D::descriptor_albedo_texture_format(),
            Mesh3D::descriptor_class_ids(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 10usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Mesh3D::descriptor_vertex_positions(),
            Mesh3D::descriptor_triangle_indices(),
            Mesh3D::descriptor_vertex_normals(),
            Mesh3D::descriptor_indicator(),
            Mesh3D::descriptor_vertex_colors(),
            Mesh3D::descriptor_vertex_texcoords(),
            Mesh3D::descriptor_albedo_factor(),
            Mesh3D::descriptor_albedo_texture_buffer(),
            Mesh3D::descriptor_albedo_texture_format(),
            Mesh3D::descriptor_class_ids(),
        ]
    });

impl Mesh3D {
    /// The total number of components in the archetype: 1 required, 3 recommended, 6 optional
    pub const NUM_COMPONENTS: usize = 10usize;
}

/// Indicator component for the [`Mesh3D`] [`::re_types_core::Archetype`]
pub type Mesh3DIndicator = ::re_types_core::GenericIndicatorComponent<Mesh3D>;

impl ::re_types_core::Archetype for Mesh3D {
    type Indicator = Mesh3DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Mesh3D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Mesh 3D"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: Mesh3DIndicator = Mesh3DIndicator::DEFAULT;
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
        let vertex_positions = {
            let array = arrays_by_name
                .get("rerun.components.Position3D")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Mesh3D#vertex_positions")?;
            <crate::components::Position3D>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Mesh3D#vertex_positions")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Mesh3D#vertex_positions")?
        };
        let triangle_indices =
            if let Some(array) = arrays_by_name.get("rerun.components.TriangleIndices") {
                Some({
                    <crate::components::TriangleIndices>::from_arrow_opt(&**array)
                        .with_context("rerun.archetypes.Mesh3D#triangle_indices")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.archetypes.Mesh3D#triangle_indices")?
                })
            } else {
                None
            };
        let vertex_normals = if let Some(array) = arrays_by_name.get("rerun.components.Vector3D") {
            Some({
                <crate::components::Vector3D>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Mesh3D#vertex_normals")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Mesh3D#vertex_normals")?
            })
        } else {
            None
        };
        let vertex_colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Mesh3D#vertex_colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Mesh3D#vertex_colors")?
            })
        } else {
            None
        };
        let vertex_texcoords =
            if let Some(array) = arrays_by_name.get("rerun.components.Texcoord2D") {
                Some({
                    <crate::components::Texcoord2D>::from_arrow_opt(&**array)
                        .with_context("rerun.archetypes.Mesh3D#vertex_texcoords")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.archetypes.Mesh3D#vertex_texcoords")?
                })
            } else {
                None
            };
        let albedo_factor = if let Some(array) = arrays_by_name.get("rerun.components.AlbedoFactor")
        {
            <crate::components::AlbedoFactor>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Mesh3D#albedo_factor")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let albedo_texture_buffer =
            if let Some(array) = arrays_by_name.get("rerun.components.ImageBuffer") {
                <crate::components::ImageBuffer>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Mesh3D#albedo_texture_buffer")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let albedo_texture_format =
            if let Some(array) = arrays_by_name.get("rerun.components.ImageFormat") {
                <crate::components::ImageFormat>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Mesh3D#albedo_texture_format")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let class_ids = if let Some(array) = arrays_by_name.get("rerun.components.ClassId") {
            Some({
                <crate::components::ClassId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Mesh3D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Mesh3D#class_ids")?
            })
        } else {
            None
        };
        Ok(Self {
            vertex_positions,
            triangle_indices,
            vertex_normals,
            vertex_colors,
            vertex_texcoords,
            albedo_factor,
            albedo_texture_buffer,
            albedo_texture_format,
            class_ids,
        })
    }
}

impl ::re_types_core::AsComponents for Mesh3D {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.vertex_positions as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(Self::descriptor_vertex_positions()),
                }
            }),
            (self
                .triangle_indices
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_triangle_indices()),
            }),
            (self
                .vertex_normals
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_vertex_normals()),
            }),
            (self
                .vertex_colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_vertex_colors()),
            }),
            (self
                .vertex_texcoords
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_vertex_texcoords()),
            }),
            (self
                .albedo_factor
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_albedo_factor()),
            }),
            (self
                .albedo_texture_buffer
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_albedo_texture_buffer()),
            }),
            (self
                .albedo_texture_format
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_albedo_texture_format()),
            }),
            (self
                .class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_class_ids()),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Mesh3D {}

impl Mesh3D {
    /// Create a new `Mesh3D`.
    #[inline]
    pub fn new(
        vertex_positions: impl IntoIterator<Item = impl Into<crate::components::Position3D>>,
    ) -> Self {
        Self {
            vertex_positions: vertex_positions.into_iter().map(Into::into).collect(),
            triangle_indices: None,
            vertex_normals: None,
            vertex_colors: None,
            vertex_texcoords: None,
            albedo_factor: None,
            albedo_texture_buffer: None,
            albedo_texture_format: None,
            class_ids: None,
        }
    }

    /// Optional indices for the triangles that make up the mesh.
    #[inline]
    pub fn with_triangle_indices(
        mut self,
        triangle_indices: impl IntoIterator<Item = impl Into<crate::components::TriangleIndices>>,
    ) -> Self {
        self.triangle_indices = Some(triangle_indices.into_iter().map(Into::into).collect());
        self
    }

    /// An optional normal for each vertex.
    #[inline]
    pub fn with_vertex_normals(
        mut self,
        vertex_normals: impl IntoIterator<Item = impl Into<crate::components::Vector3D>>,
    ) -> Self {
        self.vertex_normals = Some(vertex_normals.into_iter().map(Into::into).collect());
        self
    }

    /// An optional color for each vertex.
    #[inline]
    pub fn with_vertex_colors(
        mut self,
        vertex_colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.vertex_colors = Some(vertex_colors.into_iter().map(Into::into).collect());
        self
    }

    /// An optional uv texture coordinate for each vertex.
    #[inline]
    pub fn with_vertex_texcoords(
        mut self,
        vertex_texcoords: impl IntoIterator<Item = impl Into<crate::components::Texcoord2D>>,
    ) -> Self {
        self.vertex_texcoords = Some(vertex_texcoords.into_iter().map(Into::into).collect());
        self
    }

    /// A color multiplier applied to the whole mesh.
    #[inline]
    pub fn with_albedo_factor(
        mut self,
        albedo_factor: impl Into<crate::components::AlbedoFactor>,
    ) -> Self {
        self.albedo_factor = Some(albedo_factor.into());
        self
    }

    /// Optional albedo texture.
    ///
    /// Used with the [`components::Texcoord2D`][crate::components::Texcoord2D] of the mesh.
    ///
    /// Currently supports only sRGB(A) textures, ignoring alpha.
    /// (meaning that the tensor must have 3 or 4 channels and use the `u8` format)
    #[inline]
    pub fn with_albedo_texture_buffer(
        mut self,
        albedo_texture_buffer: impl Into<crate::components::ImageBuffer>,
    ) -> Self {
        self.albedo_texture_buffer = Some(albedo_texture_buffer.into());
        self
    }

    /// The format of the `albedo_texture_buffer`, if any.
    #[inline]
    pub fn with_albedo_texture_format(
        mut self,
        albedo_texture_format: impl Into<crate::components::ImageFormat>,
    ) -> Self {
        self.albedo_texture_format = Some(albedo_texture_format.into());
        self
    }

    /// Optional class Ids for the vertices.
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
}

impl ::re_byte_size::SizeBytes for Mesh3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.vertex_positions.heap_size_bytes()
            + self.triangle_indices.heap_size_bytes()
            + self.vertex_normals.heap_size_bytes()
            + self.vertex_colors.heap_size_bytes()
            + self.vertex_texcoords.heap_size_bytes()
            + self.albedo_factor.heap_size_bytes()
            + self.albedo_texture_buffer.heap_size_bytes()
            + self.albedo_texture_format.heap_size_bytes()
            + self.class_ids.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::Position3D>>::is_pod()
            && <Option<Vec<crate::components::TriangleIndices>>>::is_pod()
            && <Option<Vec<crate::components::Vector3D>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
            && <Option<Vec<crate::components::Texcoord2D>>>::is_pod()
            && <Option<crate::components::AlbedoFactor>>::is_pod()
            && <Option<crate::components::ImageBuffer>>::is_pod()
            && <Option<crate::components::ImageFormat>>::is_pod()
            && <Option<Vec<crate::components::ClassId>>>::is_pod()
    }
}
