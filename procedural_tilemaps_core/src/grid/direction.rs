#![allow(missing_docs)]

// Minimal direction definitions derived from ghx_grid.

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

/// Index of a direction within a coordinate system.
pub type DirectionIndex = usize;

/// Trait implemented by direction enums.
pub trait DirectionTrait: Into<DirectionIndex> + Copy {
    /// Returns the opposite direction.
    fn opposite(&self) -> Self;
    /// Returns the rotation basis for this direction (right-handed).
    fn rotation_basis(&self) -> &'static [Self];
}

/// Enumerates the six primary axes used by Cartesian grids.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub enum Direction {
    /// X+ axis
    #[default]
    XForward = 0,
    /// Y+ axis
    YForward = 1,
    /// X- axis
    XBackward = 2,
    /// Y- axis
    YBackward = 3,
    /// Z+ axis
    ZForward = 4,
    /// Z- axis
    ZBackward = 5,
}

impl DirectionTrait for Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::XForward => Direction::XBackward,
            Direction::XBackward => Direction::XForward,
            Direction::YForward => Direction::YBackward,
            Direction::YBackward => Direction::YForward,
            Direction::ZForward => Direction::ZBackward,
            Direction::ZBackward => Direction::ZForward,
        }
    }

    fn rotation_basis(&self) -> &'static [Direction] {
        match self {
            Direction::XForward => X_POS_AXIS,
            Direction::XBackward => X_NEG_AXIS,
            Direction::YForward => Y_POS_AXIS,
            Direction::YBackward => Y_NEG_AXIS,
            Direction::ZForward => Z_POS_AXIS,
            Direction::ZBackward => Z_NEG_AXIS,
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        value as usize
    }
}

const X_POS_AXIS: &[Direction] = &[
    Direction::YForward,
    Direction::ZForward,
    Direction::YBackward,
    Direction::ZBackward,
];
const X_NEG_AXIS: &[Direction] = &[
    Direction::ZForward,
    Direction::YForward,
    Direction::ZBackward,
    Direction::YBackward,
];
const Y_POS_AXIS: &[Direction] = &[
    Direction::ZForward,
    Direction::XForward,
    Direction::ZBackward,
    Direction::XBackward,
];
const Y_NEG_AXIS: &[Direction] = &[
    Direction::XForward,
    Direction::ZForward,
    Direction::XBackward,
    Direction::ZBackward,
];
const Z_POS_AXIS: &[Direction] = &[
    Direction::XForward,
    Direction::YForward,
    Direction::XBackward,
    Direction::YBackward,
];
const Z_NEG_AXIS: &[Direction] = &[
    Direction::YForward,
    Direction::XForward,
    Direction::YBackward,
    Direction::XBackward,
];
