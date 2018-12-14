//! Cube primitives.
//!
//! # Examples
//!
//! ```rust
//! # extern crate nalgebra;
//! # extern crate plexus;
//! use nalgebra::Point3;
//! use plexus::graph::MeshGraph;
//! use plexus::prelude::*;
//! use plexus::primitive::cube::Cube;
//! use plexus::primitive::index::HashIndexer;
//!
//! # fn main() {
//! let graph = Cube::new()
//!     .polygons_with_position()
//!     .collect_with_indexer::<MeshGraph<Point3<f32>>, _>(HashIndexer::default())
//!     .unwrap();
//! # }
//! ```

use decorum::R64;
use num::{One, Zero};

use crate::geometry::{Duplet, Triplet};
use crate::primitive::generate::{
    Generate, PolygonGenerator, PositionGenerator, PositionIndexGenerator,
    PositionPolygonGenerator, PositionVertexGenerator, UvMapGenerator, UvMapPolygonGenerator,
    VertexGenerator,
};
use crate::primitive::topology::{Converged, Map, Quad};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Plane {
    XY,
    NXY,
    ZY,
    NZY,
    XZ,
    XNZ,
}

#[derive(Clone, Copy)]
pub struct Bounds {
    lower: R64,
    upper: R64,
}

impl Bounds {
    pub fn with_radius(radius: R64) -> Self {
        Bounds {
            lower: -radius,
            upper: radius,
        }
    }

    pub fn with_width(width: R64) -> Self {
        Self::with_radius(width / 2.0)
    }

    pub fn unit_radius() -> Self {
        Self::with_radius(One::one())
    }

    pub fn unit_width() -> Self {
        Self::with_width(One::one())
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Self::unit_width()
    }
}

#[derive(Clone, Copy)]
pub struct Cube;

impl Cube {
    pub fn new() -> Self {
        Cube
    }

    pub fn polygons_with_plane(&self) -> Generate<Self, (), Quad<Plane>> {
        Generate::new(
            self,
            (),
            self.polygon_count(),
            Cube::polygon_with_plane_from,
        )
    }

    fn polygon_with_plane_from(&self, _: &(), index: usize) -> Quad<Plane> {
        match index {
            0 => Quad::converged(Plane::XY),  // front
            1 => Quad::converged(Plane::NZY), // right
            2 => Quad::converged(Plane::XNZ), // top
            3 => Quad::converged(Plane::ZY),  // left
            4 => Quad::converged(Plane::XZ),  // bottom
            5 => Quad::converged(Plane::NXY), // back
            _ => panic!(),
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::new()
    }
}

impl VertexGenerator for Cube {}

impl PolygonGenerator for Cube {
    fn polygon_count(&self) -> usize {
        6
    }
}

impl PositionGenerator for Cube {
    type State = Bounds;
}

impl PositionVertexGenerator for Cube {
    type Output = Triplet<R64>;

    fn vertex_with_position_from(&self, state: &Self::State, index: usize) -> Self::Output {
        let x = if index & 0b100 == 0b100 {
            state.upper
        }
        else {
            state.lower
        };
        let y = if index & 0b010 == 0b010 {
            state.upper
        }
        else {
            state.lower
        };
        let z = if index & 0b001 == 0b001 {
            state.upper
        }
        else {
            state.lower
        };
        Triplet(x, y, z)
    }

    fn vertex_with_position_count(&self) -> usize {
        8
    }
}

impl PositionPolygonGenerator for Cube {
    type Output = Quad<Triplet<R64>>;

    fn polygon_with_position_from(&self, state: &Self::State, index: usize) -> Self::Output {
        self.index_for_position(index)
            .map(|index| self.vertex_with_position_from(state, index))
    }
}

impl PositionIndexGenerator for Cube {
    type Output = Quad<usize>;

    fn index_for_position(&self, index: usize) -> <Self as PositionIndexGenerator>::Output {
        match index {
            0 => Quad::new(5, 7, 3, 1), // front
            1 => Quad::new(6, 7, 5, 4), // right
            2 => Quad::new(3, 7, 6, 2), // top
            3 => Quad::new(0, 1, 3, 2), // left
            4 => Quad::new(4, 5, 1, 0), // bottom
            5 => Quad::new(0, 2, 6, 4), // back
            _ => panic!(),
        }
    }
}

impl UvMapGenerator for Cube {
    type State = ();
}

impl UvMapPolygonGenerator for Cube {
    type Output = Quad<Duplet<R64>>;

    fn polygon_with_uv_map_from(&self, _: &Self::State, index: usize) -> Self::Output {
        let uu = Duplet::one();
        let ul = Duplet(One::one(), Zero::zero());
        let ll = Duplet::zero();
        let lu = Duplet(Zero::zero(), One::one());
        match index {
            0 | 4 | 5 => Quad::new(uu, ul, ll, lu), // front | bottom | back
            1 => Quad::new(ul, ll, lu, uu),         // right
            2 | 3 => Quad::new(lu, uu, ul, ll),     // top | left
            _ => panic!(),
        }
    }
}
