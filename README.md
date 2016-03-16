## Testing gfx-rs but not getting opacity/alpha value to work

Rust-nightly.

Trying to set alpha to `0.5` in the fragment shader.
```
cargo run
```
Everything 100% opaque.

## UPDATE: 

Change to following for fixage:

```
fn my_mask() -> gfx::state::ColorMask {
    gfx::state::ColorMask::all()
}

fn my_blender() -> gfx::state::Blend {
    use gfx::state::{Equation,Factor,BlendValue};
    gfx::state::Blend::new(
        Equation::Add,
        Factor::ZeroPlus(BlendValue::SourceAlpha),
        Factor::OneMinus(BlendValue::SourceAlpha),
    )
}

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
    normal: [f32; 2] = "a_Normal",
});

gfx_pipeline!(line {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    width: gfx::Global<f32> = "u_Width",
    color: gfx::Global<[f32; 3]> = "u_Color",
    out: gfx::BlendTarget<gfx::format::Rgba8> = ("o_Color", ::my_mask(), ::my_blender()),
});

```
