use std::marker::PhantomData;

use crate::{
    compute::{KernelBuilder, KernelLauncher},
    frontend::{CubeType, ExpandElement},
    ir::{Item, Vectorization},
    unexpanded, Runtime,
};

use super::{ArgSettings, CubePrimitive, LaunchArg, LaunchArgExpand, TensorHandle, UInt};

#[derive(new)]
pub struct Array<E> {
    _val: PhantomData<E>,
}

impl<C: CubeType> CubeType for Array<C> {
    type ExpandType = ExpandElement;
}

impl<C: CubeType> CubeType for &Array<C> {
    type ExpandType = ExpandElement;
}

impl<C: CubeType> CubeType for &mut Array<C> {
    type ExpandType = ExpandElement;
}

impl<E: CubeType> Array<E> {
    /// Obtain the array length of input
    pub fn len(&self) -> UInt {
        unexpanded!()
    }
}

impl<C: CubePrimitive> LaunchArg for Array<C> {
    type RuntimeArg<'a, R: Runtime> = ArrayHandle<'a, R>;
}

impl<C: CubePrimitive> LaunchArgExpand for &Array<C> {
    fn expand(builder: &mut KernelBuilder, vectorization: Vectorization) -> ExpandElement {
        builder.input_array(Item::vectorized(C::as_elem(), vectorization))
    }
}

impl<C: CubePrimitive> LaunchArgExpand for &mut Array<C> {
    fn expand(builder: &mut KernelBuilder, vectorization: Vectorization) -> ExpandElement {
        builder.output_array(Item::vectorized(C::as_elem(), vectorization))
    }
}

pub struct ArrayHandle<'a, R: Runtime> {
    pub handle: &'a burn_compute::server::Handle<R::Server>,
    pub length: [usize; 1],
}

impl<'a, R: Runtime> ArgSettings<R> for ArrayHandle<'a, R> {
    fn register(&self, launcher: &mut KernelLauncher<R>) {
        launcher.register_array(self)
    }
}

impl<'a, R: Runtime> ArrayHandle<'a, R> {
    pub fn new(handle: &'a burn_compute::server::Handle<R::Server>, length: usize) -> Self {
        Self {
            handle,
            length: [length],
        }
    }

    pub fn as_tensor(&self) -> TensorHandle<'_, R> {
        let shape = &self.length;

        TensorHandle {
            handle: self.handle,
            strides: &[1],
            shape,
        }
    }
}
