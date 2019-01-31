use crate::geometry::Geometry;
use crate::graph::storage::convert::{AsStorage, AsStorageMut};
use crate::graph::storage::Storage;
use crate::graph::topology::{Arc, Edge, Face, Topological, Vertex};

/// Marker trait for containers that promise to be in a consistent state.
///
/// This trait is only implemented by containers that expose storage and ensure
/// that that storage is consistent via the mutation API.  Note that `Core`
/// does not implement this trait, and instead acts as a raw container for
/// topological storage that can be freely manipulated.
///
/// This trait allows code to make assumptions about the data it operates
/// against. For example, views expose an API to user code that assumes that
/// topologies are present and implicitly unwraps values.
pub trait Consistent {}

impl<'a, T> Consistent for &'a T where T: Consistent {}

impl<'a, T> Consistent for &'a mut T where T: Consistent {}

pub trait Reborrow {
    type Target;

    fn reborrow(&self) -> &Self::Target;
}

pub trait ReborrowMut: Reborrow {
    fn reborrow_mut(&mut self) -> &mut Self::Target;
}

impl<'a, T> Reborrow for &'a T {
    type Target = T;

    fn reborrow(&self) -> &Self::Target {
        *self
    }
}

impl<'a, T> Reborrow for &'a mut T {
    type Target = T;

    fn reborrow(&self) -> &Self::Target {
        &**self
    }
}

impl<'a, T> ReborrowMut for &'a mut T {
    fn reborrow_mut(&mut self) -> &mut Self::Target {
        *self
    }
}

pub trait Bind<T, M>
where
    T: Topological,
    M: AsStorage<T>,
{
    type Output;

    fn bind(self, source: M) -> Self::Output;
}

/// Topological storage container.
///
/// A core may or may not own its storage and may or may not provide storage
/// for all topologies (vertices, arcs, edges, and faces). When a core does not
/// own its storage, it is _ephemeral_. A core that owns storage for all
/// topologies is known as an _owned core_. See the `OwnedCore` type alias.
///
/// Unlike `MeshGraph`, `Core` does not implement the `Consistent` trait.
/// `MeshGraph` contains an owned core, but does not mutate it outside of the
/// mutation API, which maintains consistency.
pub struct Core<V = (), A = (), E = (), F = ()> {
    vertices: V,
    arcs: A,
    edges: E,
    faces: F,
}

impl Core {
    pub fn empty() -> Self {
        Core {
            vertices: (),
            arcs: (),
            edges: (),
            faces: (),
        }
    }
}

impl<V, A, E, F> Core<V, A, E, F> {
    pub fn into_storage(self) -> (V, A, E, F) {
        let Core {
            vertices,
            arcs,
            edges,
            faces,
            ..
        } = self;
        (vertices, arcs, edges, faces)
    }
}

impl<V, A, E, F, G> AsStorage<Vertex<G>> for Core<V, A, E, F>
where
    V: AsStorage<Vertex<G>>,
    G: Geometry,
{
    fn as_storage(&self) -> &Storage<Vertex<G>> {
        self.vertices.as_storage()
    }
}

impl<V, A, E, F, G> AsStorage<Arc<G>> for Core<V, A, E, F>
where
    A: AsStorage<Arc<G>>,
    G: Geometry,
{
    fn as_storage(&self) -> &Storage<Arc<G>> {
        self.arcs.as_storage()
    }
}

impl<V, A, E, F, G> AsStorage<Edge<G>> for Core<V, A, E, F>
where
    E: AsStorage<Edge<G>>,
    G: Geometry,
{
    fn as_storage(&self) -> &Storage<Edge<G>> {
        self.edges.as_storage()
    }
}

impl<V, A, E, F, G> AsStorage<Face<G>> for Core<V, A, E, F>
where
    F: AsStorage<Face<G>>,
    G: Geometry,
{
    fn as_storage(&self) -> &Storage<Face<G>> {
        self.faces.as_storage()
    }
}

impl<V, A, E, F, G> AsStorageMut<Vertex<G>> for Core<V, A, E, F>
where
    V: AsStorageMut<Vertex<G>>,
    G: Geometry,
{
    fn as_storage_mut(&mut self) -> &mut Storage<Vertex<G>> {
        self.vertices.as_storage_mut()
    }
}

impl<V, A, E, F, G> AsStorageMut<Arc<G>> for Core<V, A, E, F>
where
    A: AsStorageMut<Arc<G>>,
    G: Geometry,
{
    fn as_storage_mut(&mut self) -> &mut Storage<Arc<G>> {
        self.arcs.as_storage_mut()
    }
}

impl<V, A, E, F, G> AsStorageMut<Edge<G>> for Core<V, A, E, F>
where
    E: AsStorageMut<Edge<G>>,
    G: Geometry,
{
    fn as_storage_mut(&mut self) -> &mut Storage<Edge<G>> {
        self.edges.as_storage_mut()
    }
}

impl<V, A, E, F, G> AsStorageMut<Face<G>> for Core<V, A, E, F>
where
    F: AsStorageMut<Face<G>>,
    G: Geometry,
{
    fn as_storage_mut(&mut self) -> &mut Storage<Face<G>> {
        self.faces.as_storage_mut()
    }
}

impl<V, A, E, F, G> Bind<Vertex<G>, V> for Core<(), A, E, F>
where
    V: AsStorage<Vertex<G>>,
    G: Geometry,
{
    type Output = Core<V, A, E, F>;

    fn bind(self, vertices: V) -> Self::Output {
        let Core {
            arcs, edges, faces, ..
        } = self;
        Core {
            vertices,
            arcs,
            edges,
            faces,
        }
    }
}

impl<V, A, E, F, G> Bind<Arc<G>, A> for Core<V, (), E, F>
where
    A: AsStorage<Arc<G>>,
    G: Geometry,
{
    type Output = Core<V, A, E, F>;

    fn bind(self, arcs: A) -> Self::Output {
        let Core {
            vertices,
            edges,
            faces,
            ..
        } = self;
        Core {
            vertices,
            arcs,
            edges,
            faces,
        }
    }
}

impl<V, A, E, F, G> Bind<Edge<G>, E> for Core<V, A, (), F>
where
    E: AsStorage<Edge<G>>,
    G: Geometry,
{
    type Output = Core<V, A, E, F>;

    fn bind(self, edges: E) -> Self::Output {
        let Core {
            vertices,
            arcs,
            faces,
            ..
        } = self;
        Core {
            vertices,
            arcs,
            edges,
            faces,
        }
    }
}

impl<V, A, E, F, G> Bind<Face<G>, F> for Core<V, A, E, ()>
where
    F: AsStorage<Face<G>>,
    G: Geometry,
{
    type Output = Core<V, A, E, F>;

    fn bind(self, faces: F) -> Self::Output {
        let Core {
            vertices,
            arcs,
            edges,
            ..
        } = self;
        Core {
            vertices,
            arcs,
            edges,
            faces,
        }
    }
}

pub mod alias {
    use super::*;

    use crate::graph::storage::Storage;
    use crate::graph::topology::{Arc, Face, Vertex};

    // TODO: Include composite edges.
    pub type OwnedCore<G> = Core<Storage<Vertex<G>>, Storage<Arc<G>>, (), Storage<Face<G>>>;
}
