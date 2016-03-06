
pub struct Flat2dVec<T: Default + Clone> {
    data: Vec<T>,
    width: usize,
    height: usize
}

impl<T: Default + Clone> Flat2dVec<T> {
    pub fn new(width: usize, height: usize) -> Flat2dVec<T> {
        Flat2dVec {
            data: vec![T::default(); width * height],
            width: width,
            height: height
        }
    }

    fn 
}



