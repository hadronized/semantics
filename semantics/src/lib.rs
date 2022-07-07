#[cfg(feature = "semantics-derive")]
pub use semantics_derive::Semantics;

pub trait Semantics {
  type Name;
  type Index: Copy;

  fn name(&self) -> Self::Name;

  fn index(&self) -> Self::Index;
}

pub trait HasSemantics<Name = &'static str, Index = u8> {
  type Sem: Semantics<Name = Name, Index = Index>;

  const SEMANTICS: Self::Sem;
}

#[cfg(test)]
mod test {
  use super::*;

  #[derive(Debug, Eq, PartialEq)]
  enum VertexSemantics {
    Position,
    Normal,
    Color,
  }

  impl Semantics for VertexSemantics {
    type Name = &'static str;

    type Index = u8;

    fn name(&self) -> Self::Name {
      match *self {
        VertexSemantics::Position => "position",
        VertexSemantics::Normal => "normal",
        VertexSemantics::Color => "color",
      }
    }

    fn index(&self) -> Self::Index {
      match *self {
        VertexSemantics::Position => 0,
        VertexSemantics::Normal => 1,
        VertexSemantics::Color => 2,
      }
    }
  }

  struct VPos(pub [f32; 3]);

  impl HasSemantics for VPos {
    type Sem = VertexSemantics;

    const SEMANTICS: Self::Sem = VertexSemantics::Position;
  }

  struct VNor(pub [f32; 3]);

  impl HasSemantics for VNor {
    type Sem = VertexSemantics;

    const SEMANTICS: Self::Sem = VertexSemantics::Normal;
  }

  struct VCol(pub [f32; 3]);

  impl HasSemantics for VCol {
    type Sem = VertexSemantics;

    const SEMANTICS: Self::Sem = VertexSemantics::Color;
  }

  struct Vertex {
    pos: VPos,
    nor: VNor,
    col: VCol,
  }

  #[test]
  fn test_vertex_semantics() {
    assert_eq!(VertexSemantics::Position.name(), "position");
    assert_eq!(VertexSemantics::Normal.name(), "normal");
    assert_eq!(VertexSemantics::Color.name(), "color");

    assert_eq!(VertexSemantics::Position.index(), 0);
    assert_eq!(VertexSemantics::Normal.index(), 1);
    assert_eq!(VertexSemantics::Color.index(), 2);

    assert_eq!(VPos::SEMANTICS, VertexSemantics::Position);
  }
}
