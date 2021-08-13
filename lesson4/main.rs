enum Option<u32>{
    None,
    Some(u32)
}

fn sum(v:&Vec<u32>)->Option<u32>{
    let mut sum:u32=0;
    let mut overflow_ind=0;

    for i in v{
        if sum.overflowing_add(*i).1 {
            overflow_ind=1;
            break
        }
        sum=sum+i;      
    }

    match overflow_ind {
      0=>Option::Some(sum),
      _=>Option::None
    }
}


fn main() {

    let v1:Vec<u32>=vec![1,2,3,4];
    let v2:Vec<u32>=vec![3,4,5,4294967295];
   

    let result1=sum(&v1);
    let result2=sum(&v2);

     match result1{
        Option::Some(x)=>println!("sum is {}",x),
        Option::None=>println!("None")
     }  

     match result2{
        Option::Some(x)=>println!("sum is {}",x),
        Option::None=>println!("None")
     }  

}
