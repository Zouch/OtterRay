extern crate gl;
extern crate glutin;
extern crate rand;

use std::sync::mpsc::channel;
use std::time::Instant;
use std::os::raw::c_void;
use std::ffi::CString;

use rand::distributions::{IndependentSample, Range};

use glutin::GlContext;

mod math;
use math::*;

mod utils;
use utils::*;

mod raytracer;
use raytracer::*;

use gl::types::*;

fn create_vertex_buffer(data: Vec<f32>) -> (GLuint, GLuint) {
    let mut vbo = 0u32;
    let mut vao = 0u32;

    let size = (data.len() * std::mem::size_of::<f32>()) as isize;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, size, vec_void_ptr(&data), gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, (data.len() / 4) as i32, gl::FLOAT, gl::FALSE, 0, std::ptr::null() as _);
    }

    return (vao, vbo);
}

struct Shader {
    shader_type: GLenum,
    shader_handle: GLuint,
}

impl Shader {
    fn new(shader_type: GLenum) -> Shader {
        unsafe {
            Shader {
                shader_type: shader_type,
                shader_handle: gl::CreateShader(shader_type),
            }
        }
    }
}

fn compile_shader(source: String, shader_type: GLenum) -> Shader {

    let mut shader = Shader::new(shader_type);

    unsafe {
        shader.shader_handle = gl::CreateShader(shader.shader_type);

        let c_source = CString::new(source).unwrap();

        gl::ShaderSource(shader.shader_handle, 1, &c_source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader.shader_handle);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader.shader_handle, gl::COMPILE_STATUS, &mut status);

        if status != gl::TRUE as GLint {
            shader.shader_handle = 0;
            let info_log = String::with_capacity(256);
            let mut error_size = 0i32;
            gl::GetShaderInfoLog(shader.shader_handle, 256, &mut error_size, info_log.as_ptr() as _);
            println!("Could not compile shader: {:?}", info_log);
        }
    }

    return shader;
}

fn link_program(shaders: Vec<Shader>) -> GLuint {

    let mut program: u32;
    let mut ok = true;
    for shader in shaders.iter() {
        if shader.shader_handle == 0 {
            ok = false;
        }
    }

    assert_eq!(ok, true);

    unsafe {
        program = gl::CreateProgram();
        for shader in shaders.iter() {
            gl::AttachShader(program, shader.shader_handle);
        }

        gl::LinkProgram(program);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        if status != gl::TRUE as GLint {
            program = 0;
            let info_log = String::with_capacity(256);
            let mut error_size = 0i32;
            gl::GetProgramInfoLog(program, 256, &mut error_size, info_log.as_ptr() as _);
            println!("Could not link program: {:?}", info_log);
        }
    }

    return program;
}

fn main()
{
    let mut image = Image::new(1024, 768, 8);
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, World")
        .with_dimensions(image.width, image.height);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);

    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let data = vec![
        -1.0, -1.0, 0.0,
        -1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, -1.0, 0.0,
    ];


    let (vao, vbo) = create_vertex_buffer(data);
    assert!(vao != 0);
    assert!(vbo != 0);

    let vsource = String::from("
        #version 450
        layout (location = 0) in vec3 in_pos;
        layout (location = 0) out vec2 out_texcoord;
        void main() {
            gl_Position = vec4(in_pos, 1.0);
            out_texcoord = in_pos.xy * 0.5 + 0.5;
            out_texcoord.y = 1 - out_texcoord.y;
        }");

    let fsource = String::from("
        #version 450
        uniform sampler2D u_texture;
        layout (location = 0) in vec2 in_texcoord;
        layout (location = 0) out vec4 out_color;
        void main() {
            vec3 color = texture(u_texture, in_texcoord).rgb;
            out_color = vec4(color, 1);
        }");

    let vshader = compile_shader(vsource, gl::VERTEX_SHADER);
    let fshader = compile_shader(fsource, gl::FRAGMENT_SHADER);

    assert!(vshader.shader_handle != 0);
    assert!(fshader.shader_handle != 0);

    let program = link_program(vec![vshader, fshader]);
    assert!(program != 0);

    let mut texture = 1u32;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as _);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as _, image.width as _, image.height as _,
                       0, gl::RGB, gl::FLOAT, std::ptr::null() as _);
    }

    let mut objects = Vec::new();

    let white_material = Material::new(Color::WHITE);
    let red_material = Material::new(Color::RED);
    let green_material = Material::new(Color::GREEN);
    let blue_material = Material::new(Color::BLUE);
    let yellow_material = Material::new(Color::YELLOW);

    objects.push(make_plane(Vector3::new(-1.0, 0.0, 0.0), 2.5, green_material)); // Right
    objects.push(make_plane(Vector3::new(1.0, 0.0, 0.0), 2.5, red_material));    // Left
    objects.push(make_plane(Vector3::new(0.0, 0.0, 1.0), 2.5, white_material));  // Bottom
    objects.push(make_plane(Vector3::new(0.0, 0.0, -1.0), 2.5, white_material)); // Top
    objects.push(make_plane(Vector3::new(0.0, -1.0, 0.0), 5.0, white_material)); // Back

    objects.push(make_sphere(Vector3::new(-1.0, 2.0, -1.0), 0.75, blue_material));
    objects.push(make_sphere(Vector3::new(1.0, 2.0, -1.0), 0.75, yellow_material));

    let mut camera = Camera::new(image.width, image.height, 1.0);
    camera.look_at(Vector3::new(0.0, -5.0, 0.0), Vector3::new(0.0, 0.0, 0.0));

    let world = World::new(objects, camera);

    let now = Instant::now();
    let (tx, rx) = channel();
    world.raytrace(&image, tx);

    let mut running = true;
    let mut done_rendering = false;
    while running || !done_rendering {
        let dt = Instant::now();
        while dt.elapsed().subsec_nanos() < 17000000 {
            match rx.recv() {
                Ok((x, y, color)) => {
                    image.set_pixel_color(x, y, color);
                },
                Err(_) => {
                    done_rendering = true;
                }
            }
        }

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::UseProgram(program);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as _, image.width as _, image.height as _,
                           0, gl::RGB, gl::FLOAT, image.data());

            gl::ActiveTexture(gl::TEXTURE0);
            gl::DrawArrays(gl::QUADS, 0, 4);
        }

        gl_window.swap_buffers().unwrap();
    }

    let t1 = now.elapsed();
    image.write_png("test.png".to_string());
    let t2 = now.elapsed() - t1;

    let t1_s = t1.as_secs() as f32 + (t1.subsec_nanos() as f32) * 1e-9;
    let t2_s = t2.as_secs() as f32 + (t2.subsec_nanos() as f32) * 1e-9;

    println!("Raytracing took {}s, writing image took {}s", t1_s, t2_s);
}
