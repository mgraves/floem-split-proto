mod divider;

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
use floem::style::{AlignContent, AlignItems};
use floem::widgets::checkbox;
use crate::divider::divider_with_id;

pub fn colored_pane(title: &'static str, color: Color, width_signal: RwSignal<f64>) -> impl View {
  let result = container(
    label(move || title.to_string())
      .style(|s| s.width(200)
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

fn resizer(width_signal: RwSignal<f64>) -> impl View {
  let resizer_color = create_rw_signal(Color::BLACK);
  let is_resizing = create_rw_signal(false);
  let mouse_pos = create_rw_signal((0.0, 0.0));

  let resizer_id = Id::next();
  let resizer = divider_with_id(resizer_id)
    .style(move |s| s.width(6.0)
      .position(Position::Absolute)
      .inset_left(width_signal.get())
      .width(2.)
      .height_full()
      .background(resizer_color.get())
      .hover(|s| {
        s.width(4.0)
          .background(Color::BLUE)
          .border_radius(0.)
          .border(2.)
          .border_color(Color::BLUE)
      })
    ).on_event(EventListener::PointerDown, move |event| {
    resizer_id.request_active();
    if let Event::PointerDown(pointer_event) = event {
      resizer_color.set(Color::BLUE);
      is_resizing.set(true);
      mouse_pos.set((pointer_event.pos.x, pointer_event.pos.y));
    }
    EventPropagation::Continue
  })
    .on_event(EventListener::PointerMove, move |event| {
      if let Event::PointerMove(pointer_event) = event {
        if is_resizing.get() {
          let delta = pointer_event.pos.x - mouse_pos.get().0;
          if delta.abs() > 3.0 {
            // mouse_pos.set((pointer_event.pos.x, pointer_event.pos.y));
            mouse_pos.set(((0.0), (0.0)));
            width_signal.update(move |width| { *width += delta });
          }
        }
      }
      EventPropagation::Continue
    })
    .on_event(EventListener::PointerUp, move |event| {
      is_resizing.set(false);
      resizer_color.set(Color::BLACK);
      EventPropagation::Continue
    });
  resizer
}

pub fn app_view() -> impl View {
  let left_width = create_rw_signal(150.0);
  let right_width = create_rw_signal(250.0);
  let left_pane = colored_pane("Left", Color::rgba(0.8, 0., 0.0, 0.3), left_width);
  let right_pane = colored_pane("Right", Color::rgba(0., 0., 0.8, 0.3), right_width);
  let resizer0 = resizer(left_width);
  let (is_checked, set_is_checked) = create_signal(false);
  let split_padding = create_rw_signal(0.0);

  let h_split = h_stack((left_pane,
           resizer0,
           right_pane))
    .style(move |s| {
      s.flex_row()
        .items_start()
        .width_full()
        .height_full()
        .padding(split_padding.clone().get())
        .gap(0.0, 10.0)
        .background(Color::rgba(0., 0.0, 0.0, 0.25))
    });
  let top_bar = h_stack((
    label(move || String::from("Padding:"))
    .style(|s| s//.width(100.0)
      .padding(8.)
      .height(30.0)
           ),
    checkbox(is_checked)
    .style(|s| s.margin(5.0))
    .on_click_stop(move |_| {
      set_is_checked.update(|checked| *checked = !*checked);
      if is_checked.get() {
        split_padding.set(10.0);
      } else {
        split_padding.set(0.0);
      }
      println!("padding: {}", split_padding.get());
    })
  ))
    // .style(|s| s.flex_row()
    //   .align_items(Some(AlignItems::Center))
    // )
    ;
  v_stack((top_bar, h_split))
}

fn main() {
  floem::launch(app_view);
}
