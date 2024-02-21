#![cfg(feature = "bytes")]
use crate::{allocative_trait::Allocative, visitor::Visitor, impls::common::PTR_NAME};
use bytes::Bytes;

impl Allocative for Bytes {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        // TODO: probably wrong because inner array could be large?
        let size = self.inner_data_size();
        let ptr = self.inner_data().as_ptr() as *const ();
        visitor.enter_shared(PTR_NAME, size, ptr);
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use crate::golden::golden_test;

    #[test]
    fn test_bytes_vec() {
        let bytes = Bytes::from(vec![1, 2, 3, 4, 5, 6]);
        let sliced = bytes.slice(1..4);

        golden_test!(&sliced);
    }

    #[test]
    fn test_bytes_slice() {
        let bytes = Bytes::from(vec![1, 2, 3, 4, 5, 6]);
        // TODO: get this working with slices, fix the cursed bytes code
        // (i think we might need to add a cap method to the vtable)
        //
        // let bytes = Bytes::from_static(&[1, 2, 3, 4, 5]);
        let sliced = bytes.slice(1..4);

        golden_test!(&sliced);
    }

    // TODO: other types of bytes, arc etc
}
