// Copyright 2023 The Vello authors
// SPDX-License-Identifier: Apache-2.0 OR MIT OR Unlicense

use vello_encoding::{ConfigUniform, Path, Tile};

use crate::cpu_dispatch::CpuBinding;

fn backdrop_main(config: &ConfigUniform, paths: &[Path], tiles: &mut [Tile]) {
    for drawobj_ix in 0..config.layout.n_draw_objects {
        let path = paths[drawobj_ix as usize];
        let width = path.bbox[2] - path.bbox[0];
        let height = path.bbox[3] - path.bbox[1];
        let base = path.tiles;
        for y in 0..height {
            let mut sum = 0;
            for x in 0..width {
                let tile = &mut tiles[(base + y * width + x) as usize];
                sum += tile.backdrop;
                tile.backdrop = sum;
            }
        }
    }
}

pub fn backdrop(_n_wg: u32, resources: &[CpuBinding]) {
    let config = resources[0].as_typed();
    let paths = resources[1].as_slice();
    let mut tiles = resources[2].as_slice_mut();
    backdrop_main(&config, &paths, &mut tiles);
}
