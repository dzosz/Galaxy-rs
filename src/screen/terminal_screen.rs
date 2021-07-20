use crate::screen::Screen;

use std::io::{self, Write};

const WIDTH: usize = 950;
const HEIGHT: usize = 670;
const dW: usize = 8;
const dH: usize = 8;

pub struct TerminalScreen {
    canvas: [[bool; WIDTH]; HEIGHT],
    x: f32,
    y: f32,
    zoom: f32,
    _palette: i32,
    terminal: TerminalOutputer
}

struct TerminalOutputer {
    height : usize,
    width : usize,
}

impl TerminalOutputer {
    fn new() -> TerminalOutputer {
        TerminalOutputer { height : 0, width : 0}
    }

    fn setup(&mut self) {
        use terminal_size::{terminal_size, Height, Width};
        let size = terminal_size();
        match size {
            Some((Width(w), Height(h))) => {
                self.width = w as usize;
                self.height = h as usize;
                println!("terminal w:{} h:{}", w, h);
            },
            None => panic!("can't get terminal size"),
        }
    }

    fn write(&self, buf : &[u8]) {
        let mut out = io::stdout();
        let starting_line = 0;
        let go_to_line_ansi_esacpe_code = format!("{esc}[{};1H", starting_line, esc = 27 as char);
        out.write_all(go_to_line_ansi_esacpe_code.as_bytes());
        out.write_all(buf);
        out.flush();
    }
}

struct Point(i32, i32);

impl TerminalScreen {
    pub fn new(x: f32, y: f32, z: f32) -> TerminalScreen {
        let mut obj = TerminalScreen {
            canvas: [[false; WIDTH]; HEIGHT],
            x: x,
            y: y,
            zoom: z,
            _palette: 0,
            terminal: TerminalOutputer::new()
        };
        obj.Setup();
        obj.Clear();
        obj
    }
    fn brightness(&self, count: usize) -> u8 {
        let p: &'static [(usize, &str); 3] = &[(10, " .,:;oOQ#@"), (10, "     .oO@@"), (3, " .:")];

        if 0 <= self._palette && self._palette <= 2 {
            let ref pal = p[self._palette as usize];
            pal.1.as_bytes()[count * (pal.0 - 1) / dW / dH]
        } else {
            ' ' as u8
        }
    }

    fn FillScreenWithString(&mut self, frame: &[u8]) {
        self.terminal.write(frame);
    }

    fn Setup(&mut self) {
        self.terminal.setup();
    }

    fn transform(&mut self, x: f32, y: f32) -> Point {
        // from world to screen coordinates
        let xx = ((x - self.x) * self.zoom) as i32 + (WIDTH as i32 / 2);
        let yy = ((y - self.y) * self.zoom) as i32 + (HEIGHT as i32 / 2);

        let h = HEIGHT as i32 - 1 - yy as i32;
        let w = xx as i32;
        Point(h, w)
    }
    fn drawPoint(&mut self, point: Point) {
        let (x, y) = (point.0, point.1);
        if x < 0 || y < 0 || x >= HEIGHT as i32 || y >= WIDTH as i32 {
            return;
        }
        self.canvas[x as usize][y as usize] = true;
    }

    fn drawBoldPoint(&mut self, point: Point) {
        let (x, y) = (point.0, point.1);
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                self.drawPoint(Point(x, y));
            }
        }
    }

    // Bresenham's line algorithm
    fn drawLine(&mut self, a: Point, b: Point) {
        // sorting
        let mut fromPoint = a;
        let mut toPoint = b;
        if fromPoint.0 > toPoint.0 {
            std::mem::swap(&mut fromPoint, &mut toPoint);
        }

        // algorithm
        if fromPoint.1 == toPoint.1 {
            for i in fromPoint.0..=toPoint.0 {
                self.drawBoldPoint(Point(i, fromPoint.1));
            }
            return;
        }
        if fromPoint.0 == toPoint.0 {
            if toPoint.1 < fromPoint.1 {
                std::mem::swap(&mut fromPoint.1, &mut toPoint.1);
            }

            for i in fromPoint.1..=toPoint.1 {
                self.drawBoldPoint(Point(fromPoint.0, i));
            }
            return;
        }

        let isGradientSoft = (toPoint.1 - fromPoint.1).abs() < (toPoint.0 - fromPoint.0).abs();
        if isGradientSoft {
            if fromPoint.0 > fromPoint.1 {
                self.drawLineLow(toPoint, fromPoint);
            } else {
                self.drawLineLow(fromPoint, toPoint);
            }
        } else {
            if fromPoint.1 > toPoint.1 {
                self.drawLineHigh(toPoint, fromPoint);
            } else {
                self.drawLineHigh(fromPoint, toPoint);
            }
        }
    }

    // Xialin Wu's line algorithm. Anti-aliased // TODO check correctness once pixel rendering is
    // available
    fn drawSmoothLine(&mut self, a: Point, b: Point) {
        let dx = b.0 - a.0;
        let dy = b.1 - a.1;
        let steep = dx.abs() < dy.abs();

        let mut fromPoint = a;
        let mut toPoint = b;

        let p = |x : i32, y : i32| { if steep { Point(y, x) } else { Point(x, y) } };

        if steep {
            std::mem::swap(&mut fromPoint.0, &mut fromPoint.1);
            std::mem::swap(&mut toPoint.0, &mut toPoint.1);
        }
        if toPoint.0 < fromPoint.0 {
            std::mem::swap(&mut fromPoint, &mut toPoint);
        }

        fn _rfpart(num : f32) -> f32 {
            1.0 - _fpart(num)
        }

        fn _fpart(num : f32) -> f32 {
            num - (num as i32 as f32)
        }

        let grad = dy as f32 / dx as f32;
        let mut intery = fromPoint.1 as f32 + _rfpart(fromPoint.0 as f32) * grad;

        let mut draw_endpoint = |point : &Point| -> i32 {
            let (x, y) = (point.0, point.1);
            let xend = x;//.round();
            let yend = y as f32 + grad * (xend - x) as f32;

            //let xgap = _rfpart(x as f32 + 0.5);
            //let alpha = _rfpart(yend) * xgap;
            let px = xend as i32;
            let py = yend as i32;

            self.drawPoint(p(px, py));
            self.drawPoint(p(px, py + 1));

            px
        };

        let xstart = draw_endpoint(&p(fromPoint.0, fromPoint.1)) +1;
        let xend = draw_endpoint(&p(toPoint.0, toPoint.1));

        for x in xstart..xend {
            let y = intery as i32;
            //let alpha = _rfpart(intery);
            self.drawPoint(p(x, y));
            self.drawPoint(p(x, y + 1));
            intery += grad;
        }

    }

    fn drawRectangle(&mut self, fromPoint: Point, toPoint: Point) {
        let minX = std::cmp::min(fromPoint.0, toPoint.0);
        let maxX = std::cmp::max(fromPoint.0, toPoint.0);
        let minY = std::cmp::min(fromPoint.1, toPoint.1);
        let maxY = std::cmp::max(fromPoint.1, toPoint.1);

        for x in minX..=maxX {
            for y in minY..=maxY {
                self.drawPoint(Point(x, y));
            }
        }
    }

    fn drawLineLow(&mut self, fromPoint: Point, toPoint: Point) {
        let (x0, y0, x1, y1) = (fromPoint.0, fromPoint.1, toPoint.0, toPoint.1);
        let dx = x1 - x0;
        let (dy, yi) = if y1 >= y0 {
            (y1 - y0, 1)
        } else {
            (y0 - y1, -1)
        };

        let mut D = 2 * dy - dx;
        let mut y = y0;

        for x in x0..=x1 {
            self.drawBoldPoint(Point(x, y));
            if D > 0 {
                y += yi;
                D -= 2 * dx;
            }
            D += 2 * dy;
        }
    }

    fn drawLineHigh(&mut self, fromPoint: Point, toPoint: Point) {
        let (x0, y0, x1, y1) = (fromPoint.0, fromPoint.1, toPoint.0, toPoint.1);
        let dy = y1 - y0;
        let (dx, xi) = if x1 >= x0 {
            (x1 - x0, 1)
        } else {
            (x0 - x1, -1)
        };

        let mut D = 2 * dx - dy;
        let mut x = x0;

        for y in y0..=y1 {
            self.drawBoldPoint(Point(x, y));
            if D > 0 {
                x += xi;
                D -= 2 * dy;
            }
            D += 2 * dx;
        }
    }
}

impl Screen for TerminalScreen {
    fn Clear(&mut self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.canvas[i][j] = false;
            }
        }
    }

    fn PlotPoint(&mut self, x: f32, y: f32) {
        let point = self.transform(x, y);
        self.drawPoint(point);
    }

    fn PlotLine(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let p1 = self.transform(x1, y1);
        let p2 = self.transform(x2, y2);
        self.drawLine(p1, p2);
    }

    fn PlotCircle(&mut self, x: f32, y: f32, r: f32) {
        let p1 = self.transform(x - r, y + r);
        let p2 = self.transform(x + r, y - r);

        for i in p1.0..=p2.0 {
            for j in p1.1..=p2.1 {
                let xt = (j as f32 - WIDTH as f32 / 2.0) / self.zoom + self.x as f32;
                let yt = (HEIGHT as f32 / 2.0 - 1.0 - i as f32) / self.zoom + self.y as f32;
                let radius2 = (xt - x) * (xt - x) + (yt - y) * (yt - y);
                let isInCircle = radius2 <= r * r;
                if isInCircle {
                    self.drawPoint(Point(i, j));
                }
            }
        }
    }

    fn PlotRectangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let p1 = self.transform(x1, y1);
        let p2 = self.transform(x2, y2);
        self.drawRectangle(p1, p2);
    }

    fn Position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn Zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    fn Draw(&mut self) {
        let W = self.terminal.width;
        let H = self.terminal.height;
        let mut frame = Vec::with_capacity(W*H);
        frame.resize(W*H, ' ' as u8);

        for i in 0..std::cmp::min(self.terminal.height, HEIGHT/dH) {
            for j in 0..std::cmp::min(self.terminal.width, WIDTH/dW) {
                let mut count = 0;
                for k in 0..dH {
                    for l in 0..dW {
                        count += self.canvas[i*dH][j*dW+l] as usize;
                    }
                }
                let idx = i * W + j as usize;
                frame[idx] = self.brightness(count);
            }
        }

        // newlines
        for i in 0..H {
            frame[i*W + W-1] = '\n' as u8;
        }
        // borders vertical
        for i in 0..H {
            frame[i*W] = '@' as u8;
            frame[i*W + W - 1] = '@' as u8;
        }
        // borders horizontal
        for j in 0..W {
            frame[j] = '@' as u8;
            frame[W*(H-1) + j] = '@' as u8;
        }
        frame[W*H-1] = '\0' as u8; // make sure last character will stop the print
        self.FillScreenWithString(&frame[..]);
    }

    fn set_palette(&mut self, palette: i32) {
        self._palette = palette;
    }
}
