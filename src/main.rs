#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate cgmath;

use gfx::traits::*;
use cgmath::{Vector2};
use cgmath::EuclideanVector;

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
    normal: [f32; 2] = "a_Normal",
});

gfx_pipeline!(line {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    width: gfx::Global<f32> = "u_Width",
    color: gfx::Global<[f32; 3]> = "u_Color",
    // COLOR THING
    out: gfx::RenderTarget<gfx::format::Rgba8> = "o_Color",
});

trait Norm {
    fn right(&self) -> Vector2<f32>;
}

impl Norm for Vector2<f32> {
    fn right(&self) -> Vector2<f32> {
        Vector2::new(self.y, -self.x)
    }
}

fn point_normal(p: [[f32; 2]; 2]) -> ([f32; 2], [f32; 2]) {
    let vec = Vector2::new(p[1][0] - p[0][0], p[1][1] - p[0][1]);
    let right = vec.right().normalize();
    let mut left = right.clone();
    left.neg_self();
    (left.into(), right.into())
}

fn create_line_buf<R, F>(factory: &mut F, points: [[f32; 2]; 2])
                  -> (gfx::handle::Buffer<R, Vertex>, gfx::Slice<R>)
                  where R: gfx::Resources, F: gfx::Factory<R> {
    let (right, left) = point_normal(points);
    let vertex_data = [
        Vertex { pos: points[0], normal: right },
        Vertex { pos: points[0], normal: left  },
        Vertex { pos: points[1], normal: right },
        Vertex { pos: points[1], normal: left  },
    ];
    factory.create_vertex_buffer(&vertex_data)
}

pub fn main() {
    use gfx::traits::{Device,FactoryExt};
    let builder = glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("Wunder".to_string());
    let (window, mut device, mut factory, main_color, _) =
        // COLOR THING
        gfx_window_glutin::init::<gfx::format::Rgba8, gfx::format::Depth>(builder);
    let mut encoder = factory.create_encoder();

    let line_shaders = factory.create_shader_set(
        include_bytes!("line.glslv"),
        include_bytes!("line.glslf")).unwrap();
    let line_pso = factory.create_pipeline_state(
        &line_shaders,
        gfx::Primitive::TriangleStrip,
        gfx::state::Rasterizer::new_fill(gfx::state::CullFace::Nothing),
        line::new()).unwrap();

    let (line1_buf, line1_slice) = create_line_buf(&mut factory, [[-0.5,  0.9], [ 0.9,  0.5]]);
    let (line2_buf, line2_slice) = create_line_buf(&mut factory, [[ 0.2,  1.0], [-0.2, -1.0]]);
    let (line3_buf, line3_slice) = create_line_buf(&mut factory, [[-0.7, -0.7], [ 0.8, -0.3]]);

    let line1_data = line::Data {
        vbuf: line1_buf,
        width: 0.05,
        color: [0.5, 0.5, 0.0],
        out: main_color.clone(),
    };
    let line2_data = line::Data {
        vbuf: line2_buf,
        width: 0.04,
        color: [0.5, 0.0, 0.0],
        out: main_color.clone(),
    };
    let line3_data = line::Data {
        vbuf: line3_buf,
        width: 0.06,
        color: [0.0, 0.5, 0.0],
        out: main_color.clone(),
    };

    'main: loop {
        for event in window.poll_events() {
            use glutin::Event;
            use glutin::VirtualKeyCode::{Escape};
            match event {
                Event::KeyboardInput(_, _, Some(Escape)) |
                Event::Closed => break 'main,
                _ => (),
            }
        }

        encoder.reset();
        encoder.clear(&main_color, [0.00, 0.00, 0.00, 0.5]);

        encoder.draw(&line1_slice, &line_pso, &line1_data);
        encoder.draw(&line2_slice, &line_pso, &line2_data);
        encoder.draw(&line3_slice, &line_pso, &line3_data);

        device.submit(encoder.as_buffer());
        window.swap_buffers().unwrap();
        device.cleanup();
    }

    println!("Goodbye");
}
