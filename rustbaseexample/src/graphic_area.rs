pub trait Area {
    fn area(&self) -> f64;
}

pub struct Triangle {
    pub h: f64,
    pub d: f64,
}

impl Triangle {
    pub fn new(h: f64, d: f64) -> Self {
        Triangle { h, d }
    }
}


impl Area for Triangle {
    fn area(&self) -> f64 {
        return (self.h * self.d) / 2 as f64;
    }
}


pub struct Circle {
    pub r: f64
}

impl Circle {
    pub fn new(r: f64) -> Self {
        return Circle { r };
    }
}


impl Area for Circle {
    fn area(&self) -> f64 {
        return std::f64::consts::PI * self.r * self.r;
    }
}


pub fn get_area<T: Area>(value: &T) -> f64 {
    return value.area();
}


#[cfg(test)]
mod test {
    use crate::graphic_area::{Triangle, get_area, Circle};

    #[test]
    fn test() {
        let triangle = Triangle::new(2.0, 3.0);
        let tr_area = get_area(&triangle);
        println!("tr area: {}", tr_area);

        let circle = Circle::new(2 as f64);
        let cir_area = get_area(&circle);
        println!("circle area: {}", cir_area)
    }
}