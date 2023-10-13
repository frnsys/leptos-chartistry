use std::borrow::Borrow;

use super::{InnerLayout, InnerOption, UseInner};
use crate::{
    chart::Attr,
    edge::Edge,
    layout::legend::{LegendAttr, UseLegend},
    projection::Projection,
    series::UseSeries,
    Anchor, Legend, Snippet,
};
use leptos::*;

#[derive(Clone, Debug)]
pub struct InsetLegend {
    edge: Edge,
    legend: Legend,
}

#[derive(Clone, Debug)]
pub struct InsetLegendAttr {
    edge: Edge,
    legend: LegendAttr,
}

#[derive(Clone, Debug)]
pub struct UseInsetLegend {
    edge: Edge,
    legend: UseLegend,
}

impl InsetLegend {
    fn new(edge: Edge, anchor: Anchor, snippet: impl Borrow<Snippet>) -> Self {
        Self {
            edge,
            legend: Legend::new(anchor, snippet.borrow().clone()),
        }
    }

    pub fn top_left(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Top, Anchor::Start, snippet)
    }
    pub fn top(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Top, Anchor::Middle, snippet)
    }
    pub fn top_right(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Top, Anchor::Middle, snippet)
    }
    pub fn bottom_left(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Bottom, Anchor::Start, snippet)
    }
    pub fn bottom(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Bottom, Anchor::Middle, snippet)
    }
    pub fn bottom_right(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Bottom, Anchor::Middle, snippet)
    }
    pub fn left(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Left, Anchor::Middle, snippet)
    }
    pub fn right(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Right, Anchor::Middle, snippet)
    }
}

impl<X, Y> InnerLayout<X, Y> for InsetLegend {
    fn apply_attr(self, attr: &Attr) -> Box<dyn InnerOption<X, Y>> {
        Box::new(InsetLegendAttr {
            legend: self.legend.apply_attr(attr),
            edge: self.edge,
        })
    }
}

impl<X, Y> InnerOption<X, Y> for InsetLegendAttr {
    fn to_use(
        self: Box<Self>,
        series: &UseSeries<X, Y>,
        _: Signal<Projection>,
    ) -> Box<dyn UseInner> {
        Box::new(UseInsetLegend {
            legend: self.legend.to_use(series),
            edge: self.edge,
        })
    }
}

impl UseInner for UseInsetLegend {
    fn render(self: Box<Self>, proj: Signal<Projection>, _: Signal<Option<(f64, f64)>>) -> View {
        view!( <InsetLegend legend=self.legend edge=self.edge projection=proj /> )
    }
}

#[component]
fn InsetLegend(legend: UseLegend, edge: Edge, projection: Signal<Projection>) -> impl IntoView {
    let (height, width) = (legend.height(), legend.width());
    let bounds = Signal::derive(move || {
        let bounds = projection.get().bounds();
        let height = height.get();
        let width = width.get();
        // Build legend bounds as an inset of the chart bounds
        let (top, right, bottom, left) = match edge {
            Edge::Top => (0.0, 0.0, bounds.height() - height, 0.0),
            Edge::Bottom => (bounds.height() - height, 0.0, 0.0, 0.0),
            Edge::Left => (0.0, bounds.width() - width, 0.0, 0.0),
            Edge::Right => (0.0, 0.0, 0.0, bounds.width() - width),
        };
        bounds.shrink(top, right, bottom, left)
    });

    view! {
        <g class="_chartistry_legend_inset">
            <Legend legend=legend edge=edge bounds=bounds />
        </g>
    }
}
