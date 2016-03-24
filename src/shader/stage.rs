use core::marker::PhantomData;

pub trait HasStage {
  type AStage;

  fn new_shader(shader_type: Type, src: &str) -> Result<Self::AStage, StageError>;
}

pub trait ShaderTypeable {
  fn shader_type() -> Type;
}

/// A shader stage type.
pub enum Type {
  TessellationControlShader,
  TessellationEvaluationShader,
  VertexShader,
  GeometryShader,
  FragmentShader
}

pub struct TessellationControlShader;

impl ShaderTypeable for TessellationControlShader {
  fn shader_type() -> Type { Type::TessellationControlShader }
}

pub struct TessellationEvaluationShader;

impl ShaderTypeable for TessellationEvaluationShader {
  fn shader_type() -> Type { Type::TessellationEvaluationShader }
}

pub struct VertexShader;

impl ShaderTypeable for VertexShader {
  fn shader_type() -> Type { Type::VertexShader }
}

pub struct GeometryShader;

impl ShaderTypeable for GeometryShader {
  fn shader_type() -> Type { Type::GeometryShader }
}

pub struct FragmentShader;

impl ShaderTypeable for FragmentShader {
  fn shader_type() -> Type { Type::FragmentShader }
}

/// A shader stage. The `T` type variable gives the type of the shader.
pub struct Stage<C, T> where C: HasStage {
  pub repr: C::AStage,
  _t: PhantomData<T>
}

impl<C, T> Stage<C, T> where C: HasStage, T: ShaderTypeable {
  pub fn new(src: &str) -> Result<Self, StageError> {
    let shader = C::new_shader(T::shader_type(), src);
    shader.map(|shader| Stage {
      repr: shader,
      _t: PhantomData
    })
  }
}

pub enum StageError {
  /// Occurs when a shader fails to compile.
  CompilationFailed(String),
  /// Occurs when you try to create a shader which type is not supported on the current hardware.
  UnsupportedType(Type)
}
