pub struct CBox {}

impl CBox {
    pub fn into_raw<T> (object: T) -> *mut T {
        Box::into_raw(Box::new(object))
    }

    pub fn from_raw<T>(pointer: *mut T) -> Box<T> {
        assert_eq!(pointer.is_null(), false, "CBox::from_raw(): Pointer must not be null!");
        unsafe { Box::from_raw(pointer) }
    }

    pub fn with_raw<F, R, T>(pointer: *mut T, block: F) -> R where F : FnOnce(&mut Box<T>) -> R {
        assert_eq!(pointer.is_null(), false, "CBox::with_raw(): Pointer must not be null!");

        let mut boxed_object: Box<T> = Self::from_raw(pointer);
        let result: R = block(&mut boxed_object);
        Self::into_raw(boxed_object);
        result
    }
}