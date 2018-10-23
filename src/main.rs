mod monad;

trait Getter<S, A> {
    fn view(&self, s: &S) -> A;
}

trait Lens<S, A>: Getter<S, A> {
    fn set(&self, s: &S, a: &A) -> S;
}

#[derive(Clone, Copy, Debug)]
struct Country<'a> {
    name: &'a str,
    flag: i64,
}


#[derive(Clone, Copy, Debug)]
struct ExternalAddress<'a> {
    address: &'a str,
    zip: i64,
    country: Country<'a>,
}

struct LensImpl<S, A> {
    view: Box<Fn(&S) -> A>,
    set: Box<Fn (&S, &A) -> S>,
}


struct CompoundLensImpl<'a, S, A, B> {
    lhs: &'a Lens<S, A>,
    rhs: &'a Lens<A, B>,
}

impl <S, A> Getter<S, A> for LensImpl<S, A> {
    fn view(&self, s: &S) -> A { (self.view)(s) }
}

impl <S, A> Lens<S, A> for LensImpl<S, A> {
    fn set(&self, s: &S, a: &A) -> S { (self.set)(s, a) }
}


impl <'a, S, A, B> Getter<S, B> for CompoundLensImpl<'a, S, A, B> {
    fn view(&self, s: &S) -> B { self.rhs.view(&self.lhs.view(s)) }
}

impl <'a, S, A, B> Lens<S, B> for CompoundLensImpl<'a, S, A, B> {
    fn set(&self, s: &S, b: &B) -> S { self.lhs.set(s, &self.rhs.set(&self.lhs.view(s), b)) }
}

fn compose<'a, S, A, B>(lhs: &'a Lens<S, A>, rhs: &'a Lens<A, B>) -> CompoundLensImpl<'a, S, A, B> {
    CompoundLensImpl {
        lhs: lhs,
        rhs: rhs,
    }
}

fn lens<S, A>(getter: &'static Fn(&S) -> A, setter: &'static Fn(&S, &A) -> S) -> LensImpl<S, A> {
    LensImpl {
        view: Box::new(getter),
        set: Box::new(setter),
    }
}

macro_rules! struct_lens {
    ($thetype:ident, $expr:ident) => {
        lens(&|obj: &$thetype| obj.$expr, &|obj, newval| {
            $thetype {
                $expr: *newval,
                .. *obj
            }
        })
    };
}

macro_rules! gen_lens {
    ($thetype:ident, $($expr:ident).*) => {
        lens(&|obj: &$thetype| obj.$($expr).*, &|obj, newval| {
            let mut new = *obj;
            new.$($expr).* = *newval;
            new
        })
    };
}

fn main() {

    let coa = struct_lens!(ExternalAddress, country);
    let noc = struct_lens!(Country, name);
    let noc_coa = compose(&coa, &noc);
    let noc_coa2 = gen_lens!(ExternalAddress, country.name);

    let addr: ExternalAddress = ExternalAddress { address: "123, Fake St.", zip: 12345,
                                                  country: Country { name: "USA", flag: 0 }};

    // let addr2: ExternalAddress = add_zipcode.set(addr, 20000);
    let addr2 = noc_coa.set(&addr, &"United States");
    let addr3 = noc_coa.set(&addr, &"United States of America");


    println!("Hello, world! {:?} {:?} {:?}", addr, addr2, addr3);
}
