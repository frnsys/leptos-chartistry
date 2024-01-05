use super::{InnerLayout, UseInner};
use crate::{
    colours::{Colour, LIGHTER_GREY},
    debug::DebugRect,
    projection::Projection,
    state::State,
    ticks::GeneratedTicks,
    TickLabels,
};
use leptos::*;
use std::{borrow::Borrow, rc::Rc};

#[derive(Clone)]
pub struct GridLine<Tick: Clone> {
    width: MaybeSignal<f64>,
    colour: MaybeSignal<Colour>,
    ticks: TickLabels<Tick>,
}

#[derive(Clone)]
struct UseGridLine<Tick: 'static> {
    width: MaybeSignal<f64>,
    colour: MaybeSignal<Colour>,
    ticks: Signal<GeneratedTicks<Tick>>,
}

#[derive(Clone)]
struct UseHorizontalGridLine<X: 'static>(UseGridLine<X>);
#[derive(Clone)]
struct UseVerticalGridLine<Y: 'static>(UseGridLine<Y>);

impl<Tick: Clone> GridLine<Tick> {
    fn layout(ticks: impl Borrow<TickLabels<Tick>>) -> Self {
        Self {
            width: 1.0.into(),
            colour: Into::<Colour>::into(LIGHTER_GREY).into(),
            ticks: ticks.borrow().clone(),
        }
    }
}

impl<X: Clone> GridLine<X> {
    /// Vertical grid lines running parallel to the y-axis. These run from top to bottom at each tick.
    pub fn vertical<Y: Clone>(ticks: impl Borrow<TickLabels<X>>) -> InnerLayout<X, Y> {
        InnerLayout::HorizontalGridLine(Self::layout(ticks))
    }
}

impl<Y: Clone> GridLine<Y> {
    /// Horizontal grid lines running parallel to the x-axis. These run from left to right at each tick.
    pub fn horizontal<X: Clone>(ticks: impl Borrow<TickLabels<Y>>) -> InnerLayout<X, Y> {
        InnerLayout::VerticalGridLine(Self::layout(ticks))
    }
}

impl<X: Clone + PartialEq> GridLine<X> {
    pub(crate) fn use_horizontal<Y>(self, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        let inner = state.layout.inner;
        let avail_width = Signal::derive(move || with!(|inner| inner.width()));
        Rc::new(UseHorizontalGridLine(UseGridLine {
            width: self.width,
            colour: self.colour,
            ticks: self.ticks.clone().generate_x(&state.pre, avail_width),
        }))
    }
}

impl<Y: Clone + PartialEq> GridLine<Y> {
    pub(crate) fn use_vertical<X>(self, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        let inner = state.layout.inner;
        let avail_height = Signal::derive(move || with!(|inner| inner.height()));
        Rc::new(UseVerticalGridLine(UseGridLine {
            width: self.width,
            colour: self.colour,
            ticks: self.ticks.clone().generate_y(&state.pre, avail_height),
        }))
    }
}

impl<X: Clone, Y> UseInner<X, Y> for UseHorizontalGridLine<X> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <ViewHorizontalGridLine line=self.0.clone() state=state /> )
    }
}

impl<X, Y: Clone> UseInner<X, Y> for UseVerticalGridLine<Y> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <ViewVerticalGridLine line=self.0.clone() state=state /> )
    }
}

#[component]
fn ViewHorizontalGridLine<X: 'static, Y: 'static>(
    line: UseGridLine<X>,
    state: State<X, Y>,
) -> impl IntoView {
    let debug = state.pre.debug;
    let inner = state.layout.inner;
    let proj = state.projection;
    view! {
        <g class="_chartistry_grid_line_x">
            <DebugRect label="grid_line_x" debug=debug />
            <For
                each=move || for_ticks(line.ticks, proj, true)
                key=|(_, label)| label.to_owned()
                let:tick
            >
                <DebugRect label=format!("grid_line_x/{}", tick.1) debug=debug />
                <line
                    x1=tick.0
                    y1=move || inner.get().top_y()
                    x2=tick.0
                    y2=move || inner.get().bottom_y()
                    stroke=move || line.colour.get().to_string()
                    stroke-width=line.width
                />
            </For>
        </g>
    }
}

#[component]
fn ViewVerticalGridLine<X: 'static, Y: 'static>(
    line: UseGridLine<Y>,
    state: State<X, Y>,
) -> impl IntoView {
    let debug = state.pre.debug;
    let inner = state.layout.inner;
    let proj = state.projection;
    view! {
        <g class="_chartistry_grid_line_y">
            <DebugRect label="grid_line_y" debug=debug />
            <For
                each=move || for_ticks(line.ticks, proj, false)
                key=|(_, label)| label.to_owned()
                let:tick
            >
                <DebugRect label=format!("grid_line_y/{}", tick.1) debug=debug />
                <line
                    x1=move || inner.get().left_x()
                    y1=tick.0
                    x2=move || inner.get().right_x()
                    y2=tick.0
                    stroke=move || line.colour.get().to_string()
                    stroke-width=line.width
                />
            </For>
        </g>
    }
}

fn for_ticks<Tick>(
    ticks: Signal<GeneratedTicks<Tick>>,
    proj: Signal<Projection>,
    is_x: bool,
) -> Vec<(f64, String)> {
    ticks.with(move |ticks| {
        let proj = proj.get();
        ticks
            .ticks
            .iter()
            .map(|tick| {
                let label = ticks.state.long_format(tick);
                let tick = ticks.state.position(tick);
                let tick = if is_x {
                    proj.position_to_svg(tick, 0.0).0
                } else {
                    proj.position_to_svg(0.0, tick).1
                };
                (tick, label)
            })
            .collect::<Vec<_>>()
    })
}
