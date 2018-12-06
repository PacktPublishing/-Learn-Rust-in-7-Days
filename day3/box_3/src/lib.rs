
#[derive(Debug,PartialEq)]
pub enum LinkedList<T>{
    Tail,
    Head(T,Box<LinkedList<T>>),
}

use self::LinkedList::*;

impl<T> LinkedList<T>{
    pub fn empty()->Self{
        Tail
    }

    pub fn new(t:T)->Self{
        Head(t,Box::new(Tail))
    }

    pub fn push(self,t:T)->Self {
        Head(t,Box::new(self))
    }    

    pub fn push_back(&mut self, t:T){
        match self {
            Tail => {
                *self = LinkedList::new(t);
            },
            Head(_,n)=>{
                n.push_back(t);
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut l = LinkedList::new(3);
        l = l.push(4);
        assert_eq!(l,Head(4,Box::new(Head(3,Box::new(Tail)))));
        l.push_back(2);

        assert_eq!(l,Head(4,Box::new(Head(3,Box::new(Head(2,Box::new(Tail)))))));
        
    }
}
