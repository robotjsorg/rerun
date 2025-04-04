// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/scalar.fbs".

#include "scalar.hpp"

#include "../collection_adapter_builtins.hpp"

RR_PUSH_WARNINGS
RR_DISABLE_DEPRECATION_WARNING

namespace rerun::archetypes {
    Scalar Scalar::clear_fields() {
        auto archetype = Scalar();
        archetype.scalar =
            ComponentBatch::empty<rerun::components::Scalar>(Descriptor_scalar).value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> Scalar::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(2);
        if (scalar.has_value()) {
            columns.push_back(scalar.value().partitioned(lengths_).value_or_throw());
        }
        columns.push_back(
            ComponentColumn::from_indicators<Scalar>(static_cast<uint32_t>(lengths_.size()))
                .value_or_throw()
        );
        return columns;
    }

    Collection<ComponentColumn> Scalar::columns() {
        if (scalar.has_value()) {
            return columns(std::vector<uint32_t>(scalar.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<Collection<ComponentBatch>> AsComponents<archetypes::Scalar>::as_batches(
        const archetypes::Scalar& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(2);

        if (archetype.scalar.has_value()) {
            cells.push_back(archetype.scalar.value());
        }
        {
            auto result = ComponentBatch::from_indicator<Scalar>();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return rerun::take_ownership(std::move(cells));
    }
} // namespace rerun

RR_POP_WARNINGS
