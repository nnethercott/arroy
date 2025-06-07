/// A wrapper type implementing `Ord`
///
/// Since distance metrics satisfy d(x,x)=0 and d(x,y)>0 for x!=y we don't need to operate on the
/// full range of an f32. Comparing the u32 representation of a non-negative f32 should suffice and
/// is actually a lot quicker.
///
/// We just need to ensure -0.0's and NaN's don't show up anywhere cause f32<Nan<-0
#[derive(Clone)]
pub struct NonNegativeOrderedFloat(pub f32);

impl PartialEq for NonNegativeOrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits().eq(&other.0.to_bits())
    }
}

impl Eq for NonNegativeOrderedFloat {}

impl PartialOrd for NonNegativeOrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.to_bits().partial_cmp(&other.0.to_bits())
    }
}

impl Ord for NonNegativeOrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.to_bits().cmp(&other.0.to_bits())
    }
}

// TODO: add proptests and unit tests here
