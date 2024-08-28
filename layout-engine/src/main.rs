//! This is going to be incredible lazy
use layout::core::format::ClipHandle;
use layout::core::format::RenderBackend;
use layout::core::geometry::Point;
use layout::core::style::StyleAttr;
use layout::gv;

#[derive(Clone, Default, Debug)]
pub struct RenderedGraph {
    items: Vec<Node>,
}

#[derive(Clone, Debug)]
pub struct Style {
    fill_colour: Option<String>,
    line_colour: String,
    width: usize,
    rounded: usize,
    font_size: usize,
}

pub struct Point2d {
    x: f64,
    y: f64,
}

impl From<&Point> for Point2d {
    fn from(p: &Point) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl From<&StyleAttr> for Style {
    fn from(style: &StyleAttr) -> Self {
        Self {
            fill_colour: style.fill_color.map(|x| x.to_web_color()),
            line_colour: style.line_color.to_web_color(),
            width: style.line_width,
            rounded: style.rounded,
            font_size: style.font_size,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    style: Style,
}

#[derive(Clone, Debug)]
pub struct Circle {
    x: f64,
    y: f64,
    r: f64,
    style: Style,
}

#[derive(Clone, Debug)]
pub struct Text {
    x: f64,
    y: f64,
    style: Style,
    text: String,
}

pub struct Arrow {
    segments: Vec<Line>,
    style: Style,
    start_head: bool,
    end_head: bool,
}

pub struct Line {
    start: Point2d,
    end: Point2d,
    style: Option<Style>,
}

#[derive(Clone, Debug)]
pub enum Node {
    Rect(Rectangle),
    Circle(Circle),
    Text(Text),
    Arrow(Arrow),
    Line(Line),
}

impl RenderBackend for RenderedGraph {
    // Required methods
    fn draw_rect(&mut self, xy: Point, size: Point, look: &StyleAttr, clip: Option<ClipHandle>) {
        self.items.push(Node::Rect(Rectangle {
            style: look.into(),
            x: xy.x,
            y: xy.y,
            width: size.x,
            height: size.y,
        }));
    }

    fn draw_line(&mut self, start: Point, stop: Point, look: &StyleAttr) {
        todo!()
    }
    fn draw_circle(&mut self, xy: Point, size: Point, look: &StyleAttr) {
        todo!()
    }
    fn draw_text(&mut self, xy: Point, text: &str, look: &StyleAttr) {
        self.items.push(Node::Text(Text {
            x: xy.x,
            y: xy.y,
            style: look.into(),
            text: text.to_string(),
        }));
    }
    fn draw_arrow(
        &mut self,
        path: &[(Point, Point)],
        dashed: bool,
        head: (bool, bool),
        look: &StyleAttr,
        text: &str,
    ) {
        todo!()
    }
    fn create_clip(&mut self, xy: Point, size: Point, rounded_px: usize) -> ClipHandle {
        todo!()
    }
}

fn main() {
    let contents = r#"
digraph Q {

  node [shape=record];


  nd_1   [label = "Node 1"];
  nd_2   [label = "Node 2"];
  nd_3_a [label = "Above Right Node 3"];
  nd_3_l [label = "Left of Node 3"];
  nd_3   [label = "Node 3"];
  nd_3_r [label = "Right of Node 3"];
  nd_4   [label = "Node 4"];


  nd_3_a -> nd_3_r;
  nd_1 -> nd_2 -> nd_3 -> nd_4;

  subgraph cluster_R {

    {rank=same nd_3_l nd_3 nd_3_r}

    nd_3_l -> nd_3 -> nd_3_r [color=grey arrowhead=none];

  }
}"#;
    let mut parser = gv::DotParser::new(contents);

    let graph = parser.process().unwrap();

    let mut builder = gv::GraphBuilder::new();
    builder.visit_graph(&graph);

    let mut viz_graph = builder.get();

    let mut store = RenderedGraph::default();

    viz_graph.do_it(false, false, false, &mut store);

    println!("{:?}", store);
}
