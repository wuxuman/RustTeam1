
struct Circule{
    radius:f32,
}

struct  Triangle{
    bottom:f32,
    high:f32,
}

struct  Square{
    edge_length:f32,
}

pub trait CalculateArea{
   fn calculate(&self)->f32;
}

impl CalculateArea for Circule{
    fn calculate(&self)->f32{
        let pi:f32=3.14;
        pi*self.radius*self.radius
    }
}

impl CalculateArea for Triangle{
    fn calculate(&self)->f32{
        self.bottom*self.high/2f32
    }
}

impl CalculateArea for Square{
    fn calculate(&self)->f32{
        self.edge_length*self.edge_length
    }
}


pub fn get_area<T:CalculateArea>(item:&T){
    println!("area is {}", item.calculate());

}

fn main() {
    let c=Circule{radius:5.3f32,};
    get_area(&c);

    let t=Triangle{bottom:10f32,high:8f32};
    get_area(&t);

    let s=Square{edge_length:50f32};
    get_area(&s);



}
