//   Copyright 2018 Santiago Saavedra López. All Rights Reserved.
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

pub trait Getter<S, A> {
    fn view(&self, s: &S) -> A;
}

pub trait Lens<S, A>: Getter<S, A> {
    fn set(&self, s: &S, a: &A) -> S;
}


pub fn lens<'a, S, A>(getter: &'a Fn(&S) -> A, setter: &'a Fn(&S, &A) -> S) -> LensImpl<'a, S, A> {
    LensImpl {
        view: getter,
        set: setter,
    }
}

pub fn compose<'a, S, A, B>(lhs: &'a Lens<S, A>, rhs: &'a Lens<A, B>) -> CompoundLensImpl<'a, S, A, B> {
    CompoundLensImpl {
        lhs: lhs,
        rhs: rhs,
    }
}

#[macro_export]
macro_rules! struct_lens {
    (clone $thetype:ident, $expr:ident) => {
        lenses::lens::lens(&|obj: &$thetype| obj.$expr.clone(), &|obj, newval| {
            $thetype {
                $expr: newval.clone(),
                .. obj.clone()
            }
        })
    };

    (copy $thetype:ident, $expr:ident) => {
        lenses::lens::lens(&|obj: &$thetype| obj.$expr, &|obj, newval| {
            $thetype {
                $expr: *newval,
                .. *obj
            }
        })
    };
}

#[macro_export]
macro_rules! gen_lens {
    (clone $thetype:ident, $($expr:ident).*) => {
        lenses::lens::lens(&|obj: &$thetype| obj.$($expr).*.clone(), &|obj, newval| {
            let mut new = obj.clone();
            new.$($expr).* = newval.clone();
            new
        })
    };

    (copy $thetype:ident, $($expr:ident).*) => {
        lenses::lens::lens(&|obj: &$thetype| obj.$($expr).*, &|obj, newval| {
            let mut new = *obj;
            new.$($expr).* = *newval;
            new
        })
    };
}

pub struct LensImpl<'a, S, A> {
    view: &'a Fn(&S) -> A,
    set: &'a Fn (&S, &A) -> S,
}

pub struct CompoundLensImpl<'a, S, A, B> {
    lhs: &'a Lens<S, A>,
    rhs: &'a Lens<A, B>,
}

impl <'a, S, A> Getter<S, A> for LensImpl<'a, S, A> {
    fn view(&self, s: &S) -> A { (self.view)(s) }
}

impl <'a, S, A> Lens<S, A> for LensImpl<'a, S, A> {
    fn set(&self, s: &S, a: &A) -> S { (self.set)(s, a) }
}

impl <'a, S, A, B> Getter<S, B> for CompoundLensImpl<'a, S, A, B> {
    fn view(&self, s: &S) -> B { self.rhs.view(&self.lhs.view(s)) }
}

impl <'a, S, A, B> Lens<S, B> for CompoundLensImpl<'a, S, A, B> {
    fn set(&self, s: &S, b: &B) -> S { self.lhs.set(s, &self.rhs.set(&self.lhs.view(s), b)) }
}

