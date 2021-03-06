use nalgebra::Vector3;

// Math utils

/// I just want things to be able to return unit versions of themselves ok
pub trait Unit {
    fn unit(&self) -> Self;
}

/// So now vectors can >:(
impl Unit for Vector3<f64> {
    fn unit(&self) -> Vector3<f64> {
        self / self.norm()
    }
}
