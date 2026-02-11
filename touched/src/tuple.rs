use crate::Touchable;
use impl_trait_for_tuples::impl_for_tuples;

#[impl_for_tuples(16)]
impl Touchable for Tuple {
    fn touch(&self) {
        for_tuples!( #( crate::touching::<Tuple>(&self.Tuple); )* );
    }
}
