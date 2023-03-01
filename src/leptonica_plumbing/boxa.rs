extern crate leptonica_sys;
extern crate thiserror;

use leptonica_sys::{boxaCreate, boxaDestroy, l_int32};

/// Wrapper around Leptonica's [`Boxa`](https://tpgit.github.io/Leptonica/struct_boxa.html) structure
#[derive(Debug, PartialEq)]
pub struct Boxa(*mut leptonica_sys::Boxa);

impl Drop for Boxa {
    fn drop(&mut self) {
        unsafe {
            boxaDestroy(&mut self.0);
        }
    }
}

impl AsRef<leptonica_sys::Boxa> for Boxa {
    fn as_ref(&self) -> &leptonica_sys::Boxa {
        unsafe { &*self.0 }
    }
}

impl Boxa {
    /// Create a new Boxa from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Boxa struct.
    /// The Boxa struct must not be mutated whilst the wrapper exists.
    pub unsafe fn new_from_pointer(p: *mut leptonica_sys::Boxa) -> Self {
        Self(p)
    }

    /// Wrapper for [`boxaCreate`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#ae59916b7506831be9bf2119dea063253)
    ///
    /// Input: n (initial number of ptrs) Return: boxa, or null on error
    pub fn create(n: l_int32) -> Option<Boxa> {
        let ptr = unsafe { boxaCreate(n) };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    /// Safely borrow the nth item
    pub fn get(&self, i: isize) -> Option<crate::BorrowedBox> {
        let lboxa: &leptonica_sys::Boxa = self.as_ref();
        if lboxa.n <= std::convert::TryFrom::try_from(i).ok()? {
            None
        } else {
            unsafe { Some(crate::BorrowedBox::new(&*lboxa.box_.offset(i))) }
        }
    }
}

#[test]
fn create_valid_test() {
    let boxa = Boxa::create(4).unwrap();
    let lboxa: &leptonica_sys::Boxa = boxa.as_ref();
    assert_eq!(lboxa.nalloc, 4);
    assert_eq!(lboxa.n, 0);
}
