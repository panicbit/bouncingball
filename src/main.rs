#![feature(std_misc)]
extern crate rand;
extern crate rustbox;
extern crate time;
use std::process::exit;
use time::Duration;
use rand::Rng;
use rustbox::Event::KeyEvent;
use rustbox::Key::{Esc,Char};
use rustbox::{RustBox,Color,RB_NORMAL};
use rustbox::Color::*;

struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64
}

struct Ball {
    x_pos: f64,
    y_pos: f64,
    x_velocity: f64,
    y_velocity: f64
}

impl Ball {
    fn new(x_pos: f64, y_pos: f64, x_velocity: f64, y_velocity: f64) -> Ball {
        Ball {
            x_pos: x_pos,
            y_pos: y_pos,
            x_velocity: x_velocity,
            y_velocity: y_velocity
        }
    }

    fn move_in_bounds(&mut self, bounds: &Rect) {
        let (x_oob, y_oob) = (self.x_will_be_oob(bounds), self.y_will_be_oob(bounds));
        self.x_pos = Ball::new_pos(self.x_pos - bounds.x, self.x_velocity, bounds.w - bounds.x) + bounds.x;
        self.y_pos = Ball::new_pos(self.y_pos - bounds.y, self.y_velocity, bounds.h - bounds.y) + bounds.y;
        if x_oob { self.x_velocity *= -1.0 }
        if y_oob { self.y_velocity *= -1.0 }
    }

    fn new_pos(x: f64, xv: f64, w: f64) -> f64 {
        if w == 0.0 { return 0.0 }
        if x + xv < 0.0 { Ball::new_pos(x, -xv-2.0*x, w) }
        else if x + xv > w { Ball::new_pos(x, 2.0*w-2.0*x-xv, w) }
        else { x + xv }
    }

    fn x_will_be_oob(&self, bounds: &Rect) -> bool {
        let newpos = self.x_pos + self.x_velocity;
        newpos < bounds.x || bounds.w < newpos
    }

    fn y_will_be_oob(&self, bounds: &Rect) -> bool {
        let newpos = self.y_pos + self.y_velocity;
        newpos < bounds.y || bounds.h < newpos
    }

    pub fn x(&self) -> f64 { self.x_pos }
    pub fn y(&self) -> f64 { self.y_pos }
}

fn main() {
    let rustbox = RustBox::init(Default::default()).ok().expect("rustbox init");

    let mut ball = Ball::new(2.0, 2.0, 1.0, 1.0);

    loop {
        rustbox.clear();

        match rustbox.peek_event(Duration::milliseconds(100), false) {
            Ok(KeyEvent(Some(Esc))) => {drop(rustbox); exit(0)}
            _ => {}
        }

        let (width, height) = (rustbox.width(), rustbox.height());

        let bounds = Rect {
            x: 1.0,
            y: 1.0,
            w: (width-2) as f64,
            h: (height-2) as f64
        };
        
        for x in ((bounds.x-1.0) as usize..(bounds.w+2.0) as usize) {
            rustbox.print_char(x, 0, RB_NORMAL, White, Black, '#');
            rustbox.print_char(x, height-1, RB_NORMAL, White, Black, '#');
        }
        for y in ((bounds.y-1.0) as usize..(bounds.h+2.0) as usize) {
            rustbox.print_char(0, y, RB_NORMAL, White, Black, '#');
            rustbox.print_char(width-1, y, RB_NORMAL, White, Black, '#');
        }

        ball.move_in_bounds(&bounds);

        let (x,y) = (ball.x().round() as usize, ball.y().round() as usize);

        rustbox.print_char(x, y, RB_NORMAL, White, Black, 'o');

        rustbox.present();
    }
}
