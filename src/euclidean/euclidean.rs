use anyhow::{Result, Ok};

pub struct Euclid;

impl Euclid {
  pub fn _neg<Math0:PartialOrd + From<i32>>(&self, num: Math0) -> Result<bool> {
    let zero = Math0::from(0);
    if num > zero{return Ok(false);}
    Ok(true)
  }
  
  pub fn _pos<Math0: PartialOrd + From<i32>>(&self, num: Math0) -> Result<bool>{
    let zero = Math0::from(0);
    if num < zero{return Ok(false);}
    Ok(true)
  }
  
  pub fn _custom_euclid(&self, x: i32, y:i32, pk: i32){
    let primes = (x, y);
    let max_primes = x * y;
    dbg!(primes);
    let qp = (x - 1) * (y - 1);
    let min_val = 0;
    let z = qp % pk;
    if z != 0 {
      let c = pk % z;
      if c != min_val {
        let a = z % c;
        let c = pk - (qp % pk);
        dbg!(c, a);
        let neg_c = self._neg(c).unwrap();
        if neg_c {
          let modinv = max_primes - &c;
          let (pv, sv) = ((modinv, max_primes),(pk, max_primes));
          dbg!(pv, sv);
        }
      }
    }
  }
}
