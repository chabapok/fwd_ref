use std::cell::Cell;
use std::ops::Deref;

pub struct Smallest<T>(Cell< Option<*const T> >);

impl<T> Deref for Smallest<T> {
    type Target = T;

    fn deref(&self) -> &T {
        if let Some(p) = self.0.get() {
            unsafe { &*p }
        } else{
            panic!("NPE");
        }
    }
}


pub struct Guard<'a, T:'a>(&'a Smallest<T>);

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        (self.0).0.set(None);
    }
}


impl<T> Smallest<T> {
    pub fn new() -> Smallest<T> {
        Smallest(Cell::new(None))
    }

    pub fn init<'a>(&'a self, v: &T) -> Guard<'a, T> {
        self.0.set(Some(v as *const _));
        Guard(self)
    }

    #[inline]
    pub fn exec_on<F, X, R, R1>(&self, f: F, x: X) -> Result<R, R1>
        where F: Fn(&T)->R, X: Fn()->R1 {
        match self.0.get() {
            Some(r) => Ok(f(unsafe{&*r})),
            None => Err(x())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Bar {
        x: u8
    }

//test test test

    impl Bar {
        fn do_some(&self) {
            println!("do_some");
        }
    }


    struct Foo{
        a: Smallest<Bar>
    }

    #[test]
    fn it_works() {
        let foo = Foo { a: Smallest::new() };
        let bar = Bar { x: 10 };
        {
            let guard = foo.a.init(&bar);
            foo.a.exec_on(|b| b.do_some(), || {});
        }
    }


    #[test]
    #[should_panic(expected = "NPE")]
    fn it_works2() {
        let foo = Foo { a: Smallest::new() };
        foo.a.do_some();
    }


}
