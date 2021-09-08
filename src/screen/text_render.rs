use crate::screen::{Point, TextOutputter, Screen};
use std::io::{self, Write};

const WIDTH: usize = 950;
const HEIGHT: usize = 670;

struct TerminalOutputer {
    height: usize,
    width: usize,
    frame: Vec<u8>,
}

impl TextOutputter for TerminalOutputer {
    fn setup(&mut self) {
        use terminal_size::{terminal_size, Height, Width};
        let size = terminal_size();
        match size {
            Some((Width(w), Height(h))) => {
                self.width = w as usize;
                self.height = h as usize;
                println!("terminal w:{} h:{}", w, h);
                self.frame.resize(self.width * self.height, ' ' as u8);
            }
            None => panic!("can't get terminal size"),
        }
    }

    fn write(&mut self, buf: &[u8]) {
        let mut out = io::stdout();
        let starting_line = 0;
        let go_to_line_ansi_esacpe_code = format!("{esc}[{};1H", starting_line, esc = 27 as char);
        out.write_all(go_to_line_ansi_esacpe_code.as_bytes())
            .expect("failed to write ansi code");
        out.write_all(buf).expect("failed to write buf");
        out.flush().expect("failed to flush");
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

impl TerminalOutputer {
    fn new() -> TerminalOutputer {
        TerminalOutputer {
            height: 0,
            width: 0,
            frame: Vec::new(),
        }
    }
}

pub struct Zoom(pub f64);

pub struct TextRender {
    canvas: [[bool; WIDTH]; HEIGHT],
    x: f64,
    y: f64,
    zoom: f64,
    _palette: i32,
    output: Box<dyn TextOutputter>,
    frame: Vec<u8>,
}

impl TextRender {
    pub fn new(z: Zoom) -> TextRender {
        let mut obj = TextRender {
            canvas: [[false; WIDTH]; HEIGHT],
            x: 0.0,
            y: 0.0,
            zoom: z.0,
            _palette: 0,
            output: Box::new(TerminalOutputer::new()),
            frame: Vec::new(),
        };
        obj.setup();
        obj.clear();
        obj
    }
    fn brightness(&self, count: usize, max: usize) -> u8 {
        let p: &'static [(usize, &str); 3] = &[(10, " .,:;oOQ#@"), (10, "     .oO@@"), (3, " .:")];

        if 0 <= self._palette && self._palette <= 2 {
            let ref pal = p[self._palette as usize];
            pal.1.as_bytes()[count * (pal.0 - 1) / max]
        } else {
            ' ' as u8
        }
    }

    fn setup(&mut self) {
        self.output.setup();
    }

    fn transform(&self, x: f64, y: f64) -> Point {
        // from world to screen coordinates
        let xx = ((x - self.x) * self.zoom) as i32 + (WIDTH as i32 / 2);
        let yy = ((y - self.y) * self.zoom) as i32 + (HEIGHT as i32 / 2);

        let h = HEIGHT as i32 - 1 - yy as i32;
        let w = xx as i32;
        Point(h, w)
    }
    fn draw_point(&mut self, point: Point) {
        let (x, y) = (point.0, point.1);
        if x < 0 || y < 0 || x >= HEIGHT as i32 || y >= WIDTH as i32 {
            return;
        }
        self.canvas[x as usize][y as usize] = true;
    }

    fn draw_bold_point(&mut self, point: Point) {
        let (x, y) = (point.0, point.1);
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                self.draw_point(Point(i, j));
            }
        }
    }

    // Bresenham's line algorithm
    fn draw_line(&mut self, a: Point, b: Point) {
        // sorting
        let mut from_point = a;
        let mut to_point = b;
        if from_point.0 > to_point.0 {
            std::mem::swap(&mut from_point, &mut to_point);
        }

        // algorithm
        if from_point.1 == to_point.1 {
            for i in from_point.0..=to_point.0 {
                self.draw_bold_point(Point(i, from_point.1));
            }
            return;
        }
        if from_point.0 == to_point.0 {
            if to_point.1 < from_point.1 {
                std::mem::swap(&mut from_point.1, &mut to_point.1);
            }

            for i in from_point.1..=to_point.1 {
                self.draw_bold_point(Point(from_point.0, i));
            }
            return;
        }

        let is_gradient_soft = (to_point.1 - from_point.1).abs() < (to_point.0 - from_point.0).abs();
        if is_gradient_soft {
            if from_point.0 > from_point.1 {
                self.draw_line_low(to_point, from_point);
            } else {
                self.draw_line_low(from_point, to_point);
            }
        } else {
            if from_point.1 > to_point.1 {
                self.draw_line_high(to_point, from_point);
            } else {
                self.draw_line_high(from_point, to_point);
            }
        }
    }

    // Xialin Wu's line algorithm. Anti-aliased // TODO check correctness once pixel rendering is
    // available
    fn draw_smooth_line(&mut self, a: Point, b: Point) {
        let dx = b.0 - a.0;
        let dy = b.1 - a.1;
        let steep = dx.abs() < dy.abs();

        let mut from_point = a;
        let mut to_point = b;

        let p = |x: i32, y: i32| {
            if steep {
                Point(y, x)
            } else {
                Point(x, y)
            }
        };

        if steep {
            std::mem::swap(&mut from_point.0, &mut from_point.1);
            std::mem::swap(&mut to_point.0, &mut to_point.1);
        }
        if to_point.0 < from_point.0 {
            std::mem::swap(&mut from_point, &mut to_point);
        }

        fn _rfpart(num: f64) -> f64 {
            1.0 - _fpart(num)
        }

        fn _fpart(num: f64) -> f64 {
            num - (num as i32 as f64)
        }

        let grad = dy as f64 / dx as f64;
        let mut intery = from_point.1 as f64 + _rfpart(from_point.0 as f64) * grad;

        let mut draw_endpoint = |point: &Point| -> i32 {
            let (x, y) = (point.0, point.1);
            let xend = x; //.round();
            let yend = y as f64 + grad * (xend - x) as f64;

            //let xgap = _rfpart(x as f64 + 0.5);
            //let alpha = _rfpart(yend) * xgap;
            let px = xend as i32;
            let py = yend as i32;

            self.draw_point(p(px, py));
            self.draw_point(p(px, py + 1));

            px
        };

        let xstart = draw_endpoint(&p(from_point.0, from_point.1)) + 1;
        let xend = draw_endpoint(&p(to_point.0, to_point.1));

        for x in xstart..xend {
            let y = intery as i32;
            //let alpha = _rfpart(intery);
            self.draw_point(p(x, y));
            self.draw_point(p(x, y + 1));
            intery += grad;
        }
    }

    fn draw_rectangle(&mut self, from_point: Point, to_point: Point) {
        let minX = std::cmp::min(from_point.0, to_point.0);
        let maxX = std::cmp::max(from_point.0, to_point.0);
        let minY = std::cmp::min(from_point.1, to_point.1);
        let maxY = std::cmp::max(from_point.1, to_point.1);

        for x in minX..=maxX {
            for y in minY..=maxY {
                self.draw_point(Point(x, y));
            }
        }
    }

    fn draw_line_low(&mut self, from_point: Point, to_point: Point) {
        let (x0, y0, x1, y1) = (from_point.0, from_point.1, to_point.0, to_point.1);
        let dx = x1 - x0;
        let (dy, yi) = if y1 >= y0 {
            (y1 - y0, 1)
        } else {
            (y0 - y1, -1)
        };

        let mut D = 2 * dy - dx;
        let mut y = y0;

        for x in x0..=x1 {
            self.draw_bold_point(Point(x, y));
            if D > 0 {
                y += yi;
                D -= 2 * dx;
            }
            D += 2 * dy;
        }
    }

    fn draw_line_high(&mut self, from_point: Point, to_point: Point) {
        let (x0, y0, x1, y1) = (from_point.0, from_point.1, to_point.0, to_point.1);
        let dy = y1 - y0;
        let (dx, xi) = if x1 >= x0 {
            (x1 - x0, 1)
        } else {
            (x0 - x1, -1)
        };

        let mut D = 2 * dx - dy;
        let mut x = x0;

        for y in y0..=y1 {
            self.draw_bold_point(Point(x, y));
            if D > 0 {
                x += xi;
                D -= 2 * dy;
            }
            D += 2 * dx;
        }
    }
}

impl Screen for TextRender {
    fn clear(&mut self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.canvas[i][j] = false;
            }
        }
    }

    fn plot_point(&mut self, x: f64, y: f64) {
        let point = self.transform(x, y);
        self.draw_point(point);
    }

    fn plot_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let p1 = self.transform(x1, y1);
        let p2 = self.transform(x2, y2);
        self.draw_line(p1, p2);
    }

    fn plot_circle(&mut self, x: f64, y: f64, r: f64) {
        let p1 = self.transform(x - r, y + r);
        let p2 = self.transform(x + r, y - r);

        for i in p1.0..=p2.0 {
            for j in p1.1..=p2.1 {
                let xt = (j as f64 - WIDTH as f64 / 2.0) / self.zoom + self.x as f64;
                let yt = (HEIGHT as f64 / 2.0 - 1.0 - i as f64) / self.zoom + self.y as f64;
                let radius2 = (xt - x) * (xt - x) + (yt - y) * (yt - y);
                let is_in_circle = radius2 <= r * r;
                if is_in_circle {
                    self.draw_point(Point(i, j));
                }
            }
        }
    }

    fn plot_rectangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let p1 = self.transform(x1, y1);
        let p2 = self.transform(x2, y2);
        self.draw_rectangle(p1, p2);
    }

    fn position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom;
    }

    fn draw(&mut self) {
        let W = self.output.width();
        let H = self.output.height();
        self.frame.resize(W * H, ' ' as u8);

        let compressed_width = WIDTH / W;
        let compressed_height = HEIGHT / H;

        for i in 0..std::cmp::min(H, HEIGHT / compressed_height) {
            for j in 0..std::cmp::min(W, WIDTH / compressed_width) {
                let mut count = 0;
                for k in 0..compressed_height {
                    for l in 0..compressed_width {
                        count +=
                            self.canvas[i * compressed_height + k][j * compressed_width + l] as usize;
                    }
                }
                let idx = i * W + j as usize;
                self.frame[idx] = self.brightness(count, compressed_height * compressed_width);
            }
        }

        // newlines
        for i in 0..H {
            self.frame[i * W + W - 1] = '\n' as u8;
        }
        // borders horizontal
        for j in 0..W {
            self.frame[j] = '#' as u8;
            self.frame[W * (H - 1) + j] = '#' as u8;
        }
        // borders vertical
        for i in 0..H {
            self.frame[i * W] = '#' as u8;
            self.frame[i * W + W - 1] = '\n' as u8;
        }
        self.frame[W * H - 1] = '\0' as u8; // make sure last character will stop the print
        self.output.write(&self.frame);
    }

    fn set_palette(&mut self, palette: i32) {
        self._palette = palette;
    }
}
