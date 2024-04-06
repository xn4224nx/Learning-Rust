use graphics::math::{Vec2d, add, mul_scalar};
use piston_window::*;
use rand::prelude::*;
use std::alloc::{GlobalAlloc, System, Layout};
use std::time::Instant;

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc (&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();
        
        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        return ptr;
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

/* Program Lifetime Data */
struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

/* Defines an Object in 2D Space */
struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    colour: [f32; 4], 
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        
        /* Start the particle at a rnd position at the bottom of the screen. */
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        
        /* The particle rises over time. */
        let x_vel = 0.0;
        let y_vel = rng.gen_range(-2.0..0.0);
        
        /* It rises faster over time. */
        let x_acc = 0.0;
        let y_acc = rng.gen_range(0.0..0.15);
        
        return Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),
            velocity: [x_vel, y_vel].into(),
            acceleration: [x_acc, y_acc].into(),
            /* A fully saturated white with a little transparancy */
            colour: [1.0, 1.0, 1.0, 0.99],
        }
    }
        
    fn update(&mut self) {
        /* Move the particle onto its next position. */
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        
        /* Slow down its acceleration */
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        
        /* Make the particle more transparant */
        self.colour[3] *= 0.995;
    }
}

impl World {
    fn new(width: f64, height: f64) -> World {
        return World {
            current_turn: 0,
            particles: Vec::<Box<Particle>>::new(),
            height: height,
            width: width,
            rng: thread_rng(),
        }
    }
    
    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            /* Create a particle on the stack. */
            let particle = Particle::new(&self);
            
            /* Take ownership of object and move it to the heap. */
            let boxed_particle = Box::new(particle);
            
            /* Push the reference onto self.shape */
            self.particles.push(boxed_particle);
        }
    }
    
    fn remove_shapes(&mut self, n:i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;
            
            /* Find the first transparant particle */
            for (i, particle) in self.particles.iter().enumerate() {
                if particle.colour[3] < 0.02 {
                    to_delete = Some(i);
                }
                break;
            }
            
            /* Delete the first invisible particle or the oldest */
            if let Some(i) = to_delete {
                self.particles.remove(i);
            } else {
                self.particles.remove(0);
            };
        }
    }
    
    fn update(&mut self) {
        /* Generate an integer between -3 and +3 inclusively. */
        let n = self.rng.gen_range(-3..=3);
        
        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }
        
        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }
        
        self.current_turn += 1;
    }
}

fn main() {
    let (width, height) = (1280.0, 960.0);
    
    let mut window: PistonWindow = WindowSettings::new(
        "particles", [width, height]
    )
    .exit_on_esc(true)
    .build()
    .expect("Could not create a window!");
    
    let mut world = World::new(width, height);
    world.add_shapes(1000);
    
    while let Some(event) = window.next() {
        world.update();
        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);
            
            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.colour, size, ctx.transform, renderer);
            }
        });
    }
}
