use semantics::Semantics;

#[derive(Debug, Semantics)]
pub enum VertexPosition {
  Position,
  Normal,
  Color,
}

#[test]
fn name() {
  assert_eq!(VertexPosition::Position.name(), "position");
  assert_eq!(VertexPosition::Normal.name(), "normal");
  assert_eq!(VertexPosition::Color.name(), "color");
}

#[test]
fn index() {
  assert_eq!(VertexPosition::Position.index(), 0);
  assert_eq!(VertexPosition::Normal.index(), 1);
  assert_eq!(VertexPosition::Color.index(), 2);
}

#[derive(Debug, Semantics)]
pub enum ExplicitName {
  #[sem(name = "foo")]
  JustFoo,

  #[sem(name = "bar")]
  JustBar,

  Nothing,
}

#[test]
fn explicit_names() {
  assert_eq!(ExplicitName::JustFoo.name(), "foo");
  assert_eq!(ExplicitName::JustBar.name(), "bar");
  assert_eq!(ExplicitName::Nothing.name(), "nothing");
}

#[derive(Debug, Semantics)]
pub enum ExplicitIndex {
  #[sem(index = 14)]
  Just14,

  #[sem(index = 7)]
  Just7,

  #[sem(index = 0)]
  Just0,
}

#[test]
fn explicit_indices() {
  assert_eq!(ExplicitIndex::Just14.index(), 14);
  assert_eq!(ExplicitIndex::Just7.index(), 7);
  assert_eq!(ExplicitIndex::Just0.index(), 0);
}

#[derive(Debug, Semantics)]
pub enum ExplicitNameAndIndex {
  #[sem(name = "foo", index = 14)]
  JustFoo14,

  #[sem(name = "bar", index = 7)]
  JustBar7,

  #[sem(index = 0)]
  Nothing0,
}

#[test]
fn explicit_name_and_index() {
  assert_eq!(ExplicitNameAndIndex::JustFoo14.name(), "foo");
  assert_eq!(ExplicitNameAndIndex::JustBar7.name(), "bar");
  assert_eq!(ExplicitNameAndIndex::Nothing0.name(), "nothing_0");

  assert_eq!(ExplicitNameAndIndex::JustFoo14.index(), 14);
  assert_eq!(ExplicitNameAndIndex::JustBar7.index(), 7);
  assert_eq!(ExplicitNameAndIndex::Nothing0.index(), 0);
}
