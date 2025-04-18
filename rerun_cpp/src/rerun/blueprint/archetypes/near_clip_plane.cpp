// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/near_clip_plane.fbs".

#include "near_clip_plane.hpp"

#include "../../collection_adapter_builtins.hpp"

namespace rerun::blueprint::archetypes {
    NearClipPlane NearClipPlane::clear_fields() {
        auto archetype = NearClipPlane();
        archetype.near_clip_plane =
            ComponentBatch::empty<rerun::blueprint::components::NearClipPlane>(
                Descriptor_near_clip_plane
            )
                .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> NearClipPlane::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(2);
        if (near_clip_plane.has_value()) {
            columns.push_back(near_clip_plane.value().partitioned(lengths_).value_or_throw());
        }
        columns.push_back(
            ComponentColumn::from_indicators<NearClipPlane>(static_cast<uint32_t>(lengths_.size()))
                .value_or_throw()
        );
        return columns;
    }

    Collection<ComponentColumn> NearClipPlane::columns() {
        if (near_clip_plane.has_value()) {
            return columns(std::vector<uint32_t>(near_clip_plane.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::blueprint::archetypes

namespace rerun {

    Result<Collection<ComponentBatch>>
        AsComponents<blueprint::archetypes::NearClipPlane>::as_batches(
            const blueprint::archetypes::NearClipPlane& archetype
        ) {
        using namespace blueprint::archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(2);

        if (archetype.near_clip_plane.has_value()) {
            cells.push_back(archetype.near_clip_plane.value());
        }
        {
            auto result = ComponentBatch::from_indicator<NearClipPlane>();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return rerun::take_ownership(std::move(cells));
    }
} // namespace rerun
