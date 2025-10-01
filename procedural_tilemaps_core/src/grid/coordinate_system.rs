#![allow(missing_docs)]

use super::direction::DirectionTrait;

/// Represents a coordinate system used by the grid types.
pub trait CoordinateSystem: Default + Clone + Sync + Send + 'static {
    /// Direction type associated with this coordinate system.
    type Direction: DirectionTrait;

    /// Returns the available directions.
    fn directions(&self) -> &'static [Self::Direction];

    /// Returns the number of directions.
    fn directions_count(&self) -> usize;
}
