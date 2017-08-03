use num::{Integer, Unsigned};
use std::hash::Hash;
use std::iter::FromIterator;

use generate::{HashIndexer, IndexVertices, IntoVertices, Topological};

pub struct ConjointBuffer<N, V>
where
    N: Integer + Unsigned,
{
    indeces: Vec<N>,
    vertices: Vec<V>,
}

impl<N, V> ConjointBuffer<N, V>
where
    N: Integer + Unsigned,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_index_slice(&self) -> &[N] {
        self.indeces.as_slice()
    }

    pub fn as_vertex_slice(&self) -> &[V] {
        self.vertices.as_slice()
    }

    fn extend<I, J>(&mut self, indeces: I, vertices: J)
    where
        I: IntoIterator<Item = N>,
        J: IntoIterator<Item = V>,
    {
        self.indeces.extend(indeces);
        self.vertices.extend(vertices);
    }
}

impl<N, V> ConjointBuffer<N, V>
where
    N: Copy + From<usize> + Integer + Unsigned,
{
    pub fn append(&mut self, other: &mut Self) {
        let offset = N::from(self.vertices.len());
        self.vertices.append(&mut other.vertices);
        self.indeces
            .extend(other.indeces.drain(..).map(|index| index + offset))
    }
}

impl<N, V> Default for ConjointBuffer<N, V>
where
    N: Integer + Unsigned,
{
    fn default() -> Self {
        ConjointBuffer {
            indeces: vec![],
            vertices: vec![],
        }
    }
}

impl<T, V> FromIterator<T> for ConjointBuffer<usize, V>
where
    T: IntoVertices + Topological,
    T::Vertex: Eq + Hash + Into<V>,
{
    fn from_iter<I>(input: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let (indeces, vertices) = input.into_iter().index_vertices(HashIndexer::default());
        let mut buffer = ConjointBuffer::new();
        buffer.extend(indeces, vertices.into_iter().map(|vertex| vertex.into()));
        buffer
    }
}

#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;

    use buffer::conjoint::*;
    use generate::*;

    #[test]
    fn collect_topology_into_buffer() {
        type Point<T> = (OrderedFloat<T>, OrderedFloat<T>, OrderedFloat<T>);
        let buffer = sphere::UVSphere::<f32>::with_unit_radius(3, 2)
            .spatial_polygons() // 6 triangles, 18 vertices.
            .map_vertices(|(x, y, z)| {
                (OrderedFloat(x), OrderedFloat(y), OrderedFloat(z))
            })
            .triangulate()
            .collect::<ConjointBuffer<_, Point<f32>>>();

        assert_eq!(18, buffer.as_index_slice().len());
        assert_eq!(5, buffer.as_vertex_slice().len());
    }
}
