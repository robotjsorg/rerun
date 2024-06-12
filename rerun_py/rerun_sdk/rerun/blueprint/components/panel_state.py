# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/components/panel_state.fbs".

# You can extend this class by creating a "PanelStateExt" class in "panel_state_ext.py".

from __future__ import annotations

from typing import Literal, Sequence, Union

import pyarrow as pa

from ..._baseclasses import (
    BaseBatch,
    BaseExtensionType,
    ComponentBatchMixin,
)

__all__ = ["PanelState", "PanelStateArrayLike", "PanelStateBatch", "PanelStateLike", "PanelStateType"]


from enum import Enum


class PanelState(Enum):
    """**Component**: Tri-state for panel controls."""

    Hidden = 1
    """Completely hidden."""

    Collapsed = 2
    """Visible, but as small as possible on its shorter axis."""

    Expanded = 3
    """Fully expanded."""


PanelStateLike = Union[PanelState, Literal["hidden"] | Literal["collapsed"] | Literal["expanded"]]
PanelStateArrayLike = Union[PanelStateLike, Sequence[PanelStateLike]]


class PanelStateType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.blueprint.components.PanelState"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.sparse_union([
                pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                pa.field("Hidden", pa.null(), nullable=True, metadata={}),
                pa.field("Collapsed", pa.null(), nullable=True, metadata={}),
                pa.field("Expanded", pa.null(), nullable=True, metadata={}),
            ]),
            self._TYPE_NAME,
        )


class PanelStateBatch(BaseBatch[PanelStateArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = PanelStateType()

    @staticmethod
    def _native_to_pa_array(data: PanelStateArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (PanelState, int, str)):
            data = [data]

        types: list[int] = []

        for value in data:
            if value is None:
                types.append(0)
            elif isinstance(value, PanelState):
                types.append(value.value)  # Actual enum value
            elif isinstance(value, int):
                types.append(value)  # By number
            elif isinstance(value, str):
                if hasattr(PanelState, value):
                    types.append(PanelState[value].value)  # fast path
                elif value.lower() == "hidden":
                    types.append(PanelState.Hidden.value)
                elif value.lower() == "collapsed":
                    types.append(PanelState.Collapsed.value)
                elif value.lower() == "expanded":
                    types.append(PanelState.Expanded.value)
                else:
                    raise ValueError(f"Unknown PanelState kind: {value}")
            else:
                raise ValueError(f"Unknown PanelState kind: {value}")

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
