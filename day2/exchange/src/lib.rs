
#[derive(PartialEq,Debug)]
pub struct USD (i32);
#[derive(PartialEq,Debug)]
pub struct GBP (i32);
#[derive(PartialEq,Debug)]
pub struct CAD (i32);

pub trait ToUSDv<F>{
    fn to_uv(&self,f:F)->f32;
}

pub trait FromUSDv<F>{
    fn from_uv(&self,f:f32)->F;
}

pub struct Ex {
    cad:f32,
    gbp:f32,
}

impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g:GBP)->f32{
        (g.0 as f32) * self.gbp 
    }
}

impl FromUSDv<CAD> for Ex{
    fn from_uv(&self,f:f32)->CAD{
        CAD((f /self.cad) as i32)
    }
}


pub trait Exchange<F,T>{
    fn convert(&self, f:F)->T;
}

impl<E,F,T> Exchange<F,T> for E 
    where E:ToUSDv<F> + FromUSDv<T>
{
    fn convert(&self,f:F)->T {
        self.from_uv(self.to_uv(f)) 
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = GBP(200);
        let ex = Ex{cad:0.7,gbp:1.3};
        let c:CAD = ex.convert(g);

        assert_eq!(c, CAD(371));
    }
}
