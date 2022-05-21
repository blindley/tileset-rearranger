
#[derive(Debug)]
pub struct RectangleRenderer {
    program: u32,
    vertex_array: u32,
}

impl RectangleRenderer {
    pub fn new() -> RectangleRenderer {
        let program = create_program();
        let vertex_array = create_vertex_array();

        RectangleRenderer {
            program,
            vertex_array: vertex_array.vertex_array,
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::UseProgram(self.program);

            gl::BindVertexArray(self.vertex_array);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    pub fn on_window_resize(&self, window_size: [i32;2]) {
        let infobox_height = 32.0;

        unsafe {
            gl::UseProgram(self.program);
        }

        let infobox_height = 2.0 * infobox_height / (window_size[1] as f32);
        unsafe {
            gl::UseProgram(self.program);
            let location = gl::GetUniformLocation(self.program,
                "infobox_height\0".as_ptr() as _);
            gl::Uniform1f(location, infobox_height);
        }
    }
}

struct BufferData {
    #[allow(dead_code)]
    buffer: u32,
    vertex_array: u32,
}

fn create_vertex_array() -> BufferData {
    unsafe {
        let (mut buffer, mut vertex_array) = (0, 0);

        let offset: f32 = 0.5;

        let vertices = [
            // position  // tex coords
            -offset,  offset,  0.0,    0.0,        // top left 
             offset,  offset,  offset, 0.0,        // top right
            -offset, -offset,  0.0,    offset,     // bottom left
            -offset, -offset,  0.0,    offset,     // bottom left
             offset,  offset,  offset, 0.0,        // top right
             offset, -offset,  offset, offset,     // bottom right
        ];

        let _: f32 = vertices[0]; // dumb hack to force vertices to be array of f32

        gl::GenVertexArrays(1, &mut vertex_array);
        gl::GenBuffers(1, &mut buffer);

        gl::BindVertexArray(vertex_array);

        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        let size = std::mem::size_of_val(&vertices) as _;
        let ptr = vertices.as_ptr() as _;
        gl::BufferData(gl::ARRAY_BUFFER, size, ptr, gl::STATIC_DRAW);

        let stride = (4 * std::mem::size_of::<f32>()) as _;

        let ptr = 0 as _;
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr);
        gl::EnableVertexAttribArray(0);

        let ptr = (2 * std::mem::size_of::<f32>()) as _;
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, ptr);
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);

        BufferData {
            buffer,
            vertex_array,
        }
    }
}

fn create_program() -> u32 {
    unsafe {
        let vshader = compile_shader(shader_code::VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
        let fshader = compile_shader(shader_code::FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vshader);
        gl::AttachShader(program, fshader);
        gl::LinkProgram(program);
        
        gl::DeleteShader(vshader);
        gl::DeleteShader(fshader);

        gl::UseProgram(program);

        program
    }
}

fn compile_shader(code: &str, type_: gl::types::GLenum) -> u32 {
    unsafe {
        let code_ptr = code.as_ptr() as _;
        let shader = gl::CreateShader(type_);
        gl::ShaderSource(shader, 1, &code_ptr, 0 as _);
        gl::CompileShader(shader);

        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut info_log_buffer = [0i8;512];
            let ptr = info_log_buffer.as_mut_ptr();
            gl::GetShaderInfoLog(shader, 512, 0 as _, ptr);

            let cstr = std::ffi::CStr::from_ptr(ptr);
            let str_slice = cstr.to_str().unwrap();
            panic!("{}", str_slice);
        }

        shader
    }
}

mod shader_code {
    pub const VERTEX_SHADER_SOURCE: &str =
        "\
        #version 330 core\n\
        layout (location = 0) in vec2 pos;\n\
        layout (location = 1) in vec2 tcoords;\n\
        \
        out vec2 vtcoords;\n\
        uniform float infobox_height;\n\
        \
        void main() {\n\
            float hscale = 1.0f;\n\
            float vscale = 1.0f;\n\
            vscale = vscale * (2.0f - infobox_height) / 2.0f;\n\
            float x = pos.x * hscale;\n\
            float y = pos.y * vscale + infobox_height / 2.0f;\n\
            gl_Position = vec4(x, y, 0.0, 1.0);\n\
            vtcoords = tcoords;
        }\n\
        \0";

    pub const FRAGMENT_SHADER_SOURCE: &str =
        "\
        #version 330 core\n\
        in vec2 vtcoords;\n\
        out vec4 fcolor;\n\
        \
        void main() {\n\
            fcolor = vec4(1.0, 1.0, 0.5, 1.0);
        }\n\
        \0";
}

