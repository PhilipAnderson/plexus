use plexus::integration::nalgebra;

use decorum::R64;
use nalgebra::Point3;
use pictor::pipeline::Vertex;
use pictor::{self, Color4};
use plexus::prelude::*;
use plexus::primitive;
use plexus::primitive::generate::{Normal, Position};
use plexus::primitive::sphere::UvSphere;

type E3 = Point3<R64>;

fn main() {
    let from = Point3::new(0.0, -4.0, 1.0);
    let to = Point3::origin();
    pictor::draw_with(from, to, move || {
        let sphere = UvSphere::new(32, 16);
        primitive::zip_vertices((
            sphere.polygons::<Position<E3>>().triangulate(),
            sphere.polygons::<Normal<E3>>().triangulate(),
        ))
        .map_vertices(|(position, normal)| {
            Vertex::new(position, normal.into_inner(), Color4::white().into())
        })
        .collect()
    });
}