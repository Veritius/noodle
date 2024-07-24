//! Widgets for displaying [`Graph`] objects.

use egui::{Direction, Vec2, Widget};
use noodle_core::Graph;

/// A builder for a graph view.
pub struct GraphViewBuilder<'a, G: Graph> {
    graph: &'a mut G,

    max_size: Vec2,
    direction: Direction,

    hard_edges: bool,

    allow_panning: bool,
    allow_zooming: bool,
    min_zoom: f32,
    max_zoom: f32,
}

impl<'a, G: Graph> GraphViewBuilder<'a, G> {
    /// Creates a new [`GraphViewBuilder`].
    pub fn new(graph: &'a mut G) -> Self {
        Self {
            graph,

            max_size: Vec2::INFINITY,
            direction: Direction::LeftToRight,

            hard_edges: false,

            allow_panning: true,
            allow_zooming: true,
            min_zoom: 0.2,
            max_zoom: 5.0,
        }
    }

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

    /// The direction the graph is laid out in.
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

impl<'a, G: Graph> Widget for GraphViewBuilder<'a, G> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        todo!()
    }
}