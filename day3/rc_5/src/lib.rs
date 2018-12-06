use std::rc::Rc;
use std::cell::RefCell;

pub fn make_rc(i:i32)->Rc<RefCell<i32>>{
    let a = Rc::new(RefCell::new(i));

    let b = a.clone();

    let m = &mut *a.borrow_mut();

    let n = &mut *a.borrow_mut();

    *m += 2;

    *n += 2;

    return b;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let r = make_rc(5);
        

        assert_eq!(*r.borrow() ,7);
    }
}
