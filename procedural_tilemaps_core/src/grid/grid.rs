#![allow(missing_docs)]

use std::{
    fmt::Debug,
    marker::PhantomData,
    slice::{Iter, IterMut},
};

use super::coordinate_system::CoordinateSystem;

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

/// Index of a node within a grid.
pub type GridIndex = usize;

/// Trait implemented by grid definitions.
pub trait Grid<C: CoordinateSystem>: Clone {
    /// Position type associated with this grid.
    type Position: Debug;

    /// Access to the underlying coordinate system.
    fn coord_system(&self) -> &C;
    /// Returns the number of possible neighbour directions.
    fn directions_count(&self) -> usize;
    /// Returns the total number of nodes in the grid.
    fn total_size(&self) -> usize;

    /// Fills `neighbours_buffer` with the index of the neighbour in each direction, or `None` when there is none.
    fn get_neighbours_in_all_directions(
        &self,
        grid_index: GridIndex,
        neighbours_buffer: &mut Vec<Option<GridIndex>>,
    );

    /// Returns the index corresponding to `pos`.
    fn index_from_pos(&self, pos: &Self::Position) -> GridIndex;
    /// Returns the grid position for an index.
    fn pos_from_index(&self, index: GridIndex) -> Self::Position;
}

/// Associates a grid with data for each node.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Default))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub struct GridData<C, D, G>
where
    C: CoordinateSystem,
    G: Grid<C>,
{
    grid: G,
    data: Vec<D>,
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    _phantom: PhantomData<C>,
}

impl<C, D, G> GridData<C, D, G>
where
    C: CoordinateSystem,
    G: Grid<C>,
{
    /// Creates a new grid data store from an existing grid and backing buffer.
    pub fn new(grid: G, data: Vec<D>) -> Self {
        Self {
            grid,
            data,
            _phantom: PhantomData,
        }
    }

    /// Access the grid definition.
    #[inline]
    pub fn grid(&self) -> &G {
        &self.grid
    }

    /// Writes a value without bounds checking.
    #[inline]
    pub fn set_raw(&mut self, index: GridIndex, value: D) {
        self.data[index] = value;
    }

    /// Writes a value using a [`NodeRef`].
    #[inline]
    pub fn set<N: NodeRef<C, G>>(&mut self, index_ref: N, value: D) {
        let idx = index_ref.to_index(&self.grid);
        self.data[idx] = value;
    }

    /// Returns a reference to the value at `index` (unchecked).
    #[inline]
    pub fn get(&self, index: GridIndex) -> &D {
        &self.data[index]
    }

    /// Returns a mutable reference to the value at `index` (unchecked).
    #[inline]
    pub fn get_mut(&mut self, index: GridIndex) -> &mut D {
        &mut self.data[index]
    }

    /// Iterates over all stored values.
    #[inline]
    pub fn iter(&self) -> Iter<'_, D> {
        self.data.iter()
    }

    /// Mutable iterator over all stored values.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, D> {
        self.data.iter_mut()
    }

    /// Returns a range of valid node indices.
    #[inline]
    pub fn indexes(&self) -> std::ops::Range<usize> {
        0..self.grid.total_size()
    }
}

impl<C: CoordinateSystem, D: Clone, G: Grid<C>> GridData<C, D, G> {
    /// Resets every element to `value`.
    pub fn reset(&mut self, value: D) {
        for d in &mut self.data {
            *d = value.clone();
        }
    }
}

/// Represents a reference to a node in a grid.
pub trait NodeRef<C: CoordinateSystem, G: Grid<C>> {
    /// Returns the backing index for this reference.
    fn to_index(&self, grid: &G) -> GridIndex;
}

impl<C: CoordinateSystem, G: Grid<C>> NodeRef<C, G> for GridIndex {
    fn to_index(&self, _grid: &G) -> GridIndex {
        *self
    }
}
