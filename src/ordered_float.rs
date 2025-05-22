/// A wrapper type implementing `Ord`
/// f32s are not ordered due to NaN bit patterns but we need some way of ranking leaf nodes in
/// arroy based off distance to a query
///
/// Since distance metrics satisfy d(x,x)=0 and d(x,y)>0 for x!=y we don't need to operate on the
/// full range of an f32. Comparing the u32 representation of a non-negative f32 should suffice and
/// is actually a lot quicker.
#[derive(Clone)]
pub struct NonNegativeOrderedFloat(pub f32);

impl PartialEq for NonNegativeOrderedFloat{
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits().eq(&other.0.to_bits())
    }
}

impl Eq for NonNegativeOrderedFloat{}

impl PartialOrd for NonNegativeOrderedFloat{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.to_bits().partial_cmp(&other.0.to_bits())
    }
}

impl Ord for NonNegativeOrderedFloat{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.to_bits().cmp(&other.0.to_bits())
    }
}


// TODO: add proptests and unit tests here
