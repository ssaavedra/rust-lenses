use std::rc::Rc;

pub trait HKT<U> {
    type C; // Current type
    type T; // Type with C swapped with U
}

macro_rules! derive_hkt {
    ($t:ident) => {
        impl<T, U> HKT<U> for $t<T> {
            type C = T;
            type T = $t<U>;
        }
    }
}

derive_hkt!(Vec);
derive_hkt!(Option);
derive_hkt!(Box);
derive_hkt!(Rc);

pub trait Functor<U>: HKT<U> {
    fn map<F>(&self, f: F) -> Self::T where F: Fn(&Self::C) -> U;
}

impl<T, U> Functor<U> for Vec<T> {
    fn map<F>(&self, f: F) -> Vec<U> where F: Fn(&T) -> U {
        let mut result = Vec::with_capacity(self.len());
        for value in self {
            result.push( f(value) );
        }
        result
    }
}

impl<T, U> Functor<U> for Option<T> {
    fn map<F>(&self, f: F) -> Option<U> where F: Fn(&T) -> U {
        match *self {
            Some(ref value) => Some( f(value) ),
            None => None,
        }
    }
}

impl<T, U> Functor<U> for Rc<T> {
    fn map<F>(&self, f: F) -> Rc<U> where F: Fn(&T) -> U {
        let v = f(self);
        Rc::new(v)
    }
}

impl<T, U> Functor<U> for Box<T> {
    fn map<F>(&self, f: F) -> Box<U> where F: Fn(&T) -> U {
        let v = f(self);
        Box::new(v)
    }
}

pub trait Applicative<U>: Functor<U> {
    fn pure_(value: U) -> Self::T where Self: HKT<U, C=U>;
    fn seq<F>(&self, _: <Self as HKT<F>>::T) -> <Self as HKT<U>>::T where Self: HKT<F>, F: Fn(&<Self as HKT<F>>::C) -> U;
}

impl<T, U> Applicative<U> for Option<T> {
    fn pure_(value: U) -> <Self as HKT<U>>::T { Some(value) }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<U>>::T where F: Fn(&<Self as HKT<F>>::C) -> U {
        match *self {
            Some(ref value) => match fs {
                Some(f) => Some( f(value) ),
                None => None,
            },
            None => None,
        }
    }
}

impl<T, U> Applicative<U> for Vec<T> {
    fn pure_(value: U) -> <Self as HKT<U>>::T { vec![value] }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<U>>::T where F: Fn(&<Self as HKT<F>>::C) -> U {
        let mut result = vec![];
        for (i, f) in fs.into_iter().enumerate() {
            let v = (f)( &self[i] );
            result.push(v)
        }
        return result;
    }
}

impl<T, U> Applicative<U> for Rc<T> {
    fn pure_(value: U) -> <Self as HKT<U>>::T { Rc::new(value) }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<U>>::T where F: Fn(&<Self as HKT<F>>::C) -> U {
        let v = fs(self);
        Rc::new(v)
    }
}

impl<T, U> Applicative<U> for Box<T> {
    fn pure_(value: U) -> <Self as HKT<U>>::T { Box::new(value) }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<U>>::T where F: Fn(&<Self as HKT<F>>::C) -> U {
        let v = fs(self);
        Box::new(v)
    }
}

pub trait Monad<U>: Applicative<U> {
    fn bind<F>(&self, x: F) -> Self::T where F : FnMut(&Self::C) -> Self::T;

    fn return_(x: U) -> Self::T where Self: HKT<U, C=U> {
        Self::pure_(x)
    }

    fn join<T>(&self) -> T where Self: HKT<U, T=T, C=T>, T: Clone {
        self.bind(|x| x.clone())
    }
}

impl<T, U> Monad<U> for Vec<T> {
    fn bind<F>(&self, mut f: F) -> Vec<U> where F : FnMut(&T) -> Vec<U> {
        let mut result = vec![];
        for x in self {
            let v = f(x);
            result.extend(v);
        }
        result
    }
}

impl<T, U> Monad<U> for Option<T> {
    fn bind<F>(&self, mut f: F) -> Option<U> where F : FnMut(&T) -> Option<U> {
        match *self {
            Some(ref value) => f(value),
            None => None,
        }
    }
}

impl<T, U> Monad<U> for Rc<T> {
    fn bind<F>(&self, mut f: F) -> Rc<U> where F: FnMut(&T) -> Rc<U> {
        f(self)
    }
}

impl<T, U> Monad<U> for Box<T> {
    fn bind<F>(&self, mut f: F) -> Box<U> where F: FnMut(&T) -> Box<U> {
        f(self)
    }
}

pub trait New<T> {
    fn new(_:T) -> Self;
}

impl<T> New<T> for Box<T> { fn new(v: T) -> Box<T> { Box::new(v) } }
impl<T> New<T> for Rc<T> { fn new(v: T) -> Rc<T> { Rc::new(v) } }

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use super::*;

    pub struct Foo<T, P> where P: HKT<T> {
        #[allow(unused)]
        ptr: P::T
    }

    impl<T, P> Foo<T, P> where P: HKT<T, C=()>, P::T: New<T> {
        fn new(v: T) -> Self {
            let p: P::T = P::T::new(v);
            Foo { ptr: p }
        }
    }

    #[test]
    fn test() {
        let v = Vec::return_(1);
        let v = v.bind(|x| vec![x.to_string(); 3]);
        println!("{:?}", v);

        let o = Option::return_(1);
        let o = o.bind(|&x| Some(x + 1));
        println!("{:?}", o);

        let o = Option::pure_(1);
        let o = o.bind(|&x| Some(x + 1));
        println!("{:?}", o);

        let rc = Rc::return_(7);
        let rc = rc.bind(|&x| Rc::new(x * 3));
        println!("{:?}", rc);

        let b = Box::return_(7);
        let b = b.bind(|&x| Box::new(x * 4));
        println!("{:?}", b);

        let o = Some(Some(true));
        let o = o.join();
        println!("{:?}", o);

        let v = vec![vec!(true), vec!(false)];
        let v = v.join();
        println!("{:?}", v);

        let f1: &Fn(&i32) -> i32 = &|x| x*3;
        let f = Some(f1);
        let o = Some(3);
        let o = o.seq(f);
        println!("{:?}", o);

        let f: Foo<_, Box<_>> = Foo::new(5);
        let p: Box<_> = f.ptr;
        println!("{:?}", p);

        let f: Foo<_, Rc<_>> = Foo::new("5".to_string());
        let p: Rc<_> = f.ptr;
        println!("{:?}", p);

        assert!(true);
    }
}
