use bytemuck::cast_slice;
use engine::mesh::Mesh;
use glow::HasContext;

pub struct GpuMesh {
    vao: glow::NativeVertexArray,
    position_vbo: glow::NativeBuffer,
    normal_vbo: glow::NativeBuffer,
    uv_vbo: glow::NativeBuffer,
    ebo: glow::NativeBuffer,
    index_count: i32,
}

impl GpuMesh {
    /// Загружает чистый `Mesh` в GPU.
    ///
    /// Attribute 0: vec3 position
    /// Attribute 1: vec3 normal
    /// Attribute 2: vec2 uv
    pub fn upload(gl: &glow::Context, mesh: &Mesh) -> Result<Self, String> {
        let vertex_count = mesh.vertices.len();
        let index_count =
            i32::try_from(mesh.indices.len()).map_err(|_| "Too many indices".to_string())?;

        if mesh.indices.iter().any(|&i| i as usize >= vertex_count) {
            return Err("Mesh contains invalid indices".into());
        }

        let positions: Vec<[f32; 3]> =
            mesh.vertices.iter().map(|v| v.position.to_array()).collect();
        let normals: Vec<[f32; 3]> =
            mesh.vertices.iter().map(|v| v.normal.to_array()).collect();
        let uvs: Vec<[f32; 2]> = mesh.vertices.iter().map(|v| v.uv.to_array()).collect();

        unsafe {
            let vao = gl.create_vertex_array()?;
            let position_vbo = gl.create_buffer()?;
            let normal_vbo = gl.create_buffer()?;
            let uv_vbo = gl.create_buffer()?;
            let ebo = gl.create_buffer()?;

            gl.bind_vertex_array(Some(vao));

            // Position VBO.
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(position_vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&positions), glow::STATIC_DRAW);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);

            // Normal VBO.
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(normal_vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&normals), glow::STATIC_DRAW);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 0, 0);

            // UV VBO.
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(uv_vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&uvs), glow::STATIC_DRAW);
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 0, 0);

            // Element Buffer Object.
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                cast_slice(&mesh.indices),
                glow::STATIC_DRAW,
            );

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            Ok(Self {
                vao,
                position_vbo,
                normal_vbo,
                uv_vbo,
                ebo,
                index_count,
            })
        }
    }

    /// Отрисовка меша.
    pub fn draw(&self, gl: &glow::Context) {
        if self.index_count == 0 {
            return;
        }

        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements(glow::TRIANGLES, self.index_count, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }

    /// Освобождение GPU-ресурсов.
    pub fn dispose(self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.position_vbo);
            gl.delete_buffer(self.normal_vbo);
            gl.delete_buffer(self.uv_vbo);
            gl.delete_buffer(self.ebo);
        }
    }
}
