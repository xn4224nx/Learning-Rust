/* A rust structure representing a 3D point. */
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

/* Methods associated with the structure `Point` */
impl Point {
    /// Return an origin point
    fn origin() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Calculate the Manhattan distance from the origin
    fn manhat_dist(&self) -> f64 {
        return self.x + self.y + self.z;
    }
}

fn main() {
    /* Create a structure instance. */
    let mypoint: Point = Point {
        x: 0.4,
        y: -12.7,
        z: 40.5,
    };

    /* Function call */
    println!("{:?}", Point::origin());

    /* Method call on a structure */
    println!("Manhattan Distance = {:.2}", mypoint.manhat_dist());

    println!("{:?}", mypoint);
}
