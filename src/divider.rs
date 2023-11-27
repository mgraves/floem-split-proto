use floem::{
  id::Id,
  view::{View, ViewData},
};

pub struct Divider {
  data: ViewData,
}

pub fn divider() -> Divider {
  Divider {
    data: ViewData::new(Id::next()),
  }
}
pub fn divider_with_id(id: Id) -> Divider {
  Divider {
    data: ViewData::new(id),
  }
}

impl View for Divider {
  fn view_data(&self) -> &ViewData {
    &self.data
  }

  fn view_data_mut(&mut self) -> &mut ViewData {
    &mut self.data
  }
}
