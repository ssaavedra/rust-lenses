/////// A Lens-like composable abstraction for mutable fields
pub trait ModLens<'a, S, A> {
    fn view(&self, s: &'a mut S) -> &'a mut A;
}

pub struct ModLensImpl<'a, 's, S, A>(&'s Fn(&'a mut S) -> &'a mut A);
pub struct ModLensCompoundImpl<'a, 's, S, A, B> {
    lhs: &'s ModLens<'a, S, A>,
    rhs: &'s ModLens<'a, A, B>,
}

impl <'a, 's, S, A> ModLens<'a, S, A> for ModLensImpl<'a, 's, S, A> {
    fn view(&self, s: &'a mut S) -> &'a mut A {
        (self.0)(s)
    }
}

impl <'a, 's, S, A:'a, B> ModLens<'a, S, B> for ModLensCompoundImpl<'a, 's, S, A, B> {
    fn view(&self, s: &'a mut S) -> &'a mut B {
        self.rhs.view(self.lhs.view(s))
    }
}

pub fn mod_compose<'a, 's, S, A, B>(lhs: &'s ModLens<'a, S, A>, rhs: &'s ModLens<'a, A, B>) -> ModLensCompoundImpl<'a, 's, S, A, B> {
    ModLensCompoundImpl {
        lhs: lhs,
        rhs: rhs,
    }
}

pub fn mod_lens<'a, 's, S, A>(getter: &'s Fn(&'a mut S) -> &'a mut A) -> ModLensImpl<'a, 's, S, A> {
    ModLensImpl(getter)
}
