// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/visible_time_range.fbs".

#include "time_range_boundary.hpp"

#include "time_int.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::datatypes {}

namespace rerun {
    const std::shared_ptr<arrow::DataType>& Loggable<datatypes::TimeRangeBoundary>::arrow_datatype(
    ) {
        static const auto datatype = arrow::dense_union({
            arrow::field("_null_markers", arrow::null(), true, nullptr),
            arrow::field(
                "CursorRelative",
                Loggable<rerun::datatypes::TimeInt>::arrow_datatype(),
                false
            ),
            arrow::field("Absolute", Loggable<rerun::datatypes::TimeInt>::arrow_datatype(), false),
            arrow::field("Infinite", arrow::null(), true),
        });
        return datatype;
    }

    Result<std::shared_ptr<arrow::Array>> Loggable<datatypes::TimeRangeBoundary>::to_arrow(
        const datatypes::TimeRangeBoundary* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();
        auto datatype = arrow_datatype();

        ARROW_ASSIGN_OR_RAISE(auto builder, arrow::MakeBuilder(datatype, pool))
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(Loggable<datatypes::TimeRangeBoundary>::fill_arrow_array_builder(
                static_cast<arrow::DenseUnionBuilder*>(builder.get()),
                instances,
                num_instances
            ));
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));
        return array;
    }

    rerun::Error Loggable<datatypes::TimeRangeBoundary>::fill_arrow_array_builder(
        arrow::DenseUnionBuilder* builder, const datatypes::TimeRangeBoundary* elements,
        size_t num_elements
    ) {
        if (builder == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
        }
        if (elements == nullptr) {
            return rerun::Error(
                ErrorCode::UnexpectedNullArgument,
                "Cannot serialize null pointer to arrow array."
            );
        }

        ARROW_RETURN_NOT_OK(builder->Reserve(static_cast<int64_t>(num_elements)));
        for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
            const auto& union_instance = elements[elem_idx];
            ARROW_RETURN_NOT_OK(builder->Append(static_cast<int8_t>(union_instance.get_union_tag()))
            );

            auto variant_index = static_cast<int>(union_instance.get_union_tag());
            auto variant_builder_untyped = builder->child_builder(variant_index).get();

            using TagType = datatypes::detail::TimeRangeBoundaryTag;
            switch (union_instance.get_union_tag()) {
                case TagType::None: {
                    ARROW_RETURN_NOT_OK(variant_builder_untyped->AppendNull());
                } break;
                case TagType::CursorRelative: {
                    auto variant_builder =
                        static_cast<arrow::Int64Builder*>(variant_builder_untyped);
                    RR_RETURN_NOT_OK(Loggable<rerun::datatypes::TimeInt>::fill_arrow_array_builder(
                        variant_builder,
                        &union_instance.get_union_data().cursor_relative,
                        1
                    ));
                } break;
                case TagType::Absolute: {
                    auto variant_builder =
                        static_cast<arrow::Int64Builder*>(variant_builder_untyped);
                    RR_RETURN_NOT_OK(Loggable<rerun::datatypes::TimeInt>::fill_arrow_array_builder(
                        variant_builder,
                        &union_instance.get_union_data().absolute,
                        1
                    ));
                } break;
                case TagType::Infinite: {
                    auto variant_builder =
                        static_cast<arrow::NullBuilder*>(variant_builder_untyped);
                    ARROW_RETURN_NOT_OK(variant_builder->AppendNull());
                } break;
                default:
                    assert(false && "unreachable");
            }
        }

        return Error::ok();
    }
} // namespace rerun
