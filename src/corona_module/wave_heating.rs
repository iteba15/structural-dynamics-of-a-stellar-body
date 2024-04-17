extern crate rand;
extern crate plotters;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use plotters::prelude::*;
use plotters::style::RGBColor;

const WHITE: RGBColor = RGBColor(255, 255, 255);
const RED: RGBColor = RGBColor(255, 0, 0);
const BLUE: RGBColor = RGBColor(0, 0, 255);
const GREEN: RGBColor = RGBColor(0, 255, 0);


// Define structs for Particle and MagneticField
struct Particle {
    position: f64,
    velocity: f64,
    mass: f64,
    charge: f64,
    radius: f64,
    spin: f64,
}

impl Particle {
    fn new(position: f64, velocity: f64, mass: f64, charge: f64, radius: f64, spin: f64) -> Self {
        Particle {
            position,
            velocity,
            mass,
            charge,
            radius,
            spin,
        }
    }
}
struct MagneticField {
    strength: Vec<f64>,
    direction: Vec<(f64, f64, f64)>, // Vector representing the direction of the magnetic field at each point
}

impl MagneticField {
    fn new(num_cells: usize) -> Self {
        MagneticField {
            strength: vec![0.0; num_cells], // Initialize magnetic field strength
            direction: vec![(0.0, 0.0, 1.0); num_cells], // Initialize with a uniform magnetic field pointing in the z-direction
        }
    }
}
// Define struct for AlfvenWave
struct AlfvénWave {
    position: f64,
    amplitude: f64,
    phase: f64,
    frequency: f64,
    velocity: f64, // Or any other relevant fields
}

impl AlfvénWave {
    // Constructor for AlfvenWave
    fn new(position: f64, amplitude: f64, phase: f64, frequency: f64, velocity: f64) -> Self {
        AlfvénWave {
            position,
            amplitude,
            phase,
            frequency,
            velocity,
        }
    }
}
// Define struct for AcousticWave
struct AcousticWave {
    position: f64,
    velocity: f64, // Or any other relevant fields
}

impl AcousticWave {
    // Constructor for AcousticWave
    fn new(position: f64, velocity: f64) -> Self {
        AcousticWave { position, velocity }
    }
}

// Define struct for MagnetoAcousticWave
struct MagnetoAcousticWave {
    position: f64,
    velocity: f64, // Or any other relevant fields
}

impl MagnetoAcousticWave {
    // Constructor for MagnetoAcousticWave
    fn new(position: f64, velocity: f64) -> Self {
        MagnetoAcousticWave { position, velocity }
    }
}

// Continuing with your existing code...
// Define Simulation struct to hold waves, particles, magnetic field, and simulation data

struct Simulation {
    particles: Vec<Particle>,
    magnetic_field: MagneticField,
    dt: f64,
    total_time: f64,
    num_particles: usize,
    num_cells: usize,
    num_alfven_waves: usize,
    num_acoustic_waves: usize,
    num_magneto_acoustic_waves: usize,
    alfven_waves: Vec<AlfvénWave>,
    acoustic_waves: Vec<AcousticWave>,
    magneto_acoustic_waves: Vec<MagnetoAcousticWave>,
}

impl Simulation {
    fn new(num_particles: usize, num_cells: usize, num_alfven_waves: usize, num_acoustic_waves: usize, num_magneto_acoustic_waves: usize, dt: f64, total_time: f64) -> Self {
        let mut particles = Vec::with_capacity(num_particles);
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            let position = rng.gen_range(0.0..num_cells as f64);
            let velocity = 0.0; // Initialize velocities to zero
            let mass = 1.0; // Example mass
            let charge = 1.0; // Example charge
            let radius = 0.1; // Example radius
            let spin = 0.5; // Example spin
            particles.push(Particle::new(position, velocity, mass, charge, radius, spin));
        }

        let magnetic_field = MagneticField::new(num_cells);

        Simulation {
            particles,
            magnetic_field,
            dt,
            total_time,
            num_particles,
            num_cells,
            num_alfven_waves,
            num_acoustic_waves,
            num_magneto_acoustic_waves,
            alfven_waves: Vec::new(),
            acoustic_waves: Vec::new(),
            magneto_acoustic_waves: Vec::new(),
        }
    }

    // Function to initialize particles
    fn initialize_particles(&mut self) {
        let mut rng = rand::thread_rng();
        let position_dist = Uniform::new(0.0, 100.0);
        let velocity_dist = Uniform::new(-1.0, 1.0);

        for _ in 0..self.num_particles {
            let position = position_dist.sample(&mut rng);
            let velocity = velocity_dist.sample(&mut rng);
            self.particles.push(Particle::new(position, velocity, 1.0, 1.0, 0.1, 0.5));
        }
    }

    // Function to initialize the magnetic field
    fn initialize_magnetic_field(&mut self) {
        self.magnetic_field.strength = vec![1.0; self.num_cells];
    }
    fn initialize_alfven_waves(&mut self) {
        let mut rng = rand::thread_rng();
        let position_dist = Uniform::new(0.0, 100.0);
        let amplitude_dist = Uniform::new(0.0, 1.0);
        let phase_dist = Uniform::new(0.0, 2.0 * std::f64::consts::PI);
        let frequency_dist = Uniform::new(0.0, 1.0);
        let velocity_dist = Uniform::new(-1.0, 1.0);

        for _ in 0..self.num_alfven_waves {
            let position = position_dist.sample(&mut rng);
            let amplitude = amplitude_dist.sample(&mut rng);
            let phase = phase_dist.sample(&mut rng);
            let frequency = frequency_dist.sample(&mut rng);
            let velocity = velocity_dist.sample(&mut rng);
            self.alfven_waves.push(AlfvénWave::new(position, amplitude, phase, frequency, velocity));
        }
    }
    // Function to initialize acoustic waves
    fn initialize_acoustic_waves(&mut self) {
        let mut rng = rand::thread_rng();
        let position_dist = Uniform::new(0.0, 100.0);
        let velocity_dist = Uniform::new(-1.0, 1.0);

        for _ in 0..self.num_acoustic_waves {
            let position = position_dist.sample(&mut rng);
            let velocity = velocity_dist.sample(&mut rng);
            self.acoustic_waves.push(AcousticWave::new(position, velocity));
        }
    }

    // Function to initialize magneto-acoustic waves
    fn initialize_magneto_acoustic_waves(&mut self) {
        let mut rng = rand::thread_rng();
        let position_dist = Uniform::new(0.0, 100.0);
        let velocity_dist = Uniform::new(-1.0, 1.0);

        for _ in 0..self.num_magneto_acoustic_waves {
            let position = position_dist.sample(&mut rng);
            let velocity = velocity_dist.sample(&mut rng);
            self.magneto_acoustic_waves.push(MagnetoAcousticWave::new(position, velocity));
        }
    }

    // Function for time integration
fn time_integration(&mut self) {
    for _ in 0..(self.total_time / self.dt) as usize {
        // Update particle positions and velocities
        for particle in &mut self.particles {
            // Example: Update particle position based on velocity
            particle.position += particle.velocity * self.dt;
            // Example: Update particle velocity based on forces acting on it
            // (not implemented here, as it depends on the specific dynamics of your system)
        }

        // Update wave properties (e.g., amplitude, phase)
        for wave in &mut self.alfven_waves {
            // Example: Update wave amplitude and phase
            wave.amplitude = wave.amplitude * (1.0 - 0.1 * self.dt); // Example decay
            wave.phase += 2.0 * std::f64::consts::PI * wave.frequency * self.dt;
        }

        // Update magnetic field strength (e.g., based on wave-induced fluctuations)
        for strength in &mut self.magnetic_field.strength {
            // Example: Update magnetic field strength based on wave-induced fluctuations
            *strength += 0.1 * self.dt; // Example increase
        }
    }
}

// Function for visualization
fn plot_results(&self) {
    // Create a new plot
    let root = BitMapBackend::new("simulation_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Plot particle positions
    let mut chart = ChartBuilder::on(&root)
        .caption("Particle Positions", ("sans-serif", 20).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..100.0, 0.0..100.0)
        .unwrap();

    chart
        .draw_series(self.particles.iter().map(|particle| {
            Circle::new((particle.position, 0.0), 5, RED.filled())
        }))
        .unwrap();

    // Plot wave amplitudes over time
    let mut wave_chart = ChartBuilder::on(&root)
        .caption("Wave Amplitudes", ("sans-serif", 20).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..self.total_time, 0.0..1.0)
        .unwrap();

    wave_chart
        .configure_mesh()
        .x_desc("Time")
        .y_desc("Amplitude")
        .draw()
        .unwrap();

    wave_chart
        .draw_series(LineSeries::new(
            self.alfven_waves.iter().map(|wave| (wave.phase, wave.amplitude)),
            &BLUE,
        ))
        .unwrap();

    // Plot magnetic field strength over time
    let mut mag_field_chart = ChartBuilder::on(&root)
        .caption("Magnetic Field Strength", ("sans-serif", 20).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..self.total_time, 0.0..1.0)
        .unwrap();

    mag_field_chart
        .configure_mesh()
        .x_desc("Time")
        .y_desc("Field Strength")
        .draw()
        .unwrap();

    mag_field_chart
        .draw_series(LineSeries::new(
            self.magnetic_field.strength.iter().enumerate().map(|(i, &strength)| (i as f64 * self.dt, strength)),
            &GREEN,
        ))
        .unwrap();
}

    // Function for analysis
    fn analyze_results(&self) {
        // Calculate maximum and average particle velocity
        let max_velocity = self.particles.iter().map(|particle| particle.velocity).fold(f64::NEG_INFINITY, f64::max);
        let average_velocity: f64 = self.particles.iter().map(|particle| particle.velocity).sum::<f64>() / self.particles.len() as f64;
        println!("Maximum particle velocity: {}", max_velocity);
        println!("Average particle velocity: {}", average_velocity);

        // Calculate maximum and average wave amplitude
        let max_amplitude = self.alfven_waves.iter().map(|wave| wave.amplitude).fold(f64::NEG_INFINITY, f64::max);
        let average_amplitude: f64 = self.alfven_waves.iter().map(|wave| wave.amplitude).sum::<f64>() / self.alfven_waves.len() as f64;
        println!("Maximum wave amplitude: {}", max_amplitude);
        println!("Average wave amplitude: {}", average_amplitude);

        // Calculate maximum and average magnetic field strength
        let max_field_strength = self.magnetic_field.strength.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let average_field_strength: f64 = self.magnetic_field.strength.iter().sum::<f64>() / self.magnetic_field.strength.len() as f64;
        println!("Maximum magnetic field strength: {}", max_field_strength);
        println!("Average magnetic field strength: {}", average_field_strength);
    }
}

fn main() {
    // Define simulation parameters
    let num_alfven_waves = 100;
    let num_acoustic_waves = 100;
    let num_magneto_acoustic_waves = 100;
    let num_particles = 100;
    let num_cells = 1000;
    let dt = 0.01;
    let total_time = 10.0;

    // Create a new simulation
    let mut simulation = Simulation::new(num_particles, num_cells, num_alfven_waves, num_acoustic_waves, num_magneto_acoustic_waves, dt, total_time);

    // Initialize waves, particles, and magnetic field
    simulation.initialize_alfven_waves();
    simulation.initialize_acoustic_waves();
    simulation.initialize_magneto_acoustic_waves();
    simulation.initialize_particles();
    simulation.initialize_magnetic_field();

    // Perform time integration
    simulation.time_integration();

    // Plot and visualize the results
    simulation.plot_results();

    // Analyze the results
    simulation.analyze_results();
}