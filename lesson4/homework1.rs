
enum TrafficLight{
    Red,
    Green,
    Yellow,
}

trait GetTime{
    fn time(&self) -> u8;
}

impl GetTime for TrafficLight{

    fn time(&self) -> u8{
        match self{
            TrafficLight::Red => 50,
            TrafficLight::Green => 60,
            TrafficLight::Yellow => 3,

        }       
    }
}

fn main() {

    let red_light=TrafficLight::Red;
    let green_light=TrafficLight::Green;
    let yellow_light=TrafficLight::Yellow;

    println!("Red light duration is {} seconds",red_light.time());
    println!("Green light duration is {} seconds",green_light.time());
    println!("Yellow light duration is {} seconds",yellow_light.time());
}
