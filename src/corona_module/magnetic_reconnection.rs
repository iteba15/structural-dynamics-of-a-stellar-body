// Import necessary crates
extern crate rand;
extern crate plotters;

use rand::Rng;
use plotters::prelude::*;

// Define structs for Particle and MagneticField
struct Particle {
    position: f64,
    velocity: f64,
}

impl Particle {
    fn new(position: f64, velocity: f64) -> Self {
        Particle { position, velocity }
    }
}

struct MagneticField {
    strength: Vec<f64>,
}

impl MagneticField {
    fn new(num_cells: usize) -> Self {
        MagneticField {
            strength: vec![0.0; num_cells], // Initialize magnetic field strength
        }
    }
}

// Define Simulation struct to hold particles, magnetic field, and simulation data
struct Simulation {
    particles: Vec<Particle>,
    magnetic_field: MagneticField,
    dt: f64,
    total_time: f64,
    reconnection_events: usize, // Counter for reconnection events
    time_series_data: Vec<(f64, f64)>, // Time series data for average magnetic field strength
}

impl Simulation {
    fn new(num_particles: usize, num_cells: usize, dt: f64, total_time: f64) -> Self {
        let mut particles = Vec::with_capacity(num_particles);
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            let position = rng.gen_range(0.0..num_cells as f64);
            let velocity = 0.0; // Initialize velocities to zero
            particles.push(Particle::new(position, velocity));
        }

        let magnetic_field = MagneticField::new(num_cells);

        Simulation {
            particles,
            magnetic_field,
            dt,
            total_time,
            reconnection_events: 0, // Initialize reconnection events counter
            time_series_data: Vec::new(), // Initialize time series data vector
        }
    }

    fn initialize_magnetic_field(&mut self) {
        // Initialize a uniform magnetic field along the corona
        for strength in self.magnetic_field.strength.iter_mut() {
            *strength = 1.0; // Set magnetic field strength to 1.0 for all cells
        }
    }

    fn determine_initial_particle_distribution(&mut self) {
        // Initialize particles with random positions and velocities
        let mut rng = rand::thread_rng();
        for particle in self.particles.iter_mut() {
            particle.position = rng.gen_range(0.0..100.0); // Initialize particle position
            particle.velocity = rng.gen_range(-1.0..1.0); // Initialize particle velocity
        }
    }

    fn time_integration(&mut self) {
        // Perform time integration to evolve the system forward in time
        let mut time = 0.0;
        let mut total_strength = 0.0;
        let mut num_measurements = 0;
        while time < self.total_time {
            self.calculate_magnetic_field_evolution();
            self.model_particle_dynamics();
            self.detect_reconnection_events();
            self.update_magnetic_field_and_particles();

            // Measure the average magnetic field strength every 0.1 units of time
            if (time % 0.1).abs() < std::f64::EPSILON {
                total_strength += self.magnetic_field.strength.iter().sum::<f64>();
                num_measurements += 1;
            }

            // Implement an adaptive time stepping method
            // For simplicity, let's adjust the time step based on the maximum magnetic field strength
            let max_strength = self.magnetic_field.strength.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            self.dt = 0.01 / (1.0 + max_strength); // Decrease time step if magnetic field strength is high

            time += self.dt;
        }

        // Calculate and store average magnetic field strength
        let average_strength = total_strength / (num_measurements as f64 * self.magnetic_field.strength.len() as f64);
        self.time_series_data.push((self.total_time, average_strength));
    }

    fn calculate_magnetic_field_evolution(&mut self) {
        // Get the number of cells
        let num_cells = self.magnetic_field.strength.len();

        // Create a new vector to store the updated magnetic field strengths
        let mut new_strengths = vec![0.0; num_cells];

        // Apply finite differences to update the magnetic field strengths
        for i in 2..(num_cells - 2) {
            // Use a fourth-order central difference scheme for the spatial derivatives
            let d2_b_dx2 = (-self.magnetic_field.strength[i + 2] + 16.0 * self.magnetic_field.strength[i + 1] - 30.0 * self.magnetic_field.strength[i] + 16.0 * self.magnetic_field.strength[i - 1] - self.magnetic_field.strength[i - 2]) / (12.0 * self.dt * self.dt);

            // Update the magnetic field strength based on the FDTD method
            new_strengths[i] = self.magnetic_field.strength[i] + self.dt * d2_b_dx2;
        }

        // Update the magnetic field strengths
        self.magnetic_field.strength = new_strengths;
    }

    fn model_particle_dynamics(&mut self) {
        // Update the particle positions and velocities based on the Lorentz force
        for particle in self.particles.iter_mut() {
            particle.position += particle.velocity * self.dt; // Update particle position
        }
    }

    fn detect_reconnection_events(&mut self) {
        // Detect regions of magnetic reconnection
        for strength in &self.magnetic_field.strength {
            if *strength > 1.5 {
                self.reconnection_events += 1;
            }
        }
    }

    fn update_magnetic_field_and_particles(&mut self) {
        // Update the system when reconnection events are detected
        for strength in self.magnetic_field.strength.iter_mut() {
            if *strength > 1.5 {
                *strength -= 0.5; // Reduce magnetic field strength
                for particle in self.particles.iter_mut() {
                    particle.velocity *= 1.1; // Accelerate particles
                }
            }
        }
    }

    fn plot_time_series_data(&self) {
        // Find the minimum and maximum values of the average magnetic field strength
        let (min_strength, max_strength) = self.time_series_data.iter()
            .map(|(_, strength)| strength)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
                (min.min(val), max.max(val))
            });

    
        // Create a plotter backend
        let root = BitMapBackend::new("time_series_plot.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE).unwrap();
    
        // Create a chart context with dynamically adjusted y-axis range
        let mut chart = ChartBuilder::on(&root)
            .caption("Average Magnetic Field Strength Over Time", ("sans-serif", 20).into_font())
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0.0..self.total_time, min_strength..max_strength)
            .unwrap();
    
        // Draw the time series data
        chart
            .configure_mesh()
            .x_desc("Time")
            .y_desc("Average Magnetic Field Strength")
            .draw()
            .unwrap();
    
        chart
            .draw_series(LineSeries::new(
                self.time_series_data.iter().map(|(x, y)| (*x, *y)),
                &RED,
            ))
            .unwrap();
    }
}

fn main() {
    // Define simulation parameters
    let num_particles = 100;
    let num_cells = 1000;
    let dt = 0.01;
    let total_time = 10.0;

    // Create a new simulation
    let mut simulation = Simulation::new(num_particles, num_cells, dt, total_time);

    // Initialize magnetic field and particle distribution
    simulation.initialize_magnetic_field();
    simulation.determine_initial_particle_distribution();

    // Perform time integration
    simulation.time_integration();

    // Plot time series data
    simulation.plot_time_series_data();

    // Print the number of reconnection events detected
    println!("Total reconnection events: {}", simulation.reconnection_events);
}