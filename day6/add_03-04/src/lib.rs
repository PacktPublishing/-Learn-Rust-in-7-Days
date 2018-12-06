use sqlite::Error as SqErr;
use bcrypt::BcryptError;

#[derive(Debug)]
pub enum UBaseErr{
    DbErr(SqErr),
    HashError(BcryptError),
}

impl From<SqErr> for UBaseErr{
    fn from(s:SqErr)->Self{
        UBaseErr::DbErr(s)
    }
}

impl From<BcryptError> for UBaseErr{
    fn from(b:BcryptError)->Self{
        UBaseErr::HashError(b)
    }
}

#[derive(Debug,PartialEq)]
pub struct UserBase {
    fname:String,
}


impl UserBase {
    pub fn new(f:&str)->Self{
        UserBase{fname:f.to_string()}
    }
    pub fn add_user(&self,u_name:&str, p_word:&str)->Result<(),UBaseErr>{
        let conn = sqlite::open(&self.fname)?; 
        let hpass = bcrypt::hash(p_word,8)?;

        let mut st = conn.prepare("insert into users (u_name,p_word) values (?,?);")?;
        st.bind(1,u_name)?;
        st.bind(2,&hpass as &str)?;
        st.next()?;
        Ok(())
    }


    pub fn validate_password(&self, u_name:&str, p_word:&str)->Result<bool,UBaseErr>{
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare("select p_word from users where u_name = ?;")?;
        st.bind(1,u_name)?;
        
        if let Ok(sqlite::State::Row) = st.next(){
            let phash = st.read::<String>(0)?;
            return Ok(bcrypt::verify(p_word,&phash)?);
        }
        Ok(false)
    }

    pub fn pay(&self,u_from:&str,u_to:&str,amount:i64)->Result<(),UBaseErr>{
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare(r#"insert into transactions (u_from,u_to,t_date,t_amount) values ( ? , ? , datetime("now"),?);"#)?;
        st.bind(1,u_from)?;
        st.bind(2,u_to)?;
        st.bind(3,amount)?;
        st.next()?;
        Ok(())
    }

    pub fn get_balance(&self,u_name:&str)->Result<i64,UBaseErr>{
        let mut res = 0;
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare("select t_amount from transactions where u_to =?;")?;
        st.bind(1,u_name)?;
        while let Ok(sqlite::State::Row) = st.next(){
            res += st.read::<i64>(0)?;
        }

        let mut st = conn.prepare("select t_amount from transactions where u_from =?;")?;
        st.bind(1,u_name)?;
        while let Ok(sqlite::State::Row) = st.next(){
            res -= st.read::<i64>(0)?;
        }

        Ok(res)
    }

}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn add_user_test(){
        let fname = "test_data/users.db";
        //prepare for tests
        let conn = sqlite::open(fname).unwrap();
        let mut st = conn.prepare("delete from users").unwrap();
        st.next().unwrap();
        let mut st = conn.prepare("delete from transactions").unwrap();
        st.next().unwrap();

        //begin tests
        let ub = UserBase{fname:fname.to_string()};

        ub.add_user("matt","mattpw").unwrap();
        assert!(ub.validate_password("matt","mattpw").unwrap());
        assert!(! ub.validate_password("matt","mattw").unwrap());

        ub.pay("matt","dave",30).unwrap();
        ub.pay("pete","matt",20).unwrap();
        assert_eq!(ub.get_balance("matt").unwrap(),-10);
    }
}
