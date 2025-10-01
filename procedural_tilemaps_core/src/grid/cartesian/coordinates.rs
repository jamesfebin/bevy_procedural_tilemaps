#![allow(missing_docs)]

use std::fmt;

use crate::grid::{coordinate_system::CoordinateSystem, direction::Direction};

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

/// Right-handed 2D Cartesian coordinate system with four directions.
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub struct Cartesian2D;
impl CoordinateSystem for Cartesian2D {
    type Direction = Direction;

    #[inline]
    fn directions(&self) -> &'static [Direction] {
        CARTESIAN_2D_DIRECTIONS
    }

    #[inline]
    fn directions_count(&self) -> usize {
        CARTESIAN_2D_DIRECTIONS.len()
    }
}
impl CartesianCoordinates for Cartesian2D {
    #[inline]
    fn deltas(&self) -> &'static [GridDelta] {
        CARTESIAN_2D_DELTAS
    }
}

/// Right-handed 3D Cartesian coordinate system with six directions.
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub struct Cartesian3D;
impl CoordinateSystem for Cartesian3D {
    type Direction = Direction;

    #[inline]
    fn directions(&self) -> &'static [Direction] {
        CARTESIAN_3D_DIRECTIONS
    }

    #[inline]
    fn directions_count(&self) -> usize {
        CARTESIAN_3D_DIRECTIONS.len()
    }
}
impl CartesianCoordinates for Cartesian3D {
    #[inline]
    fn deltas(&self) -> &'static [GridDelta] {
        CARTESIAN_3D_DELTAS
    }
}

/// All directions for a 2D Cartesian system.
pub const CARTESIAN_2D_DIRECTIONS: &[Direction] = &[
    Direction::XForward,
    Direction::YForward,
    Direction::XBackward,
    Direction::YBackward,
];

/// Grid deltas for each 2D direction.
pub const CARTESIAN_2D_DELTAS: &[GridDelta] = &[
    GridDelta {
        dx: 1,
        dy: 0,
        dz: 0,
    },
    GridDelta {
        dx: 0,
        dy: 1,
        dz: 0,
    },
    GridDelta {
        dx: -1,
        dy: 0,
        dz: 0,
    },
    GridDelta {
        dx: 0,
        dy: -1,
        dz: 0,
    },
];

/// All directions for a 3D Cartesian system.
pub const CARTESIAN_3D_DIRECTIONS: &[Direction] = &[
    Direction::XForward,
    Direction::YForward,
    Direction::XBackward,
    Direction::YBackward,
    Direction::ZForward,
    Direction::ZBackward,
];

/// Grid deltas for each 3D direction.
pub const CARTESIAN_3D_DELTAS: &[GridDelta] = &[
    GridDelta {
        dx: 1,
        dy: 0,
        dz: 0,
    },
    GridDelta {
        dx: 0,
        dy: 1,
        dz: 0,
    },
    GridDelta {
        dx: -1,
        dy: 0,
        dz: 0,
    },
    GridDelta {
        dx: 0,
        dy: -1,
        dz: 0,
    },
    GridDelta {
        dx: 0,
        dy: 0,
        dz: 1,
    },
    GridDelta {
        dx: 0,
        dy: 0,
        dz: -1,
    },
];

/// Represents a displacement on the grid.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub struct GridDelta {
    pub dx: i32,
    pub dy: i32,
    pub dz: i32,
}

impl GridDelta {
    pub fn new(dx: i32, dy: i32, dz: i32) -> Self {
        Self { dx, dy, dz }
    }
}

impl std::ops::Mul<i32> for GridDelta {
    type Output = GridDelta;

    fn mul(self, rhs: i32) -> GridDelta {
        GridDelta {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
            dz: self.dz * rhs,
        }
    }
}

/// Helper trait for coordinate systems that expose cartesian deltas.
pub trait CartesianCoordinates: CoordinateSystem<Direction = Direction> {
    /// Returns the displacement for each direction.
    fn deltas(&self) -> &'static [GridDelta];
}

/// Represents a 3D position in the grid (used for both 2D and 3D grids).
#[derive(Default, Hash, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub struct CartesianPosition {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl CartesianPosition {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    pub fn new_xy(x: u32, y: u32) -> Self {
        Self { x, y, z: 0 }
    }

    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }

    pub(crate) fn get_delta_position(&self, delta: &GridDelta) -> (i64, i64, i64) {
        (
            i64::from(self.x) + i64::from(delta.dx),
            i64::from(self.y) + i64::from(delta.dy),
            i64::from(self.z) + i64::from(delta.dz),
        )
    }
}

impl From<(u32, u32)> for CartesianPosition {
    fn from(xy: (u32, u32)) -> Self {
        CartesianPosition::new(xy.0, xy.1, 0)
    }
}

impl From<(u32, u32, u32)> for CartesianPosition {
    fn from(xyz: (u32, u32, u32)) -> Self {
        CartesianPosition::new(xyz.0, xyz.1, xyz.2)
    }
}

impl fmt::Display for CartesianPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
