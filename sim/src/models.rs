use glium::{VertexBuffer, IndexBuffer};
use glium::index::PrimitiveType;
use glium::backend::Facade;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

pub fn triangle<F: Facade>(display: &F) -> (VertexBuffer<Vertex>, IndexBuffer<u16>) {
    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = VertexBuffer::new(display, &[
        Vertex { position: [-0.5, -0.5, 1.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 0.0,  0.5, 1.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [ 0.5, -0.5, 1.0], color: [1.0, 0.0, 0.0] },
    ]).unwrap();

    // building the index buffer
    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList,
                                               &[0u16, 1, 2]).unwrap();

    (vertex_buffer, index_buffer)
}

pub fn square<F: Facade>(display: &F) -> (VertexBuffer<Vertex>, IndexBuffer<u16>) {
    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = VertexBuffer::new(display, &[
        Vertex { position: [-0.5, -0.5, 1.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [-0.5,  0.5, 1.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [ 0.5,  0.5, 1.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [ 0.5, -0.5, 1.0], color: [1.0, 0.0, 0.0] },
    ]).unwrap();

    // building the index buffer
    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList,
                                               &[0u16, 1, 2, 0, 2, 3]).unwrap();

    (vertex_buffer, index_buffer)
}

pub const FIELD_LENGTH: f32    = 9.000;
pub const FIELD_WIDTH: f32     = 6.000;
pub const LINE_WIDTH: f32      = 0.010;
pub const CENTER_DIAMETER: f32 = 1.000;
pub const DEFENSE_RADIUS: f32  = 1.000;
pub const DEFENSE_STRETCH: f32 = 0.500;
pub const FIELD_MARGIN: f32    = 0.300;
pub const REFEREE_MARGIN: f32  = 0.700;
pub const GOAL_WIDTH: f32      = 1.000;
pub const GOAL_DEPTH: f32      = 0.180;
pub const GOAL_WALL_WIDTH: f32 = 0.020;

#[allow(dead_code)]
enum TeamSide {
    Undefined,
    YellowIsLeft,
    BlueIsLeft,
}

pub fn field<F: Facade>(display: &F) -> (VertexBuffer<Vertex>, IndexBuffer<u16>) {
    let white = { let (r, g, b) = ::colors::WHITE; [r, g, b] };
    //let grey = { let (r, g, b) = ::colors::GREY; [r, g, b] };
    let black = { let (r, g, b) = ::colors::BLACK; [r, g, b] };
    let yellow = { let (r, g, b) = ::colors::YELLOW; [r, g, b] };
    let blue = { let (r, g, b) = ::colors::BLUE; [r, g, b] };
    let field = { let (r, g, b) = ::colors::FIELD_GREEN; [r, g, b] };
    let spot = white;
    let team_side = TeamSide::BlueIsLeft;
    let (left, right) = match team_side {
        TeamSide::Undefined => (black, black),
        TeamSide::YellowIsLeft => (yellow, blue),
        TeamSide::BlueIsLeft => (blue, yellow),
    };
    const Z: f32 = 0.0;

    const FIELD_HLENGTH: f32    = FIELD_LENGTH / 2.0;
    const FIELD_HWIDTH: f32     = FIELD_WIDTH  / 2.0;
    const FIELD_RLENGTH: f32    = FIELD_LENGTH / 2.0 + FIELD_MARGIN;
    const FIELD_RWIDTH: f32     = FIELD_WIDTH  / 2.0 + FIELD_MARGIN;;
    const FIELD_ILENGTH: f32    = FIELD_LENGTH / 2.0 - LINE_WIDTH;
    const FIELD_IWIDTH: f32     = FIELD_WIDTH  / 2.0 - LINE_WIDTH;
    const LINE_HWIDTH: f32      = LINE_WIDTH / 2.0;
    const SPOT_RADIUS: f32      = LINE_WIDTH; // XXX: this is actually inaccurate
    const CENTER_RADIUS: f32    = CENTER_DIAMETER / 2.0;
    const CENTER_IRADIUS: f32   = CENTER_RADIUS - LINE_WIDTH;
    const DEFENSE_HSTRETCH: f32 = DEFENSE_STRETCH / 2.0;
    const DEFENSE_IRADIUS: f32  = DEFENSE_RADIUS - LINE_WIDTH;
    const GOAL_HWIDTH: f32      = GOAL_WIDTH / 2.0;

    fn arc(cx: f32, cy: f32, r: f32, a: f32) -> [f32; 3] {
        let w = a.to_radians();
        [cx + r * w.cos(), cy + r * w.sin(), Z]
    }

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = VertexBuffer::new(display, &[
        // outer field vertices
        /*   0 */ Vertex { position: [-FIELD_HLENGTH, -FIELD_HWIDTH, Z], color: white },
        /*   1 */ Vertex { position: [ FIELD_HLENGTH, -FIELD_HWIDTH, Z], color: white },
        /*   2 */ Vertex { position: [ FIELD_HLENGTH,  FIELD_HWIDTH, Z], color: white },
        /*   3 */ Vertex { position: [-FIELD_HLENGTH,  FIELD_HWIDTH, Z], color: white },
        // inner field vertices
        /*   4 */ Vertex { position: [-FIELD_ILENGTH, -FIELD_IWIDTH, Z], color: white },
        /*   5 */ Vertex { position: [ FIELD_ILENGTH, -FIELD_IWIDTH, Z], color: white },
        /*   6 */ Vertex { position: [ FIELD_ILENGTH,  FIELD_IWIDTH, Z], color: white },
        /*   7 */ Vertex { position: [-FIELD_ILENGTH,  FIELD_IWIDTH, Z], color: white },
        // middle line vertices
        /*   8 */ Vertex { position: [-LINE_HWIDTH, -FIELD_IWIDTH, Z], color: white },
        /*   9 */ Vertex { position: [ LINE_HWIDTH, -FIELD_IWIDTH, Z], color: white },
        /*  10 */ Vertex { position: [ LINE_HWIDTH,  FIELD_IWIDTH, Z], color: white },
        /*  11 */ Vertex { position: [-LINE_HWIDTH,  FIELD_IWIDTH, Z], color: white },
        // longitudinal line vertices TODO
        /*  12 */ Vertex { position: [-FIELD_ILENGTH, -LINE_HWIDTH, Z], color: white },
        /*  13 */ Vertex { position: [ FIELD_ILENGTH, -LINE_HWIDTH, Z], color: white },
        /*  14 */ Vertex { position: [ FIELD_ILENGTH,  LINE_HWIDTH, Z], color: white },
        /*  15 */ Vertex { position: [-FIELD_ILENGTH,  LINE_HWIDTH, Z], color: white },
        // center spot vertices
        /*  16 */ Vertex { position: [0.0, 0.0, Z], color: spot },
        /*  17 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS,   0.0), color: spot },
        /*  18 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS,  15.0), color: spot },
        /*  19 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS,  30.0), color: spot },
        /*  20 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS,  45.0), color: spot },
        /*  21 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS,  60.0), color: spot },
        /*  22 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS,  75.0), color: spot },
        /*  23 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS,  90.0), color: spot },
        /*  24 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 105.0), color: spot },
        /*  25 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 120.0), color: spot },
        /*  26 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 135.0), color: spot },
        /*  27 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 150.0), color: spot },
        /*  28 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 165.0), color: spot },
        /*  29 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 180.0), color: spot },
        /*  30 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 195.0), color: spot },
        /*  31 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 210.0), color: spot },
        /*  32 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 225.0), color: spot },
        /*  33 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 240.0), color: spot },
        /*  34 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 255.0), color: spot },
        /*  35 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 270.0), color: spot },
        /*  36 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 285.0), color: spot },
        /*  37 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 300.0), color: spot },
        /*  38 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 315.0), color: spot },
        /*  39 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 330.0), color: spot },
        /*  40 */ Vertex { position: arc(0.0, 0.0, SPOT_RADIUS, 345.0), color: spot },
        // center circle inner, vertices
        /*  41 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,   0.0), color: white },
        /*  42 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,   6.0), color: white },
        /*  43 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  12.0), color: white },
        /*  44 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  18.0), color: white },
        /*  45 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  24.0), color: white },
        /*  46 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  30.0), color: white },
        /*  47 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  36.0), color: white },
        /*  48 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  42.0), color: white },
        /*  49 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  48.0), color: white },
        /*  50 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  54.0), color: white },
        /*  51 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  60.0), color: white },
        /*  52 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  66.0), color: white },
        /*  53 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  72.0), color: white },
        /*  54 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  78.0), color: white },
        /*  55 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  84.0), color: white },
        /*  56 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  90.0), color: white },
        /*  57 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS,  96.0), color: white },
        /*  58 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 102.0), color: white },
        /*  59 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 108.0), color: white },
        /*  60 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 114.0), color: white },
        /*  61 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 120.0), color: white },
        /*  62 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 126.0), color: white },
        /*  63 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 132.0), color: white },
        /*  64 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 138.0), color: white },
        /*  65 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 144.0), color: white },
        /*  66 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 150.0), color: white },
        /*  67 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 156.0), color: white },
        /*  68 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 162.0), color: white },
        /*  69 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 168.0), color: white },
        /*  70 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 174.0), color: white },
        /*  71 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 180.0), color: white },
        /*  72 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 186.0), color: white },
        /*  73 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 192.0), color: white },
        /*  74 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 198.0), color: white },
        /*  75 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 204.0), color: white },
        /*  76 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 210.0), color: white },
        /*  77 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 216.0), color: white },
        /*  78 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 222.0), color: white },
        /*  79 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 228.0), color: white },
        /*  80 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 234.0), color: white },
        /*  81 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 240.0), color: white },
        /*  82 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 246.0), color: white },
        /*  83 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 252.0), color: white },
        /*  84 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 258.0), color: white },
        /*  85 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 264.0), color: white },
        /*  86 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 270.0), color: white },
        /*  87 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 276.0), color: white },
        /*  88 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 282.0), color: white },
        /*  89 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 288.0), color: white },
        /*  90 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 294.0), color: white },
        /*  91 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 300.0), color: white },
        /*  92 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 306.0), color: white },
        /*  93 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 312.0), color: white },
        /*  94 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 318.0), color: white },
        /*  95 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 324.0), color: white },
        /*  96 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 330.0), color: white },
        /*  97 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 336.0), color: white },
        /*  98 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 342.0), color: white },
        /*  99 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 348.0), color: white },
        /* 100 */ Vertex { position: arc(0.0, 0.0, CENTER_IRADIUS, 354.0), color: white },
        // center circle, outer vertices
        /* 101 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,   0.0), color: white },
        /* 102 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,   6.0), color: white },
        /* 103 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  12.0), color: white },
        /* 104 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  18.0), color: white },
        /* 105 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  24.0), color: white },
        /* 106 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  30.0), color: white },
        /* 107 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  36.0), color: white },
        /* 108 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  42.0), color: white },
        /* 109 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  48.0), color: white },
        /* 110 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  54.0), color: white },
        /* 111 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  60.0), color: white },
        /* 112 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  66.0), color: white },
        /* 113 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  72.0), color: white },
        /* 114 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  78.0), color: white },
        /* 115 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  84.0), color: white },
        /* 116 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  90.0), color: white },
        /* 117 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS,  96.0), color: white },
        /* 118 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 102.0), color: white },
        /* 119 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 108.0), color: white },
        /* 120 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 114.0), color: white },
        /* 121 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 120.0), color: white },
        /* 122 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 126.0), color: white },
        /* 123 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 132.0), color: white },
        /* 124 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 138.0), color: white },
        /* 125 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 144.0), color: white },
        /* 126 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 150.0), color: white },
        /* 127 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 156.0), color: white },
        /* 128 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 162.0), color: white },
        /* 129 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 168.0), color: white },
        /* 130 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 174.0), color: white },
        /* 131 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 180.0), color: white },
        /* 132 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 186.0), color: white },
        /* 133 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 192.0), color: white },
        /* 134 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 198.0), color: white },
        /* 135 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 204.0), color: white },
        /* 136 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 210.0), color: white },
        /* 137 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 216.0), color: white },
        /* 138 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 222.0), color: white },
        /* 139 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 228.0), color: white },
        /* 140 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 234.0), color: white },
        /* 141 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 240.0), color: white },
        /* 142 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 246.0), color: white },
        /* 143 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 252.0), color: white },
        /* 144 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 258.0), color: white },
        /* 145 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 264.0), color: white },
        /* 146 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 270.0), color: white },
        /* 147 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 276.0), color: white },
        /* 148 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 282.0), color: white },
        /* 149 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 288.0), color: white },
        /* 150 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 294.0), color: white },
        /* 151 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 300.0), color: white },
        /* 152 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 306.0), color: white },
        /* 153 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 312.0), color: white },
        /* 154 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 318.0), color: white },
        /* 155 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 324.0), color: white },
        /* 156 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 330.0), color: white },
        /* 157 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 336.0), color: white },
        /* 158 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 342.0), color: white },
        /* 159 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 348.0), color: white },
        /* 160 */ Vertex { position: arc(0.0, 0.0, CENTER_RADIUS, 354.0), color: white },
        // left defense area, inner vertices
        /* 161 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -90.0), color: white },
        /* 162 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -84.0), color: white },
        /* 163 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -78.0), color: white },
        /* 164 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -72.0), color: white },
        /* 165 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -66.0), color: white },
        /* 166 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -60.0), color: white },
        /* 167 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -54.0), color: white },
        /* 168 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -48.0), color: white },
        /* 169 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -42.0), color: white },
        /* 170 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -36.0), color: white },
        /* 171 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -30.0), color: white },
        /* 172 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -24.0), color: white },
        /* 173 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -18.0), color: white },
        /* 174 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, -12.0), color: white },
        /* 175 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  -6.0), color: white },
        /* 176 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS,   0.0), color: white },
        /* 177 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,   0.0), color: white },
        /* 178 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,   6.0), color: white },
        /* 179 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  12.0), color: white },
        /* 180 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  18.0), color: white },
        /* 181 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  24.0), color: white },
        /* 182 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  30.0), color: white },
        /* 183 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  36.0), color: white },
        /* 184 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  42.0), color: white },
        /* 185 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  48.0), color: white },
        /* 186 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  54.0), color: white },
        /* 187 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  60.0), color: white },
        /* 188 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  66.0), color: white },
        /* 189 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  72.0), color: white },
        /* 190 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  78.0), color: white },
        /* 191 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  84.0), color: white },
        /* 192 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  90.0), color: white },
        // left defense area, outer vertices
        /* 193 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -90.0), color: white },
        /* 194 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -84.0), color: white },
        /* 195 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -78.0), color: white },
        /* 196 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -72.0), color: white },
        /* 197 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -66.0), color: white },
        /* 198 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -60.0), color: white },
        /* 199 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -54.0), color: white },
        /* 200 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -48.0), color: white },
        /* 201 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -42.0), color: white },
        /* 202 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -36.0), color: white },
        /* 203 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -30.0), color: white },
        /* 204 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -24.0), color: white },
        /* 205 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -18.0), color: white },
        /* 206 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, -12.0), color: white },
        /* 207 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS,  -6.0), color: white },
        /* 208 */ Vertex { position: arc(-FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS,   0.0), color: white },
        /* 209 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,   0.0), color: white },
        /* 210 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,   6.0), color: white },
        /* 211 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  12.0), color: white },
        /* 212 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  18.0), color: white },
        /* 213 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  24.0), color: white },
        /* 214 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  30.0), color: white },
        /* 215 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  36.0), color: white },
        /* 216 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  42.0), color: white },
        /* 217 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  48.0), color: white },
        /* 218 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  54.0), color: white },
        /* 219 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  60.0), color: white },
        /* 220 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  66.0), color: white },
        /* 221 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  72.0), color: white },
        /* 222 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  78.0), color: white },
        /* 223 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  84.0), color: white },
        /* 224 */ Vertex { position: arc(-FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  90.0), color: white },
        // right defense area, inner vertices
        /* 225 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  90.0), color: white },
        /* 226 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS,  96.0), color: white },
        /* 227 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 102.0), color: white },
        /* 228 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 108.0), color: white },
        /* 229 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 114.0), color: white },
        /* 230 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 120.0), color: white },
        /* 231 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 126.0), color: white },
        /* 232 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 132.0), color: white },
        /* 233 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 138.0), color: white },
        /* 234 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 144.0), color: white },
        /* 235 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 150.0), color: white },
        /* 236 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 156.0), color: white },
        /* 237 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 162.0), color: white },
        /* 238 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 168.0), color: white },
        /* 239 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 174.0), color: white },
        /* 240 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 180.0), color: white },
        /* 241 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 180.0), color: white },
        /* 242 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 186.0), color: white },
        /* 243 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 192.0), color: white },
        /* 244 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 198.0), color: white },
        /* 245 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 204.0), color: white },
        /* 246 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 210.0), color: white },
        /* 247 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 216.0), color: white },
        /* 248 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 222.0), color: white },
        /* 249 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 228.0), color: white },
        /* 250 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 234.0), color: white },
        /* 251 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 240.0), color: white },
        /* 252 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 246.0), color: white },
        /* 253 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 252.0), color: white },
        /* 254 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 258.0), color: white },
        /* 255 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 264.0), color: white },
        /* 256 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_IRADIUS, 270.0), color: white },
        // right defense area, outer vertices
        /* 257 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  90.0), color: white },
        /* 258 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS,  96.0), color: white },
        /* 259 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 102.0), color: white },
        /* 260 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 108.0), color: white },
        /* 261 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 114.0), color: white },
        /* 262 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 120.0), color: white },
        /* 263 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 126.0), color: white },
        /* 264 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 132.0), color: white },
        /* 265 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 138.0), color: white },
        /* 266 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 144.0), color: white },
        /* 267 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 150.0), color: white },
        /* 268 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 156.0), color: white },
        /* 269 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 162.0), color: white },
        /* 270 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 168.0), color: white },
        /* 271 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 174.0), color: white },
        /* 272 */ Vertex { position: arc( FIELD_HLENGTH,  DEFENSE_HSTRETCH, DEFENSE_RADIUS, 180.0), color: white },
        /* 273 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 180.0), color: white },
        /* 274 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 186.0), color: white },
        /* 275 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 192.0), color: white },
        /* 276 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 198.0), color: white },
        /* 277 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 204.0), color: white },
        /* 278 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 210.0), color: white },
        /* 279 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 216.0), color: white },
        /* 280 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 222.0), color: white },
        /* 281 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 228.0), color: white },
        /* 282 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 234.0), color: white },
        /* 283 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 240.0), color: white },
        /* 284 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 246.0), color: white },
        /* 285 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 252.0), color: white },
        /* 286 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 258.0), color: white },
        /* 287 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 264.0), color: white },
        /* 288 */ Vertex { position: arc( FIELD_HLENGTH, -DEFENSE_HSTRETCH, DEFENSE_RADIUS, 270.0), color: white },
        // left goal, inner vertices
        /* 289 */ Vertex { position: [-FIELD_HLENGTH, -GOAL_HWIDTH, Z], color: left },
        /* 290 */ Vertex { position: [-(FIELD_HLENGTH + GOAL_DEPTH), -GOAL_HWIDTH, Z], color: left },
        /* 291 */ Vertex { position: [-(FIELD_HLENGTH + GOAL_DEPTH), GOAL_HWIDTH, Z], color: left },
        /* 292 */ Vertex { position: [-FIELD_HLENGTH, GOAL_HWIDTH, Z], color: left },
        // left goal, inner vertices
        /* 293 */ Vertex { position: [-FIELD_HLENGTH, -(GOAL_HWIDTH + GOAL_WALL_WIDTH), Z], color: left },
        /* 294 */ Vertex { position: [-(FIELD_HLENGTH + GOAL_DEPTH + GOAL_WALL_WIDTH), -(GOAL_HWIDTH + GOAL_WALL_WIDTH), Z], color: left },
        /* 295 */ Vertex { position: [-(FIELD_HLENGTH + GOAL_DEPTH + GOAL_WALL_WIDTH), (GOAL_HWIDTH + GOAL_WALL_WIDTH), Z], color: left },
        /* 296 */ Vertex { position: [-FIELD_HLENGTH, (GOAL_HWIDTH + GOAL_WALL_WIDTH), Z], color: left },
        // right goal, inner vertices
        /* 297 */ Vertex { position: [FIELD_HLENGTH, -GOAL_HWIDTH, Z], color: right },
        /* 298 */ Vertex { position: [(FIELD_HLENGTH + GOAL_DEPTH), -GOAL_HWIDTH, Z], color: right },
        /* 299 */ Vertex { position: [(FIELD_HLENGTH + GOAL_DEPTH), GOAL_HWIDTH, Z], color: right },
        /* 300 */ Vertex { position: [FIELD_HLENGTH, GOAL_HWIDTH, Z], color: right },
        // right goal, inner vertices
        /* 301 */ Vertex { position: [FIELD_HLENGTH, -(GOAL_HWIDTH + GOAL_WALL_WIDTH), Z], color: right },
        /* 302 */ Vertex { position: [(FIELD_HLENGTH + GOAL_DEPTH + GOAL_WALL_WIDTH), -(GOAL_HWIDTH + GOAL_WALL_WIDTH), Z], color: right },
        /* 303 */ Vertex { position: [(FIELD_HLENGTH + GOAL_DEPTH + GOAL_WALL_WIDTH), (GOAL_HWIDTH + GOAL_WALL_WIDTH), Z], color: right },
        /* 304 */ Vertex { position: [FIELD_HLENGTH, (GOAL_HWIDTH + GOAL_WALL_WIDTH), Z], color: right },
        // field with margin vertices
        /* 305 */ Vertex { position: [-FIELD_RLENGTH, -FIELD_RWIDTH, Z], color: field },
        /* 306 */ Vertex { position: [ FIELD_RLENGTH, -FIELD_RWIDTH, Z], color: field },
        /* 307 */ Vertex { position: [ FIELD_RLENGTH,  FIELD_RWIDTH, Z], color: field },
        /* 308 */ Vertex { position: [-FIELD_RLENGTH,  FIELD_RWIDTH, Z], color: field },
    ]).unwrap();

    // building the index buffer
    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &[
        // field
        305, 307, 306, 307, 305, 308,
        // field border
        0, 4, 1, 1, 4, 5, // bottom
        1, 5, 2, 2, 5, 6, // right
        2, 6, 3, 3, 6, 7, // top
        3, 7, 0, 0, 7, 4, // left
        // middle line
        8, 9, 10, 10, 8, 11,
        // longitudinal line
        12, 13, 14, 14, 12, 15,
        // center spot
        16, 18, 17, 16, 19, 18, 16, 20, 19, 16, 21, 20, 16, 22, 21, 16, 23, 22,
        16, 24, 23, 16, 25, 24, 16, 26, 25, 16, 27, 26, 16, 28, 27, 16, 29, 28,
        16, 30, 29, 16, 31, 30, 16, 32, 31, 16, 33, 32, 16, 34, 33, 16, 35, 34,
        16, 36, 35, 16, 37, 36, 16, 38, 37, 16, 39, 38, 16, 40, 39, 16, 17, 40,
        // center circle
        41, 102, 101, 102, 41, 42, 42, 103, 102, 103, 42, 43, 43, 104, 103, 104, 43, 44,
        44, 105, 104, 105, 44, 45, 45, 106, 105, 106, 45, 46, 46, 107, 106, 107, 46, 47,
        47, 108, 107, 108, 47, 48, 48, 109, 108, 109, 48, 49, 49, 110, 109, 110, 49, 50,
        50, 111, 110, 111, 50, 51, 51, 112, 111, 112, 51, 52, 52, 113, 112, 113, 52, 53,
        53, 114, 113, 114, 53, 54, 54, 115, 114, 115, 54, 55, 55, 116, 115, 116, 55, 56,
        56, 117, 116, 117, 56, 57, 57, 118, 117, 118, 57, 58, 58, 119, 118, 119, 58, 59,
        59, 120, 119, 120, 59, 60, 60, 121, 120, 121, 60, 61, 61, 122, 121, 122, 61, 62,
        62, 123, 122, 123, 62, 63, 63, 124, 123, 124, 63, 64, 64, 125, 124, 125, 64, 65,
        65, 126, 125, 126, 65, 66, 66, 127, 126, 127, 66, 67, 67, 128, 127, 128, 67, 68,
        68, 129, 128, 129, 68, 69, 69, 130, 129, 130, 69, 70, 70, 131, 130, 131, 70, 71,
        71, 132, 131, 132, 71, 72, 72, 133, 132, 133, 72, 73, 73, 134, 133, 134, 73, 74,
        74, 135, 134, 135, 74, 75, 75, 136, 135, 136, 75, 76, 76, 137, 136, 137, 76, 77,
        77, 138, 137, 138, 77, 78, 78, 139, 138, 139, 78, 79, 79, 140, 139, 140, 79, 80,
        80, 141, 140, 141, 80, 81, 81, 142, 141, 142, 81, 82, 82, 143, 142, 143, 82, 83,
        83, 144, 143, 144, 83, 84, 84, 145, 144, 145, 84, 85, 85, 146, 145, 146, 85, 86,
        86, 147, 146, 147, 86, 87, 87, 148, 147, 148, 87, 88, 88, 149, 148, 149, 88, 89,
        89, 150, 149, 150, 89, 90, 90, 151, 150, 151, 90, 91, 91, 152, 151, 152, 91, 92,
        92, 153, 152, 153, 92, 93, 93, 154, 153, 154, 93, 94, 94, 155, 154, 155, 94, 95,
        95, 156, 155, 156, 95, 96, 96, 157, 156, 157, 96, 97, 97, 158, 157, 158, 97, 98,
        98, 159, 158, 159, 98, 99, 99, 160, 159, 160, 99, 100, 100, 101, 160, 101, 100, 41,
        // left defense area
        161, 194, 193, 194, 161, 162, 162, 195, 194, 195, 162, 163, 163, 196, 195, 196, 163, 164,
        164, 197, 196, 197, 164, 165, 165, 198, 197, 198, 165, 166, 166, 199, 198, 199, 166, 167,
        167, 200, 199, 200, 167, 168, 168, 201, 200, 201, 168, 169, 169, 202, 201, 202, 169, 170,
        170, 203, 202, 203, 170, 171, 171, 204, 203, 204, 171, 172, 172, 205, 204, 205, 172, 173,
        173, 206, 205, 206, 173, 174, 174, 207, 206, 207, 174, 175, 175, 208, 207, 208, 175, 176,
        176, 209, 208, 209, 176, 177, 177, 210, 209, 210, 177, 178, 178, 211, 210, 211, 178, 179,
        179, 212, 211, 212, 179, 180, 180, 213, 212, 213, 180, 181, 181, 214, 213, 214, 181, 182,
        182, 215, 214, 215, 182, 183, 183, 216, 215, 216, 183, 184, 184, 217, 216, 217, 184, 185,
        185, 218, 217, 218, 185, 186, 186, 219, 218, 219, 186, 187, 187, 220, 219, 220, 187, 188,
        188, 221, 220, 221, 188, 189, 189, 222, 221, 222, 189, 190, 190, 223, 222, 223, 190, 191,
        191, 224, 223, 224, 191, 192,
        // right defense area
        225, 258, 257, 258, 225, 226, 226, 259, 258, 259, 226, 227, 227, 260, 259, 260, 227, 228,
        228, 261, 260, 261, 228, 229, 229, 262, 261, 262, 229, 230, 230, 263, 262, 263, 230, 231,
        231, 264, 263, 264, 231, 232, 232, 265, 264, 265, 232, 233, 233, 266, 265, 266, 233, 234,
        234, 267, 266, 267, 234, 235, 235, 268, 267, 268, 235, 236, 236, 269, 268, 269, 236, 237,
        237, 270, 269, 270, 237, 238, 238, 271, 270, 271, 238, 239, 239, 272, 271, 272, 239, 240,
        240, 273, 272, 273, 240, 241, 241, 274, 273, 274, 241, 242, 242, 275, 274, 275, 242, 243,
        243, 276, 275, 276, 243, 244, 244, 277, 276, 277, 244, 245, 245, 278, 277, 278, 245, 246,
        246, 279, 278, 279, 246, 247, 247, 280, 279, 280, 247, 248, 248, 281, 280, 281, 248, 249,
        249, 282, 281, 282, 249, 250, 250, 283, 282, 283, 250, 251, 251, 284, 283, 284, 251, 252,
        252, 285, 284, 285, 252, 253, 253, 286, 285, 286, 253, 254, 254, 287, 286, 287, 254, 255,
        255, 288, 287, 288, 255, 256,
        // left goal
        289, 293, 294, 294, 290, 289,
        290, 294, 295, 295, 291, 290,
        291, 295, 296, 296, 292, 291,
        // right goal
        297, 302, 301, 302, 297, 298,
        298, 303, 302, 303, 298, 299,
        299, 304, 303, 304, 299, 300,
    ]).unwrap();

    (vertex_buffer, index_buffer)
}
