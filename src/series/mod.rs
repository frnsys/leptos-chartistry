pub mod line;

use self::line::UseLine;
use crate::{
    bounds::Bounds,
    colours::{self, Colour, ColourScheme},
    debug::DebugRect,
    state::State,
    Font,
};
use chrono::prelude::*;
use leptos::*;
use std::rc::Rc;

type GetX<T, X> = Rc<dyn Fn(&T) -> X>;
pub type GetY<T, Y> = Rc<dyn Fn(&T) -> Y>;

#[derive(Clone)]
pub struct SeriesData<T: 'static, X: 'static, Y: 'static> {
    get_x: GetX<T, X>,
    series: Vec<Rc<dyn IntoSeries<T, X, Y>>>,
    colours: ColourScheme,
    x_lower: Signal<Option<X>>,
    x_upper: Signal<Option<X>>,
    y_lower: Signal<Option<Y>>,
    y_upper: Signal<Option<Y>>,
}

pub trait IntoSeries<T, X, Y> {
    fn into_use(self: Rc<Self>, id: usize, colour: Colour) -> (GetY<T, Y>, UseSeries);
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum UseSeries {
    Line(UseLine),
}

#[derive(Clone)]
pub struct UseSeriesData<X: 'static, Y: 'static> {
    pub(crate) series: Vec<UseSeries>,
    pub(crate) data: Signal<Data<X, Y>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Data<X, Y> {
    position_range: Bounds,
    x_points: Vec<X>,
    x_positions: Vec<f64>,
    x_range: Option<(X, X)>,
    y_points: Vec<Y>,
    y_positions: Vec<f64>,
    y_range: Option<(Y, Y)>,
}

impl<T: 'static, X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static>
    SeriesData<T, X, Y>
{
    pub fn new(get_x: impl Fn(&T) -> X + 'static) -> Self {
        Self {
            get_x: Rc::new(get_x),
            series: Vec::new(),
            colours: colours::ARBITRARY.as_ref().into(),
            x_lower: Signal::default(),
            x_upper: Signal::default(),
            y_lower: Signal::default(),
            y_upper: Signal::default(),
        }
    }

    pub fn set_colours(mut self, colours: impl Into<ColourScheme>) -> Self {
        self.colours = colours.into();
        self
    }

    pub fn set_x_min<Opt>(mut self, lower: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<X>> + 'static,
    {
        let lower = lower.into();
        self.x_lower = Signal::derive(move || lower.get().into());
        self
    }

    pub fn set_x_max<Opt>(mut self, upper: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<X>> + 'static,
    {
        let upper = upper.into();
        self.x_upper = Signal::derive(move || upper.get().into());
        self
    }

    pub fn set_x_range<Lower, Upper>(
        self,
        lower: impl Into<MaybeSignal<Lower>>,
        upper: impl Into<MaybeSignal<Upper>>,
    ) -> Self
    where
        Lower: Clone + Into<Option<X>> + 'static,
        Upper: Clone + Into<Option<X>> + 'static,
    {
        self.set_x_min(lower).set_x_max(upper)
    }

    pub fn set_y_min<Opt>(mut self, lower: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<Y>> + 'static,
    {
        let lower = lower.into();
        self.y_lower = Signal::derive(move || lower.get().into());
        self
    }

    pub fn set_y_max<Opt>(mut self, upper: impl Into<MaybeSignal<Opt>>) -> Self
    where
        Opt: Clone + Into<Option<Y>> + 'static,
    {
        let upper = upper.into();
        self.y_upper = Signal::derive(move || upper.get().into());
        self
    }

    pub fn set_y_range<LowerOpt, UpperOpt>(
        self,
        lower: impl Into<MaybeSignal<LowerOpt>>,
        upper: impl Into<MaybeSignal<UpperOpt>>,
    ) -> Self
    where
        LowerOpt: Clone + Into<Option<Y>> + 'static,
        UpperOpt: Clone + Into<Option<Y>> + 'static,
    {
        self.set_y_min(lower).set_y_max(upper)
    }

    pub fn add_series(mut self, series: impl IntoSeries<T, X, Y> + 'static) -> Self {
        self.series.push(Rc::new(series));
        self
    }

    pub fn use_data<Ts>(self, data: impl Into<MaybeSignal<Ts>> + 'static) -> UseSeriesData<X, Y>
    where
        Ts: AsRef<[T]> + 'static,
        X: PartialOrd + Position,
        Y: PartialOrd + Position,
    {
        // Apply colours to lines
        let (get_ys, lines): (Vec<_>, Vec<_>) = self
            .series
            .into_iter()
            .enumerate()
            .zip(self.colours.iter())
            .map(|((id, series), colour)| series.into_use(id, colour))
            .unzip();

        // Convert data to a signal
        let data = data.into();
        let data = create_memo(move |_| {
            let get_x = self.get_x.clone();
            let x_lower = self.x_lower.get();
            let x_upper = self.x_upper.get();
            let y_lower = self.y_lower.get();
            let y_upper = self.y_upper.get();
            let get_ys = get_ys.iter().as_slice();
            data.with(move |data| {
                let data = data.as_ref();
                // Collect data points
                let x_points = data.iter().map(|datum| (get_x)(datum)).collect::<Vec<_>>();
                let x_positions = x_points.iter().map(|x| x.position()).collect::<Vec<_>>();
                let y_points = get_ys
                    .iter()
                    .flat_map(|get_y| data.iter().map(|datum| (get_y)(datum)))
                    .collect::<Vec<_>>();
                let y_positions = y_points.iter().map(|y| y.position()).collect::<Vec<_>>();

                // Find min/max
                let x_range = Self::find_min_max_index(&x_positions)
                    .map(|(min_i, max_i)| (get_x(&data[min_i]), get_x(&data[max_i])))
                    .map(Self::map_min_max_range(x_lower, x_upper));
                let y_range = Self::find_min_max_index(&y_positions)
                    .map(|(min_i, max_i)| {
                        (
                            Self::reverse_get_y(get_ys, data, min_i),
                            Self::reverse_get_y(get_ys, data, max_i),
                        )
                    })
                    .map(Self::map_min_max_range(y_lower, y_upper));

                let position_range = {
                    let (x_min, x_max) = x_range
                        .as_ref()
                        .map(|(min, max)| (min.position(), max.position()))
                        .unwrap_or_default();
                    let (y_min, y_max) = y_range
                        .as_ref()
                        .map(|(min, max)| (min.position(), max.position()))
                        .unwrap_or_default();
                    Bounds::from_points(
                        x_min.position(),
                        y_min.position(),
                        x_max.position(),
                        y_max.position(),
                    )
                };

                Data {
                    position_range,
                    x_points,
                    x_positions,
                    x_range,
                    y_points,
                    y_positions,
                    y_range,
                }
            })
        })
        .into();

        UseSeriesData {
            series: lines,
            data,
        }
    }

    fn map_min_max_range<V: PartialOrd>(
        lower: Option<V>,
        upper: Option<V>,
    ) -> impl FnOnce((V, V)) -> (V, V) {
        |(min, max)| {
            (
                lower
                    .and_then(|v| if v < min { Some(v) } else { None })
                    .unwrap_or(min),
                upper
                    .and_then(|v| if v > max { Some(v) } else { None })
                    .unwrap_or(max),
            )
        }
    }

    fn find_min_max_index(positions: &[f64]) -> Option<(usize, usize)> {
        positions.iter().enumerate().fold(None, |range, (i, &pos)| {
            // Skip NaN values
            if pos.is_nan() {
                return range;
            };
            range.map_or_else(
                || Some((i, i)), // First seen
                |(min_i, max_i)| {
                    // Find index of min/max
                    Some((
                        if pos < positions[min_i] { i } else { min_i },
                        if pos > positions[max_i] { i } else { max_i },
                    ))
                },
            )
        })
    }

    /// Given an Data::y_points index, return the corresponding y value. Note that y_points is a flat map of all the y values for each series.
    fn reverse_get_y(get_ys: &[GetY<T, Y>], data: &[T], index: usize) -> Y {
        let series_i = index / data.len();
        let data_i = index % data.len();
        (get_ys[series_i])(&data[data_i])
    }
}

impl<X, Y> Data<X, Y> {
    pub fn position_range(&self) -> Bounds {
        self.position_range
    }

    pub fn x_range(&self) -> Option<&(X, X)> {
        self.x_range.as_ref()
    }

    pub fn y_range(&self) -> Option<&(Y, Y)> {
        self.y_range.as_ref()
    }

    fn nearest_x_index(&self, pos: f64) -> Option<usize> {
        // No values
        if self.x_positions.is_empty() {
            return None;
        }
        // Find index after pos
        let index = self.x_positions.partition_point(|&v| v < pos);
        // No value before
        if index == 0 {
            return Some(0);
        }
        // No value ahead
        if index == self.x_points.len() {
            return Some(index - 1);
        }
        // Find closest index
        let ahead = self.x_positions[index] - pos;
        let before = pos - self.x_positions[index - 1];
        if ahead < before {
            Some(index)
        } else {
            Some(index - 1)
        }
    }

    pub fn nearest_x(&self, x_pos: f64) -> Option<&X> {
        self.nearest_x_index(x_pos)
            .map(|x_index| &self.x_points[x_index])
    }

    /// Given an arbitrary (unaligned to data) X position, find the nearest X position aligned to data. Returns `f64::NAN` if no data.
    pub fn nearest_x_position(&self, x_pos: f64) -> f64 {
        self.nearest_x_index(x_pos)
            .map(|x_index| self.x_positions[x_index])
            .unwrap_or(f64::NAN)
    }

    pub fn nearest_y(&self, x_pos: f64, line_id: usize) -> Option<Y>
    where
        Y: Clone,
    {
        self.nearest_x_index(x_pos).map(|x_index| {
            let index = line_id * self.x_points.len() + x_index;
            self.y_points[index].clone()
        })
    }
}

pub trait Position {
    fn position(&self) -> f64;
}

impl Position for f64 {
    fn position(&self) -> f64 {
        *self
    }
}

impl<Tz: TimeZone> Position for DateTime<Tz> {
    fn position(&self) -> f64 {
        self.timestamp() as f64 + (self.timestamp_subsec_nanos() as f64 / 1e9)
    }
}

impl UseSeries {
    pub fn id(&self) -> usize {
        match self {
            Self::Line(line) => line.id(),
        }
    }

    pub fn name(&self) -> MaybeSignal<String> {
        match self {
            Self::Line(line) => line.name(),
        }
    }

    pub fn taster_bounds(font: Signal<Font>) -> Memo<Bounds> {
        create_memo(move |_| {
            let font = font.get();
            Bounds::new(font.width() * 2.0, font.height())
        })
    }

    pub fn snippet_width(font: Signal<Font>) -> Signal<f64> {
        let taster_bounds = Self::taster_bounds(font);
        Signal::derive(move || taster_bounds.get().width() + font.get().width())
    }

    pub fn taster<X, Y>(&self, bounds: Memo<Bounds>, state: &State<X, Y>) -> View {
        match self {
            Self::Line(line) => line.taster(bounds, state),
        }
    }

    pub fn render<X, Y>(&self, positions: Signal<Vec<(f64, f64)>>, state: &State<X, Y>) -> View {
        match self {
            Self::Line(line) => line.render(positions, state),
        }
    }
}

impl<X: Clone, Y: Clone> UseSeriesData<X, Y> {
    pub fn render(self, state: &State<X, Y>) -> View {
        view!( <RenderSeriesData series=self state=state /> )
    }
}

#[component]
pub fn Snippet<'a, X: 'static, Y: 'static>(
    series: UseSeries,
    state: &'a State<X, Y>,
) -> impl IntoView {
    let debug = state.pre.debug;
    let name = series.name();
    view! {
        <div class="_chartistry_snippet" style="white-space: nowrap;">
            <DebugRect label="snippet" debug=debug />
            <Taster series=series state=state />
            {name}
        </div>
    }
}

#[component]
pub fn Taster<'a, X: 'static, Y: 'static>(
    series: UseSeries,
    state: &'a State<X, Y>,
) -> impl IntoView {
    let debug = state.pre.debug;
    let font = state.pre.font;
    let bounds = UseSeries::taster_bounds(font);
    view! {
        <svg
            class="_chartistry_taster"
            width=move || bounds.get().width()
            height=move || bounds.get().height()
            viewBox=move || format!("0 0 {} {}", bounds.get().width(), bounds.get().height())
            style:padding-right=move || format!("{}px", font.get().width())>
            <DebugRect label="taster" debug=debug bounds=vec![bounds.into()] />
            {series.taster(bounds, state)}
        </svg>
    }
}

#[component]
pub fn RenderSeriesData<'a, X: Clone + 'static, Y: Clone + 'static>(
    series: UseSeriesData<X, Y>,
    state: &'a State<X, Y>,
) -> impl IntoView {
    let proj = state.projection;
    let get_positions = move |id| {
        Signal::derive(move || {
            series.data.with(|data| {
                let proj = proj.get();
                let points = data.x_points.len();
                let start = id * points;
                let end = start + points;
                data.x_positions
                    .iter()
                    .zip(&data.y_positions[start..end])
                    .map(|(x, y)| {
                        // Map from data to viewport coords
                        proj.data_to_svg(*x, *y)
                    })
                    .collect::<Vec<_>>()
            })
        })
    };

    let render = {
        let state = state.clone();
        move |series: UseSeries| {
            let id = series.id();
            let positions = get_positions(id);
            series.render(positions, &state)
        }
    };

    view! {
        <g class="_chartistry_series">
            <For
                each=move || series.series.to_vec()
                key=|series| series.id()
                children=render
            />
        </g>
    }
}
