use floem::event::{Event, EventListener};
use floem::EventPropagation;
use floem::id::Id;
use floem::peniko::Color;
use floem::reactive::{create_rw_signal, RwSignal};
use floem::style::{CursorStyle, Position};
use floem::view::View;
use floem::views::{Decorators, empty};
use crate::divider::{divider, divider_with_id};

const DIVIDER_COLOR: Color = Color {
  r: 10,
  g: 10,
  b: 10,
  a: 200,
};

const DRAG_DIVIDER_COLOR: Color = Color::BLUE;
pub fn split_dragger(width_signal: RwSignal<f64>, is_resizing: RwSignal<bool>) -> impl View {
  let mouse_pos = create_rw_signal((0.0, 0.0));
  let background_color = create_rw_signal(DIVIDER_COLOR);
  let divider_id = Id::next();
  divider_with_id(divider_id)
    .style(move |s| {
      s//.position(Position::Absolute)
        .z_index(11)
        .inset_top(0)
        .inset_bottom(0)
        // .inset_left(left_view_width.get())
        .width(5)
        .background(background_color.get())
        .border_left(2)
        .border_color(background_color.get())
        .cursor(CursorStyle::ColResize)
        .hover(|s| {
          s.border_color(DRAG_DIVIDER_COLOR)
            .cursor(CursorStyle::ColResize)
        })
    })
    .on_event(EventListener::PointerDown, move |event| {
      if let Event::PointerDown(pointer_event) = event {
        divider_id.request_active();
        is_resizing.set(true);
        background_color.set(DRAG_DIVIDER_COLOR);
        mouse_pos.set((pointer_event.pos.x, pointer_event.pos.y));
        // println!("SplitDragger::on_event: PointerDown is_resizing: {:?}, x: {:.2?}, width: {:.2?}",
        //          is_resizing.get(), pointer_event.pos.x, width_signal.get());
      }
      EventPropagation::Continue
    })
    .on_event(EventListener::PointerMove, move |event| {
      if let Event::PointerMove(pointer_event) = event {
        if is_resizing.get() {
          let delta = pointer_event.pos.x - mouse_pos.get().0;
          if delta.abs() > 3.0 {
            let old_width = width_signal.get();
            width_signal.update(move |width| { *width += delta });
            let new_width = width_signal.get();

            // println!("SplitDragger::on_event: PointerMove is_resizing: {:?}, delta: {:.2?}, x: {:.2?}, old  width: {:.2?}, new_width: {:.2?}",
            //          is_resizing.get(), delta, pointer_event.pos.x, old_width, new_width);
            mouse_pos.set((0.0, 0.0));
          }
        }
      }
      EventPropagation::Continue
    })
    // .on_event(EventListener::DragEnd, move |_| {
    .on_event(EventListener::PointerUp, move |_| {
      is_resizing.set(false);
      background_color.set(DIVIDER_COLOR);
      // println!("SplitDragger::on_event: PointerUp is_resizing: {:?}", is_resizing.get());
      EventPropagation::Continue
    })
    // .on_event(EventListener::DoubleClick, move |_| {
    //   // left_view_width.set(100.0);
    //   EventPropagation::Continue
    // })
}