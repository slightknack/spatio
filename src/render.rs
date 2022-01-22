use miniquad::*;
use crate::quad::{Node, Embed};

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

struct Stage<A: Embed, B: Embed> {
    pipeline: Pipeline,
    bindings: Bindings,
    texture:  Texture,
    node:     Node<A, B>,
}

const SIZE: usize = 4;

impl<A: Embed, B: Embed> Stage<A, B> {
    pub fn new(ctx: &mut Context) -> Self {
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1., y: -1. }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  1., y: -1. }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  1., y:  1. }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -1., y:  1. }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let pixels: [u8; SIZE*SIZE*4] = [0x77; SIZE*SIZE*4];
        let texture = Texture::from_rgba8(ctx, SIZE as u16, SIZE as u16, &pixels);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::SHADER_META);

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        Stage { pipeline, bindings, texture, node: todo!() }
    }
}

impl<A: Embed, B: Embed> EventHandler for Stage<A, B> {
    fn update(&mut self, ctx: &mut Context) {
        let t = date::now();
        let bright = t.sin()*0.5+0.5;
        let bytes = [(bright * 255.0) as u8; SIZE*SIZE*4];

        self.texture.update(ctx, &bytes)
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        // ctx.apply_uniforms(&shader::Uniforms {});

        ctx.draw(0, 6, 1);
        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

pub fn graphics() {
    // assert_eq!((2 as usize).pow(node.depth as u32), SIZE);
    // miniquad::start(conf::Conf::default(), move |mut ctx| {
    //     UserData::owning(Stage::new(&mut ctx), ctx)
    // });
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(pos, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub const SHADER_META: ShaderMeta = ShaderMeta {
        images: &["tex"],
        uniforms: UniformBlockLayout {
            uniforms: &[], // &[("offset", UniformType::Float2)],
        },
    };

    // #[repr(C)]
    // pub struct Uniforms {
    //     // pub offset: (f32, f32),
    // }
}
