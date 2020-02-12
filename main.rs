extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::camera::{ArcBall, FirstPerson};
use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point3, Vector3, UnitQuaternion};

mod config;

fn main() {
    let eye = Point3::new(10.0f32, 10.0, 10.0);
    let at = Point3::origin();
    let mut first_person = FirstPerson::new(eye, at);
    let mut arc_ball = ArcBall::new(eye, at);
    let mut use_arc_ball = true;

    let mut window = Window::new("Kiss3d: camera with Cube");
    window.set_light(Light::StickToCamera);
        
    let mut cube      = window.add_cube(1.0, 1.0, 1.0);
        cube.set_color(1.0, 0.0, 0.0);
    
        let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while !window.should_close() {
        // rotate the arc-ball camera.
        let curr_yaw = arc_ball.yaw();
        arc_ball.set_yaw(curr_yaw + 0.05);

        // update the current camera.
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(key, Action::Release, _) => {
                    if key == Key::Numpad1 {
                        use_arc_ball = true
                    } else if key == Key::Numpad2 {
                        use_arc_ball = false
                    }
                }
                _ => {}
            }
        }

        window.draw_line(
            &Point3::origin(),
            &Point3::new(1.0, 0.0, 0.0),
            &Point3::new(1.0, 0.0, 0.0),
        );
        window.draw_line(
            &Point3::origin(),
            &Point3::new(0.0, 1.0, 0.0),
            &Point3::new(0.0, 1.0, 0.0),
        );
        window.draw_line(
            &Point3::origin(),
            &Point3::new(0.0, 0.0, 1.0),
            &Point3::new(0.0, 0.0, 1.0),
        );

        if use_arc_ball {
            window.render_with_camera(&mut arc_ball);
        } else {
            window.render_with_camera(&mut first_person);
        }
    }
}
