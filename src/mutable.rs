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
