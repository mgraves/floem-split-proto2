mod divider;
mod split_stack;
mod split_dragger;

use floem::{
    event::EventListener,
    peniko::Color,
    reactive::{create_signal, create_rw_signal, RwSignal},
    style::Position,
    view::View,
    views::{
        container, empty, h_stack, label, scroll, v_stack, virtual_list, Decorators, VirtualListDirection,
        VirtualListItemSize,
    },
    context::{EventPropagation},
};
use floem::event::Event;
use floem::id::Id;
use floem::kurbo::Point;
use floem::reactive::WriteSignal;
use crate::divider::divider_with_id;
use crate::split_stack::h_split_stack;

const LEFT_PANE_WIDTH: f64 = 140.0;
const CENTER_PANE_WIDTH: f64 = 300.0;
const RIGHT_PANE_WIDTH: f64 = 300.0;

pub fn colored_pane(title: &'static str, color: Color, width_signal: RwSignal<f64>) -> impl View {
    let result = container(
        label(move || title.to_string())
          .style(|s| s.width(80)
            .padding(10.0)
          ),
    ).style(move |s| s.flex_col().items_center()
      .items_start()
      .padding_bottom(10.0)
      .width(width_signal.get() - 1.0)
      .height_full()
      .background(color)
    );
    result
}

pub fn app_view() -> impl View {
    let left_width = create_rw_signal(LEFT_PANE_WIDTH);
    let center_width = create_rw_signal(CENTER_PANE_WIDTH);
    let right_width = create_rw_signal(RIGHT_PANE_WIDTH);

    let left_pane = colored_pane("Left",     Color::rgba(0.8, 0.0, 0.0, 0.5), left_width);
    let center_pane = colored_pane("Center", Color::rgba(0.0, 0.8, 0.0, 0.5), center_width);
    let right_pane = colored_pane("Right",   Color::rgba(0.0, 0.0, 0.8, 0.5), right_width);


    let split = h_split_stack((left_pane, center_pane, right_pane),
                      vec![left_width, center_width, right_width])
      .style(|s| s.padding(10.0));

    // split.id().inspect();
    println!("split id: {:?}", split.id());
    split
}




fn main() {
    floem::launch(app_view);
}