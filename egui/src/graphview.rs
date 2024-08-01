//! Widgets for displaying [`Graph`] objects.

use std::marker::PhantomData;
use egui::{Color32, Direction, LayerId, Rect, Response, Rounding, Sense, Vec2, Widget};
use noodle_core::*;

/// A builder for a graph view.
/// 
/// Can be constructed from the following types:
/// - `&G` where `G: Graph` - read-only graph view
/// - `&mut G` where `G: Graph` - editable graph view
pub struct GraphViewBuilder<G, N> {
    graph: G,
    ph: PhantomData<N>,

    max_size: Vec2,
    rounding: Rounding,
    direction: Direction,

    hard_edges: bool,

    allow_panning: bool,
    allow_zooming: bool,
    min_zoom: f32,
    max_zoom: f32,
}

impl<G, N> GraphViewBuilder<G, N> {
    fn new_inner(graph: G) -> Self {
        Self {
            graph,
            ph: PhantomData,

            max_size: Vec2::INFINITY,
            rounding: Rounding::same(5.0),
            direction: Direction::LeftToRight,

            hard_edges: false,

            allow_panning: true,
            allow_zooming: true,
            min_zoom: 0.2,
            max_zoom: 5.0,
        }
    }
}

impl<'a, G, N> GraphViewBuilder<&'a G, N> {
    /// Creates a new, read-only [`GraphViewBuilder`].
    pub fn new(graph: &'a G) -> Self {
        Self::new_inner(graph)
    }
}

impl<'a, G, N> GraphViewBuilder<&'a mut G, N> {
    /// Creates a new, editable [`GraphViewBuilder`].
    pub fn new(graph: &'a mut G) -> Self {
        Self::new_inner(graph)
    }
}

impl<G, N> GraphViewBuilder<G, N> {
    /// Sets the maximum width of the outer frame of the graph view.
    /// 
    /// Use `f32::INFINITY` if you want the graph view to fit the surrounding area.
    /// This is also the default.
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_size.x = width;
        return self;
    }

    /// Sets the maximum height of the outer frame of the graph view.
    /// 
    /// Use `f32::INFINITY` if you want the graph view to fit the surrounding area.
    /// This is also the default.
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_size.y = height;
        return self;
    }

    /// Sets the rounding applied to the corners of the graph view.
    /// 
    /// Defaults to `5.0` at all corners.
    pub fn rounding(mut self, rounding: impl Into<Rounding>) -> Self {
        self.rounding = rounding.into();
        return self;
    }

    /// The direction the graph is laid out in.
    /// 
    /// Defaults to [`LeftToRight`](Direction::LeftToRight).
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        return self;
    }

    /// Whether or not edges are 'hard'.
    /// With hard edges, nodes outside of view will be snapped into view.
    /// 
    /// Defaults to `false`.
    pub fn hard_edges(mut self, hard: bool) -> Self {
        self.hard_edges = hard;
        return self;
    }

    /// Whether or not to allow zooming in and out in the graph view.
    /// 
    /// Defaults to `true`.
    pub fn allow_zooming(mut self, allowed: bool) -> Self {
        self.allow_zooming = allowed;
        return self;
    }

    /// Set how far out the user can zoom.
    /// 
    /// Defaults to `0.2`.
    pub fn min_zoom(mut self, min_zoom: f32) -> Self {
        self.min_zoom = min_zoom;
        return self;
    }

    /// Set how far in the user can zoom.
    /// 
    /// Defaults to `2.0`.
    pub fn max_zoom(mut self, max_zoom: f32) -> Self {
        self.max_zoom = max_zoom;
        return self;
    }

    /// Whether or not to allow panning across the graph view.
    /// It's recommended to set [`hard_edges`](Self::hard_edges) to true when disabling this,
    /// otherwise previously-accessible nodes may become impossible to see, and therefore edit.
    /// 
    /// Defaults to `true`.
    pub fn allow_panning(mut self, allowed: bool) -> Self {
        self.allow_panning = allowed;
        return self;
    }
}

impl<'a, G: Graph<N>, N: Node> Widget for GraphViewBuilder<&'a G, N> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, bg_response) = layout_view_rect(ui, self.max_size);
        let painter = ui.painter().clone()
            .with_clip_rect(rect)
            .with_layer_id(LayerId::background());

        painter.rect_filled(rect, self.rounding, Color32::DARK_GRAY);

        return bg_response
    }
}

impl<'a, G: Graph<N>, N: Node> Widget for GraphViewBuilder<&'a mut G, N> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        todo!()
    }
}

fn layout_view_rect(
    ui: &mut egui::Ui,
    desired_size: Vec2,
) -> (Rect, Response) {
    let maximum_size = ui.available_size().min(desired_size);
    ui.allocate_exact_size(maximum_size, Sense::click_and_drag())
}