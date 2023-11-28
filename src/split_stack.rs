
use floem::{context::UpdateCx, EventPropagation, id::Id, style::Style, view::{View, ViewData}, view_tuple::ViewTuple};
use floem::event::{Event, EventListener};
use floem::reactive::{create_rw_signal, RwSignal};
use floem::style::{FlexDirection, Position};
use floem::views::Decorators;
use crate::split_dragger::split_dragger;

/// SplitStack is a Stack with draggable dividers between its (regular) children.
pub struct SplitStack {
  data: ViewData,
  children: Vec<Box<dyn View>>,
  direction: Option<FlexDirection>,
}

pub fn h_split_stack<VT: ViewTuple + 'static>(children: VT, width_signals: Vec<RwSignal<f64>>) -> SplitStack {
  let view0_width = create_rw_signal(150.0);
  let is_dragging = create_rw_signal(false);
  let mouse_pos = create_rw_signal((0.0, 0.0));

  // insert dividers between children
  let num_views = children.children().len();
  // first convert the view tuple into views
  let views: Vec<Box<dyn View>> = children.into_views();
  let mut joined: Vec<Box<dyn View>> = Vec::with_capacity(num_views * 2 - 1);


  for (index, view) in views.into_iter().enumerate() {
    joined.push(view);

    // insert a divider after each element except the last one.
    if index < num_views - 1 {
      // view.style( move |s| s.width(view_width.get()));
      // let width_signal = view.width_signal();
      let width_signal = match width_signals.get(index) {
        None => {
          println!("[ERROR] SplitStack::h_split_stack: no width_signal for index: {:?}",index);
          // dummy signal that won't do anything when resized, prevents panic if `width_signals`
          // doesn't have enough elements.
          create_rw_signal(150.0f64)
        }
        Some(signal) => signal.clone(),
      };
      let dragger = split_dragger(width_signal, is_dragging.clone());
      joined.push(Box::new(dragger));
    }
  }

  let mut result =  SplitStack {
    data: ViewData::new(Id::next()),
    children: joined,
    direction: Some(FlexDirection::Row),
  };

  result
    .style(|s| {
      s.inset_top(0.0)
        .inset_bottom(0.0)
        .width_full()
        .height_full()
    })
}

// TODO: add support for vertical split stacks

impl View for SplitStack {
  fn view_data(&self) -> &ViewData {
    &self.data
  }

  fn view_data_mut(&mut self) -> &mut ViewData {
    &mut self.data
  }

  fn view_style(&self) -> Option<Style> {
    self.direction
      .map(|direction| Style::new().flex_direction(direction))
  }

  fn for_each_child<'a>(&'a self, for_each: &mut dyn FnMut(&'a dyn View) -> bool) {
    for child in &self.children {
      if for_each(child) {
        break;
      }
    }
  }

  fn for_each_child_mut<'a>(&'a mut self, for_each: &mut dyn FnMut(&'a mut dyn View) -> bool) {
    for child in &mut self.children {
      if for_each(child) {
        break;
      }
    }
  }

  fn for_each_child_rev_mut<'a>(
    &'a mut self,
    for_each: &mut dyn FnMut(&'a mut dyn View) -> bool,
  ) {
    for child in self.children.iter_mut().rev() {
      if for_each(child) {
        break;
      }
    }
  }

  fn debug_name(&self) -> std::borrow::Cow<'static, str> {
    match self.direction {
      Some(FlexDirection::Column) => "Vertical Stack".into(),
      Some(FlexDirection::Row) => "Horizontal Stack".into(),
      _ => "Stack".into(),
    }
  }

  fn update(&mut self, cx: &mut UpdateCx, state: Box<dyn std::any::Any>) {
    if let Ok(state) = state.downcast() {
      self.children = *state;
      cx.request_all(self.id());
    }
  }
}
