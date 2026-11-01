use crate::output::Output;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutKind {
    Dwindle,
    Master,
}

impl LayoutKind {
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "master" => Self::Master,
            _ => Self::Dwindle,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

pub fn tile(kind: LayoutKind, output: &Output, count: usize) -> Vec<Rect> {
    if count == 0 {
        return Vec::new();
    }
    match kind {
        LayoutKind::Dwindle => dwindle(output, count),
        LayoutKind::Master => master(output, count),
    }
}

fn dwindle(output: &Output, count: usize) -> Vec<Rect> {
    let mut rects = Vec::with_capacity(count);
    let mut x = 0i32;
    let mut y = 0i32;
    let mut w = output.width;
    let mut h = output.height;
    for _ in 0..count {
        if w >= h {
            let half = w / 2;
            rects.push(Rect { x, y, w: half, h });
            x += half as i32;
            w -= half;
        } else {
            let half = h / 2;
            rects.push(Rect { x, y, w, h: half });
            y += half as i32;
            h -= half;
        }
    }
    rects
}

fn master(output: &Output, count: usize) -> Vec<Rect> {
    let mut rects = Vec::with_capacity(count);
    if count == 1 {
        rects.push(Rect { x: 0, y: 0, w: output.width, h: output.height });
        return rects;
    }
    let master_w = output.width / 2;
    let stack_w = output.width - master_w;
    let stack_h = output.height / (count as u32 - 1);
    rects.push(Rect { x: 0, y: 0, w: master_w, h: output.height });
    let mut y = 0i32;
    for _ in 1..count {
        rects.push(Rect { x: master_w as i32, y, w: stack_w, h: stack_h });
        y += stack_h as i32;
    }
    rects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dwindle_returns_requested_count() {
        let out = Output::new("test", 1920, 1080);
        let r = tile(LayoutKind::Dwindle, &out, 4);
        assert_eq!(r.len(), 4);
    }

    #[test]
    fn master_returns_requested_count() {
        let out = Output::new("test", 1920, 1080);
        let r = tile(LayoutKind::Master, &out, 3);
        assert_eq!(r.len(), 3);
    }

    #[test]
    fn empty_layout_is_empty() {
        let out = Output::new("test", 1920, 1080);
        assert!(tile(LayoutKind::Dwindle, &out, 0).is_empty());
    }
}
