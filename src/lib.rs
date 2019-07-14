#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;

use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

mod utils;
mod engine;

use engine::Engine;

lazy_static! {
    static ref ENGINE: Mutex<Engine> = Mutex::new(Engine::init(0.0, 0.0, 0.0));
}

#[wasm_bindgen]
pub fn init(
    width: u32,
    height: u32,
) {
    {
        let engine = &mut ENGINE.lock().unwrap();
    }
}

#[wasm_bindgen]
pub fn key_down(key: u32) {
    let engine = &mut ENGINE.lock().unwrap();
    engine.set_key(key);
}

#[wasm_bindgen]
pub fn key_up(key: u32) {
    let engine = &mut ENGINE.lock().unwrap();
    engine.unset_key(key);
}

#[wasm_bindgen]
pub fn tick(
    dt: f32,
    context: &WebGlRenderingContext,
) {
    update(dt);
    draw(context);
}

fn update(dt: f32) {
    let engine = &mut ENGINE.lock().unwrap();
    engine.update(dt);
    //utils::log_f32(dt);
}

fn draw(
    context: &WebGlRenderingContext,
) -> Result<(), JsValue> {
    let engine = &mut ENGINE.lock().unwrap();

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
        "#,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
        "#,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let mut vertices: [f32; 12] = [
        engine.x-1.0, engine.y-1.0, engine.z,
        engine.x,     engine.y-1.0, engine.z,
        engine.x,     engine.y,     engine.z,
        engine.x-1.0, engine.y,     engine.z
        ];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(1.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGlRenderingContext::TRIANGLE_FAN,
        0,
        (vertices.len() / 3) as i32,
    );
    Ok(())
}

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}