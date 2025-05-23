from __future__ import annotations

import os
from argparse import Namespace
from uuid import uuid4

import rerun as rr
import rerun.blueprint as rrb

README = """\
# Context Menu - Invalid sub-container kind


* Single-select the horizontal container, check that it disallow adding a horizontal container inside it.
* Same for the vertical container.
* Single select a view inside a horizontal container, check that it disallow moving to a new horizontal container.
* Same for a view inside a vertical container.
"""


def log_readme() -> None:
    rr.log("readme", rr.TextDocument(README, media_type=rr.MediaType.MARKDOWN), static=True)


def blueprint() -> rrb.BlueprintLike:
    return rrb.Horizontal(
        rrb.TextDocumentView(origin="readme"),
        rrb.Grid(
            rrb.Vertical(
                rrb.Spatial3DView(origin="/"),
            ),
            rrb.Horizontal(
                rrb.Spatial2DView(origin="/"),
            ),
            grid_columns=1,
        ),
        column_shares=[2, 1],
    )


def log_some_views() -> None:
    rr.set_time("frame_nr", sequence=0)

    rr.log(
        "boxes3d",
        rr.Boxes3D(centers=[[0.0, 0.0, 0.0], [1.0, 1.5, 1.15], [3.0, 2.0, 1.0]], half_sizes=[0.5, 1.0, 0.5] * 3),
    )
    rr.log("boxes2d", rr.Boxes2D(centers=[[0.0, 0.0], [1.3, 0.5], [3.0, 2.0]], half_sizes=[0.5, 1.0] * 3))


def run(args: Namespace) -> None:
    rr.script_setup(args, f"{os.path.basename(__file__)}", recording_id=uuid4())
    rr.send_blueprint(blueprint(), make_active=True, make_default=True)

    log_readme()
    log_some_views()


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Interactive release checklist")
    rr.script_add_args(parser)
    args = parser.parse_args()
    run(args)
