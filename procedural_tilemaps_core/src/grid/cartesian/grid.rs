#![allow(missing_docs)]

use std::{fmt, ops::Range};

use crate::grid::{
    coordinate_system::CoordinateSystem,
    direction::Direction,
    grid::{Grid, GridData, GridIndex, NodeRef},
};

use super::coordinates::{
    Cartesian2D, Cartesian3D, CartesianCoordinates, CartesianPosition, GridDelta,
};

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

/// Definition of a Cartesian grid (2D or 3D).
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Default))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub struct CartesianGrid<C: CoordinateSystem> {
    size_x: u32,
    size_y: u32,
    size_z: u32,
    looping_x: bool,
    looping_y: bool,
    looping_z: bool,
    pub(crate) coord_system: C,
    size_xy: u32,
}

impl<C: CartesianCoordinates> Grid<C> for CartesianGrid<C> {
    type Position = CartesianPosition;

    #[inline]
    fn coord_system(&self) -> &C {
        &self.coord_system
    }

    #[inline]
    fn directions_count(&self) -> usize {
        self.coord_system.directions().len()
    }

    #[inline]
    fn total_size(&self) -> usize {
        (self.size_xy * self.size_z) as usize
    }

    fn get_neighbours_in_all_directions(
        &self,
        grid_index: GridIndex,
        neighbours_buffer: &mut Vec<Option<GridIndex>>,
    ) {
        let pos = self.pos_from_index(grid_index);
        for dir in self.coord_system.directions() {
            neighbours_buffer[usize::from(*dir)] = self.get_next_index_in_direction(&pos, *dir);
        }
    }

    #[inline]
    fn index_from_pos(&self, grid_position: &CartesianPosition) -> GridIndex {
        self.index_from_coords(grid_position.x, grid_position.y, grid_position.z)
    }

    #[inline]
    fn pos_from_index(&self, grid_index: GridIndex) -> CartesianPosition {
        let index = grid_index as u32;
        CartesianPosition {
            x: index % self.size_x,
            y: (index / self.size_x) % self.size_y,
            z: index / self.size_xy,
        }
    }
}

impl<C: CartesianCoordinates> fmt::Display for CartesianGrid<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "( size: {} {} {}, looping: {} {} {} )",
            self.size_x, self.size_y, self.size_z, self.looping_x, self.looping_y, self.looping_z
        )
    }
}

impl CartesianGrid<Cartesian2D> {
    pub fn new_cartesian_2d(
        size_x: u32,
        size_y: u32,
        looping_x: bool,
        looping_y: bool,
    ) -> CartesianGrid<Cartesian2D> {
        Self::new(size_x, size_y, 1, looping_x, looping_y, false, Cartesian2D)
    }

    #[inline]
    pub fn get_index_2d(&self, x: u32, y: u32) -> GridIndex {
        (x + y * self.size_x) as GridIndex
    }

    #[inline]
    pub fn get_index_from_pos_2d(&self, pos: &CartesianPosition) -> GridIndex {
        (pos.x + pos.y * self.size_x) as GridIndex
    }
}

impl CartesianGrid<Cartesian3D> {
    pub fn new_cartesian_3d(
        size_x: u32,
        size_y: u32,
        size_z: u32,
        looping_x: bool,
        looping_y: bool,
        looping_z: bool,
    ) -> CartesianGrid<Cartesian3D> {
        Self::new(
            size_x,
            size_y,
            size_z,
            looping_x,
            looping_y,
            looping_z,
            Cartesian3D,
        )
    }
}

impl<C: CartesianCoordinates> CartesianGrid<C> {
    pub fn new(
        size_x: u32,
        size_y: u32,
        size_z: u32,
        looping_x: bool,
        looping_y: bool,
        looping_z: bool,
        coord_system: C,
    ) -> Self {
        Self {
            size_x,
            size_y,
            size_z,
            looping_x,
            looping_y,
            looping_z,
            size_xy: size_x * size_y,
            coord_system,
        }
    }

    #[inline]
    pub fn size(&self) -> (u32, u32, u32) {
        (self.size_x, self.size_y, self.size_z)
    }

    #[inline]
    pub fn size_x(&self) -> u32 {
        self.size_x
    }

    #[inline]
    pub fn size_y(&self) -> u32 {
        self.size_y
    }

    #[inline]
    pub fn size_z(&self) -> u32 {
        self.size_z
    }

    #[inline]
    pub fn indexes(&self) -> Range<GridIndex> {
        0..self.total_size()
    }

    #[inline]
    pub fn index_from_coords(&self, x: u32, y: u32, z: u32) -> GridIndex {
        (x + y * self.size_x + z * self.size_xy) as GridIndex
    }

    #[inline]
    pub fn pos_from_index(&self, index: GridIndex) -> CartesianPosition {
        let index = index as u32;
        CartesianPosition {
            x: index % self.size_x,
            y: (index / self.size_x) % self.size_y,
            z: index / self.size_xy,
        }
    }

    pub fn get_next_index_in_direction(
        &self,
        grid_position: &CartesianPosition,
        direction: Direction,
    ) -> Option<GridIndex> {
        let delta = &self.coord_system.deltas()[direction as usize];
        self.get_next_pos(grid_position, delta)
            .map(|pos| self.index_from_pos(&pos))
    }

    pub fn get_index_in_direction(
        &self,
        grid_position: &CartesianPosition,
        direction: Direction,
        units: i32,
    ) -> Option<GridIndex> {
        let delta = self.coord_system.deltas()[direction as usize] * units;
        self.get_next_pos(grid_position, &delta)
            .map(|pos| self.index_from_pos(&pos))
    }

    pub fn get_next_pos(
        &self,
        grid_position: &CartesianPosition,
        delta: &GridDelta,
    ) -> Option<CartesianPosition> {
        let mut next = grid_position.get_delta_position(delta);
        for (looping, coord, size) in [
            (self.looping_x, &mut next.0, self.size_x),
            (self.looping_y, &mut next.1, self.size_y),
            (self.looping_z, &mut next.2, self.size_z),
        ] {
            if looping {
                if *coord < 0 {
                    *coord += size as i64;
                }
                if *coord >= size as i64 {
                    *coord -= size as i64;
                }
            } else if *coord < 0 || *coord >= size as i64 {
                return None;
            }
        }
        Some(CartesianPosition {
            x: next.0 as u32,
            y: next.1 as u32,
            z: next.2 as u32,
        })
    }

    pub fn direction(&self, from: GridIndex, to: GridIndex) -> Direction {
        let from = self.pos_from_index(from);
        let to = self.pos_from_index(to);
        if from.x < to.x {
            Direction::XForward
        } else if from.x > to.x {
            Direction::XBackward
        } else if from.y < to.y {
            Direction::YForward
        } else if from.y > to.y {
            Direction::YBackward
        } else if from.z < to.z {
            Direction::ZForward
        } else {
            Direction::ZBackward
        }
    }

    pub fn default_grid_data<D: Default + Clone>(&self) -> GridData<C, D, CartesianGrid<C>> {
        GridData::new(self.clone(), vec![D::default(); self.total_size()])
    }

    pub fn new_grid_data<D: Clone>(&self, value: D) -> GridData<C, D, CartesianGrid<C>> {
        GridData::new(self.clone(), vec![value; self.total_size()])
    }
}

impl<C: CartesianCoordinates> NodeRef<C, CartesianGrid<C>> for CartesianPosition {
    fn to_index(&self, grid: &CartesianGrid<C>) -> GridIndex {
        grid.index_from_pos(self)
    }
}

impl<C: CartesianCoordinates> NodeRef<C, CartesianGrid<C>> for (u32, u32) {
    fn to_index(&self, grid: &CartesianGrid<C>) -> GridIndex {
        grid.index_from_coords(self.0, self.1, 0)
    }
}

impl<C: CartesianCoordinates> NodeRef<C, CartesianGrid<C>> for (u32, u32, u32) {
    fn to_index(&self, grid: &CartesianGrid<C>) -> GridIndex {
        grid.index_from_coords(self.0, self.1, self.2)
    }
}
