# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/components/background_kind.fbs".

# You can extend this class by creating a "BackgroundKindExt" class in "background_kind_ext.py".

from __future__ import annotations

from typing import Literal, Sequence, Union

import pyarrow as pa

from ..._baseclasses import (
    BaseBatch,
    BaseExtensionType,
    ComponentBatchMixin,
)

__all__ = [
    "BackgroundKind",
    "BackgroundKindArrayLike",
    "BackgroundKindBatch",
    "BackgroundKindLike",
    "BackgroundKindType",
]


from enum import Enum


class BackgroundKind(Enum):
    """**Component**: The type of the background in a view."""

    GradientDark = 1
    """
    A dark gradient.

    In 3D views it changes depending on the direction of the view.
    """

    GradientBright = 2
    """
    A bright gradient.

    In 3D views it changes depending on the direction of the view.
    """

    SolidColor = 3
    """Simple uniform color."""


BackgroundKindLike = Union[BackgroundKind, Literal["gradientdark"] | Literal["gradientbright"] | Literal["solidcolor"]]
BackgroundKindArrayLike = Union[BackgroundKindLike, Sequence[BackgroundKindLike]]


class BackgroundKindType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.blueprint.components.BackgroundKind"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.sparse_union([
                pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                pa.field("GradientDark", pa.null(), nullable=True, metadata={}),
                pa.field("GradientBright", pa.null(), nullable=True, metadata={}),
                pa.field("SolidColor", pa.null(), nullable=True, metadata={}),
            ]),
            self._TYPE_NAME,
        )


class BackgroundKindBatch(BaseBatch[BackgroundKindArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = BackgroundKindType()

    @staticmethod
    def _native_to_pa_array(data: BackgroundKindArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (BackgroundKind, int, str)):
            data = [data]

        types: list[int] = []

        for value in data:
            if value is None:
                types.append(0)
            elif isinstance(value, BackgroundKind):
                types.append(value.value)  # Actual enum value
            elif isinstance(value, int):
                types.append(value)  # By number
            elif isinstance(value, str):
                if hasattr(BackgroundKind, value):
                    types.append(BackgroundKind[value].value)  # fast path
                elif value.lower() == "gradientdark":
                    types.append(BackgroundKind.GradientDark.value)
                elif value.lower() == "gradientbright":
                    types.append(BackgroundKind.GradientBright.value)
                elif value.lower() == "solidcolor":
                    types.append(BackgroundKind.SolidColor.value)
                else:
                    raise ValueError(f"Unknown BackgroundKind kind: {value}")
            else:
                raise ValueError(f"Unknown BackgroundKind kind: {value}")

        buffers = [
            None,
            pa.array(types, type=pa.int8()).buffers()[1],
        ]
        children = (1 + 3) * [pa.nulls(len(data))]

        return pa.UnionArray.from_buffers(
            type=data_type,
            length=len(data),
            buffers=buffers,
            children=children,
        )
