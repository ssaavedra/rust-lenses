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


#[cfg(test)]
mod test {
    use crate::mutable::*;
    #[derive(Copy, Clone, Debug)]
    struct Country<'a> {
        name: &'a str,
        flag: i64,
    }

    #[derive(Copy, Clone, Debug)]
    struct ExternalAddress<'a> {
        address: &'a str,
        zip: i64,
        country: Country<'a>,
    }

    #[test]
    pub fn test1() {
        let mut a = ExternalAddress {
            address: "test",
            zip: 0,
            country: Country { name: "USA", flag: 0 }
        };

        let mut _c = Country { name: "USA", flag: 0 };
        let lens_cn = mod_lens(&|x: &mut Country| &mut x.name);
        let lens_ac = mod_lens(&|x: &mut ExternalAddress| &mut x.country);
        let lens_acn = mod_lens(&|x: &mut ExternalAddress| &mut x.country.name);
        let lens_acn2 = mod_compose(&lens_ac, &lens_cn); // mod_lens(&closure);
        *lens_acn.view(&mut a) = "UNI2";
        // println!("Lens {:?}\n", a);
        *lens_acn2.view(&mut a) = "UNI3";
        // *lens_acn2.view(&mut a) = "UNI4"; // Cannot call a second time!
        println!("Lens {:?}\n", a);
        // */
    }
}

