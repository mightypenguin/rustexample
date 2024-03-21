
// From: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/traits.html

pub trait HasArea {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
        let c = crate::example_traits::Circle {
            x: 0.0f64,
            y: 0.0f64,
            radius: 4.2f64
        };
        //println!("Circle's area is:{}", c.area());
        assert_eq!(c.area(), 55.41769440932395,"We expect area == 55.41769440932395");
    }
}
