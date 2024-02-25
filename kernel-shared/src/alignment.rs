// Generic struct wrappers that specify alignment
// so we then can through a packed structure in them.
// Needed as you cannot just make the struct both aligned
// and packed in one shot.

#[repr(C, align(16))]
pub struct Aligned16<T> {
    pub Field: T,
}
