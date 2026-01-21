use graviplex::{
    Camera2D, CirclePipeline, GameLoop, GpuContext, InputState, PhysicsInstance, Vertex,
};

use crate::nbody::GpuEngine;

/// N-body simulation game.
pub struct NBodyGame {
    pub particle_count: u32,
    pub gpu_engine: Option<GpuEngine>,
    pub pipeline: Option<CirclePipeline>,
    pub gravity: f32,
    pub theta: f32,
    pub show_quadtree: bool,
}

impl NBodyGame {
    /// Create a new n-body simulation with the given particle count.
    pub fn new(particle_count: u32) -> Self {
        Self {
            particle_count,
            gpu_engine: None,
            pipeline: None,
            gravity: 500.0,
            theta: 0.5,
            show_quadtree: false,
        }
    }
}

impl GameLoop for NBodyGame {
    fn init(&mut self, gpu: &GpuContext) {
        let engine = GpuEngine::new(&gpu.device, &gpu.queue, self.particle_count);
        engine.init(&gpu.queue);
        self.gpu_engine = Some(engine);

        let format = gpu.config.as_ref().unwrap().format;
        self.pipeline = Some(CirclePipeline::with_instance_layout(
            &gpu.device,
            format,
            &Camera2D::new([0.0, 0.0], 1.0, [1.0, 1.0]),
            PhysicsInstance::desc(),
        ));
    }

    fn update(&mut self, dt: f32, gpu: &GpuContext) {
        if let Some(engine) = &self.gpu_engine {
            engine.update(&gpu.device, &gpu.queue, dt, self.gravity, self.theta);
        }
    }

    fn render(&mut self, gpu: &GpuContext, view: &wgpu::TextureView, camera: &Camera2D) {
        if let (Some(engine), Some(pipeline)) = (&self.gpu_engine, &self.pipeline) {
            let vertices = vec![
                Vertex { pos: [0.0, 5.0] },
                Vertex { pos: [4.33, -2.5] },
                Vertex { pos: [-4.33, -2.5] },
            ];

            pipeline.camera_gpu_data().update(&gpu.queue, camera);

            pipeline.render_with_external_buffer(
                &gpu.device,
                &gpu.queue,
                view,
                camera,
                &vertices,
                self.particle_count,
                &engine.particle_buffer,
            );
        }
    }

    fn handle_input(&mut self, _input: &InputState, _camera: &Camera2D) -> bool {
        false
    }

    fn gui(&mut self, ctx: &egui::Context) {
        egui::Window::new("N-Body Simulation")
            .default_width(280.0)
            .show(ctx, |ui| {
                ui.heading("Physics");
                ui.add(egui::Slider::new(&mut self.gravity, 0.0..=1000.0).text("Gravity"));
                ui.add(egui::Slider::new(&mut self.theta, 0.1..=1.5).text("Theta (Accuracy)"));

                ui.separator();
                ui.heading("Info");
                ui.label(format!("Particles: {}", self.particle_count));

                ui.separator();
                ui.heading("Debug");
                ui.checkbox(&mut self.show_quadtree, "Show Quadtree");
            });
    }
}
