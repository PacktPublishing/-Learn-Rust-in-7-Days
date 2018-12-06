use minibankdb::UserBase;
use std::sync::{Arc,Mutex};
use std::collections::HashMap;

pub struct Session{
    pub ub:UserBase,
    data:Arc<Mutex<HashMap<u64,String>>>,
}

impl Session{
    pub fn new(dbfile:&str)->Self{
        Session{
            ub:UserBase::new(dbfile),
            data:Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_session(&self,uname:String)->u64{
        let mut mp = self.data.lock().unwrap();
        let mut r:u64 = rand::random();
        while let Some(_) = mp.get(&r){
            r = rand::random();
        }
        mp.insert(r,uname);
        r
    }

    pub fn get_session(&self,s:u64)->Option<String>{
        let mp = self.data.lock().unwrap();
        mp.get(&s).map(|s|s.clone())
    }

}

#[cfg(test)]
mod test{
    use super::*;
    fn test_sess(){
        let ss = Session::new("");
        let id = ss.add_session("dave".to_string());

        assert_eq!(&ss.get_session(id).unwrap(),"dave");
        assert!(&ss.get_session(3).is_none());

    }
}

