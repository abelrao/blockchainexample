
mod traffic {
    pub trait TrafficTime {
        fn get_time(&self) -> i32;
    }

    pub enum TrafficLight {
        Red(i32),
        Green(i32),
        Yellow(i32),
    }


    impl TrafficTime for TrafficLight {
        fn get_time(&self) -> i32 {
            return match self {
                TrafficLight::Red(tr) => {
                    *tr
                }
                TrafficLight::Green(tg) => {
                    *tg
                }
                TrafficLight::Yellow(yt) => {
                    *yt
                }
            };
        }
    }
}


#[cfg(test)]
mod traffic_test {
    use crate::traffic_light::traffic;
    use crate::traffic_light::traffic::TrafficTime;

    #[test]
    fn test() {
        let green_light = traffic::TrafficLight::Green(15);
        println!("greenLight: {}", green_light.get_time());

        let red_light = traffic::TrafficLight::Red(10);
        println!("redLight: {}", red_light.get_time());
        
        let yellow_light = traffic::TrafficLight::Yellow(5);
        println!("yellowLight: {}", yellow_light.get_time());
    }
}


