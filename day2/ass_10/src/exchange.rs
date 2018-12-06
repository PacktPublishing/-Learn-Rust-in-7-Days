use currencies::*;

pub trait ToUsdEx<M:Currency>{
    fn to_uv(&self,&M)->f64;
}

pub trait FromUsdEx<M:Currency>{
    fn from_uv(&self,f64)->M;
}

pub trait MoneyConverter<F:Currency,T:Currency>{
    fn convert(&self,f:&F)->T;
}

impl<E,F,T> MoneyConverter<F,T> for E
    where E:ToUsdEx<F> + FromUsdEx<T>,
          F:Currency,
          T:Currency,
{
    fn convert(&self,f:&F)->T{
        self.from_uv(self.to_uv(f))
    }
}

#[derive(Debug,PartialEq)]
pub struct Transaction<ID,T:Currency>{
    pub f_id:ID,
    pub t_id:ID,
    pub amount:T,
}

pub trait AcId<T:Currency,ID>{
    fn ac_id(&self,t:&T)->ID;//here t is just a signal type
}

pub trait Exchange<AIn,AOut,F:Currency,T:Currency>{
    fn exchange(&self,f_ac:AIn,t_ac:AOut,f:F)
        ->(Transaction<AIn,F>,Transaction<AOut,T>);
}

impl<E,AIn,AOut,F,T> Exchange<AIn,AOut,F,T> for E 
    where E:MoneyConverter<F,T> + AcId<F,AIn> + AcId<T,AOut>,
        F:Currency,
        T:Currency,
    
{
    fn exchange(&self,f_ac:AIn,t_ac:AOut,f:F)
        ->(Transaction<AIn,F>,Transaction<AOut,T>){
        let rv = self.convert(&f);
        let f_id = self.ac_id(&f);
        let t_id = self.ac_id(&rv);
        (  
            Transaction{f_id:f_ac,t_id:f_id,amount:f},
            Transaction{f_id:t_id,t_id:t_ac,amount:rv}
        )
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    struct TEx{}
    impl ToUsdEx<Money<GBP>> for TEx{
        fn to_uv(&self,g:&Money<GBP>)->f64{
            (g.value() as f64) * 1.3
        }
    }
    impl FromUsdEx<Money<CAD>> for TEx{
        fn from_uv(&self,u:f64)->Money<CAD>{
            Money::from_value((u*1.3)as i32)
        }
    }
    impl AcId<Money<GBP>,i32> for TEx{
        fn ac_id(&self,_:&Money<GBP>)->i32{10}
    }

    impl AcId<Money<CAD>,i32> for TEx{
        fn ac_id(&self,_:&Money<CAD>)->i32{20}
    }

    #[test]
    pub fn convert_money(){
        let gbp:Money<GBP> = Money::from_value(200);
        assert_eq!(gbp.to_string(),"£2.00");
        let ex = TEx{};
        let (tout,tto) = ex.exchange(5,6,gbp);
        assert_eq!(tout,Transaction{f_id:5,t_id:10,amount:Money::from_value(200)});
        assert_eq!(tto,Transaction{f_id:20,t_id:6,amount:Money::from_value(338)});
    }

} 
