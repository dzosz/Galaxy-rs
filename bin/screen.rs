use std::io::{self, Write};

const WIDTH : usize = 950;
const HEIGHT : usize= 670;
const dW : usize= 8;
const dH : usize= 8;
static mut termWidth : usize = 80;
static mut termHeight : usize = 24;

pub struct Screen {
    canvas : [[bool; WIDTH]; HEIGHT],
    x : f32,
    y : f32,
    zoom : f32,
    _palette : i32,
}

impl Screen {
    pub fn new(x : f32, y : f32, z : f32) -> Screen {
        let mut obj = Screen { canvas : [[false; WIDTH]; HEIGHT], x : x, y: y, zoom: z, _palette : 0 };
        obj.Setup();
        obj.Clear();
        obj
    }

    pub fn Clear(&mut self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.canvas[i][j] = false;
            }
        }
    }

    pub fn PlotPoint(&mut self, x : f32, y :f32) {
        let mut pos = [0; 2];
        self.transform(&mut pos, x, y);
        self.drawPoint(pos[0] as usize, pos[1] as usize);
    }

    pub fn PlotLine(&mut self, x1 : f32, y1 :f32, x2 : f32, y2 : f32) {
        let mut pos1 = [0; 2];
        let mut pos2 = [0; 2];
        self.transform(&mut pos1, x1, y1);
        self.transform(&mut pos2, x2, y2);
        self.drawLine(pos1[0] as usize, pos1[1] as usize, pos2[0] as usize, pos2[1] as usize);
    }

    pub fn PlotCircle(&mut self, x : f32, y: f32, r: f32) {
        let mut p1 = [0; 2];
        let mut p2 = [0; 2];

        self.transform(&mut p1, x - r, y + r);
        self.transform(&mut p2, x + r, y - r);

        for i in p1[0]..=p2[0] {
            for j in p1[1]..=p2[1] {
                let xt = (j as f32 - WIDTH as f32/2.0) / self.zoom + self.x as f32;
                let yt = (HEIGHT as f32/2.0 - 1.0 - i as f32) / self.zoom + self.y as f32;
                let radius2 = (xt-x) * (xt-x) + (yt - y) * (yt - y);
                if radius2 <= r*r {
                    self.drawPoint(i as usize, j as usize);
                }
            }
        }
    }

    pub fn PlotRectangle(&mut self, x1 : f32, y1 :f32, x2 : f32, y2 : f32) {
        let mut p1 = [0; 2];
        let mut p2 = [0; 2];

        self.transform(&mut p1, x1, y1);
        self.transform(&mut p2, x2, y2);
        self.drawRectangle(p1[0], p1[1], p2[0], p2[1]);
    }

    pub fn Position(&mut self, x : f32, y : f32) {
        self.x = x;
        self.y = y;
    }

    pub fn Zoom(&mut self, zoom : f32) {
        self.zoom = zoom;
    }

    pub fn Draw(&mut self) {
        let mut frame = [['x'; WIDTH / dW + 1]; HEIGHT/dH];
        for i in 0..HEIGHT/dH -1 {
            frame[i][WIDTH/dW] = '\n';
        }
        frame[HEIGHT / dH - 1][WIDTH/dW] = '\0';
        let mut countMax = 0;

        for i in 0..HEIGHT/dH {
            for j in 0..WIDTH/dW {
                let mut count = 0;

                // calculating brightness
                for k in 0..dH {
                    for l in 0 ..dW {
                        count += self.canvas[dH * i + k][dW * j + l] as usize;
                    }
                }

                frame[i][j] = self.brightness(count);
                countMax = std::cmp::max(count, countMax);
            }
        }
        
        // borders
        for i in 0..HEIGHT/dH {
            frame[i][0] = '@';
            frame[i][WIDTH/dW - 1] = '@';
        }
        for j in 0..WIDTH/dW {
            frame[0][j] = '@';
            frame[HEIGHT/dH-1][j] = '@';
        }
        self.FillScreenWithString(&frame);
        //print!("cmax {}",countMax);
        let canvssum = { 
            let mut s = 0;
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    s += self.canvas[i][j] as usize;
                }
            }
            s
        };
        //println!(" canvas {}", canvssum);
    }

    pub fn Height() -> usize {
        HEIGHT
    }

    pub fn Width() -> usize {
        WIDTH
    }

    pub fn set_palette(&mut self, palette : i32) {
        self._palette = palette;
    }

    fn Setup(&mut self) {
        use terminal_size::{Width, Height, terminal_size};
        let size = terminal_size();
        match size {
            Some((Width(w), Height(h))) => unsafe {
                termWidth = w as usize; termHeight = h as usize;
                println!("terminal w:{} h{}", termWidth, termHeight);
            },
            None => panic!("can't get terminal size"),
        }
    }
    fn transform(&mut self, pos : &mut [i64; 2], x : f32, y: f32 ) {
        // from world to screen coordinates
        let xx = ((x - self.x) as f32 * self.zoom) as i64 + (WIDTH as i64 / 2);
        let yy = ((y - self.y) as f32* self.zoom) as i64 + (HEIGHT as i64 /2);

        pos[0] = HEIGHT as i64 -1 - yy as i64;
        pos[1] = xx as i64;
    }
    fn drawPoint(&mut self, A : usize, B: usize ) {
        if A < 0 || B < 0 || A >= HEIGHT || B>= WIDTH {
            return;
        }
        self.canvas[A][B] = true;
    }

    fn drawBoldPoint(&mut self, A : i64, B : i64) {
        for i in A-1..=A+1 {
            for j in B-1..=B+1 {
                self.drawPoint(i as usize, j as usize); // TODO avoid under/overflow
            }
        }
    }

    fn drawLine(&mut self, a : usize, b: usize, c : usize, d : usize) {
        // sorting
        let (mut A,mut B,mut C,mut D) = (a,b,c,d);
        if A > C {
            std::mem::swap(&mut A, &mut C);
            std::mem::swap(&mut B, &mut D);
        }

        // algorithm
        if B == D {
            for i in A..=C {
                self.drawBoldPoint(i as i64, B as i64);
            }
            return;
        }
        if A == C {
            if D < B {
                std::mem::swap(&mut B, &mut D);
            }

            for i in B..=D {
                self.drawBoldPoint(A as i64, i as i64);
            }
            return;
        }
        if (D as i64-B as i64).abs() < (C as i64-A as i64).abs() {
            self.drawLineLow(A,B,C,D);
        } else {
            if B > D { self.drawLineHigh(C,D,A,B); }
            else { self.drawLineHigh(A,B,C,D); }
        }
    }

    fn drawRectangle(&mut self, i1 : i64, j1: i64, i2 : i64, j2 : i64) {
        let minI = std::cmp::min(i1, i2);
        let maxI = std::cmp::max(i1, i2);
        let minJ = std::cmp::min(j1, j2);
        let maxJ = std::cmp::max(j1, j2);

        for i in minI..=maxI {
            for j in minJ..=maxJ {
                self.drawPoint(i as usize, j as usize);
            }
        }
    }

    fn drawLineLow(&mut self, x0 : usize, y0: usize, x1: usize, y1: usize) {
        let dx = x1 as i64 - x0 as i64;
        let (dy, yi) = if y1 >= y0 { (y1 as i64-y0 as i64, 1i64) } else { (y0 as i64-y1 as i64, -1i64) };

        let mut D = 2 * dy - dx;
        let mut y = y0 as i64;

        for x in x0..=x1 {
            self.drawBoldPoint(x as i64, y);
            if D > 0 {
                y += yi;
                D -= 2* dx;
            }
            D += 2 * dy;
        }
    }

    fn drawLineHigh(&mut self, x0 : usize, y0: usize, x1: usize, y1: usize) {
        let dy = y1 as i64 - y0 as i64;
        let (dx, xi) = if x1 >= x0 { (x1 as i64 -x0 as i64, 1i64) } else { (x0 as i64 -x1 as i64, -1i64) };

        let mut D = 2 * dx - dy;
        let mut x = x0 as i64;

        for y in y0..=y1 {
            self.drawBoldPoint(x, y as i64);
            if D > 0 {
                x += xi;
                D -= 2* dy;
            }
            D += 2 * dx;
        }
    }

    fn brightness(&self, count : usize) -> char {
        let p : &'static [(usize, &str); 3]= &[(10, " .,:;oOQ#@"), (10, "     .oO@@"), (3, " .:")];

        if 0 <= self._palette  && self._palette <=2 {
            let ref pal = p[self._palette as usize];
            pal.1.as_bytes()[count * (pal.0 - 1) / dW / dH] as char
        } else {
            ' '
        }
    }
    fn FillScreenWithString(&mut self, frame : &[[char; WIDTH/dW + 1]; HEIGHT/dH]) {
        let mut out = io::stdout();
        let lineheight = unsafe {std::cmp::min(termHeight as usize, HEIGHT/dH) };
        let mut go_to_line_ansi_esacpe_code = String::new();
        let linewidth = unsafe { std::cmp::min(termWidth, WIDTH / dW +1) };
        let mut buf : String;

        for line_idx in 0..lineheight {
            go_to_line_ansi_esacpe_code = format!("{esc}[{};1H", line_idx, esc=27 as char);
            out.write_all(go_to_line_ansi_esacpe_code.as_bytes());
            buf = frame[line_idx].iter().take(linewidth).collect();
            //let slice = std::slice::from_raw_parts(frame[line_idx].as_ptr() as *const u8, linewidth);
            out.write_all(buf.as_bytes());
            //let x = std::str::from_utf8(&frame[line_idx][..]).unwrap();
        }
        out.flush();
    }
}
