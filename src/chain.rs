/// A utility type to allow chaining parser outputs together.
pub trait Chain<T> {
    /// The number of items that this chain link consists of.
    fn len(&self) -> usize;
    /// Append the elements in this link to the chain.
    fn append(self, v: &mut Vec<T>);
}

impl<T> Chain<T> for T {
    fn len(&self) -> usize { 1 }
    fn append(self, v: &mut Vec<T>) { v.push(self) }
}

impl<T> Chain<T> for Option<T> {
    fn len(&self) -> usize { self.is_some() as usize }
    fn append(self, v: &mut Vec<T>) { self.map(|x| v.push(x)); }
}

impl<T> Chain<T> for Vec<T> {
    fn len(&self) -> usize { self.as_slice().len() }
    fn append(mut self, v: &mut Vec<T>) { v.append(&mut self) }
}
