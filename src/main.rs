#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod behaviors;
mod penguin;
mod pid;

use penguin::Penguin;

use rand::prelude::*;

use raylib::consts::MouseButton;
use raylib::prelude::*;

use pid::PidController;
use behaviors::build_randomizer;
use behaviors::Pause;
use behaviors::DirectionChange;
use behaviors::Grab;
use behaviors::Slide;


fn main() {
    let window_size = Vector2::new(200.0, 200.0);
    let waddle_bounds = (Vector2::new(15.0, 15.0), Vector2::new(100.0, 100.0));

    let mut position = Vector2::new(100.0, 100.0);

    let mut target_velocity = Vector2::ZERO;
    let mut velocity = Vector2::ZERO; // pixels/second

    let mut direction_pid = PidController::new(0.1, 0.0, 2.0);
    let mut target_direction = 0.0;
    let mut direction = 0.0;
    let mut angular_velocity = 0.0; // radians/second

    //Behaviors
    let mut grab = Grab::new();

    let mut slide = Slide::new(50.0, 150.0);

    let mut pause = Pause::new();

    let mut direction_change = DirectionChange::new();

    let pause_randomizer = build_randomizer(0.1, 3.0);
    let direction_change_randomizer = build_randomizer(0.3, 3.0);
    let slide_randomizer = build_randomizer(0.5, 5.0);
    

    let mut sprite = Penguin::new(
        Vector2::new(50.0, 50.0),   //sprite_center
        Vector2::new(0.0, 0.0),     //torso_position
        Vector2::new(-23.0, -15.0), //head_position
        Vector2::new(-5.0, 42.0),   //right_foot_position
        Vector2::new(32.0, 32.0),   //left_foot_position
        Vector2::new(-5.0, 37.0),   //right_foot_target
        Vector2::new(32.0, 37.0),   //left_foot_target
        0.06,                       //foot_scale
        0.08,                       //torso_scale
        0.13,                       //head_scale
    );

    let (mut rl, thread) = raylib::init()
        .size(window_size.x as i32, window_size.y as i32)
        .title("panguin")
        .transparent()
        .undecorated()
        .topmost()
        .msaa_4x()
        .vsync()
        .borderless_windowed_mode()
        .build();

    let screen_dimensions = Vector2::new(
        get_monitor_width(get_current_monitor_index()) as f32,
        get_monitor_height(get_current_monitor_index()) as f32,
    );

    sprite.load_textures(&mut rl, &thread, "foot.png", "torso.png", "head.png");

    while !rl.window_should_close() {
        let corrected_direction = correct_angle(direction + PI as f32 / 2.0);        

        pause.update(pause_randomizer(rl.get_time()), rl.get_frame_time());
        direction_change.update(direction_change_randomizer(rl.get_time()));
        target_velocity = from_angle(target_direction).scale(slide.update(slide_randomizer(rl.get_time())));
        
        if pause.running() {
            target_velocity = from_angle(target_direction).scale(0.0);
        }
        if direction_change.running() {
            target_direction = rand::rng().random_range(0.0..2.0 * PI) as f32;
        }

        if let Some(vector) = grab.update(rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT), rl.get_mouse_position()) {
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                position += vector;
            } else {
                velocity = vector;
                angular_velocity = 20.0;
            }
        } else {

            angular_velocity *= 0.99;
            angular_velocity += direction_pid
                .update(from_angle(corrected_direction).angle(from_angle(target_direction)));
            direction = correct_angle(direction + angular_velocity * rl.get_frame_time());

            velocity *= 0.99;
            velocity += from_angle(corrected_direction).scale(
                (target_velocity - velocity)
                    .normalize()
                    .dot(from_angle(corrected_direction))
                    * (target_velocity.length() - velocity.length()).clamp(0.0, 20.0),
            );
            position += velocity.scale(rl.get_frame_time());

            if position.x < -waddle_bounds.0.x {
                if rl.get_time() % 1.0 < 0.01 {
                    target_direction = rand::rng().random_range(3.0 / 2.0 * PI..5.0 / 2.0 * PI) as f32;
                }
                velocity = cancel_normal(velocity, Vector2::new(1.0, 0.0));
            }
            if position.x > screen_dimensions.x - waddle_bounds.1.x {
                if rl.get_time() % 1.0 < 0.01 {
                    target_direction = rand::rng().random_range(1.0 / 2.0 * PI..3.0 / 2.0 * PI) as f32;
                }
                velocity = cancel_normal(velocity, Vector2::new(-1.0, 0.0));
            }
            if position.y < -waddle_bounds.0.y {
                if rl.get_time() % 1.0 < 0.01 {
                    target_direction = rand::rng().random_range(0.0..PI) as f32;
                }
                velocity = cancel_normal(velocity, Vector2::new(0.0, 1.0));
            }
            if position.y > screen_dimensions.y - waddle_bounds.1.y {
                if rl.get_time() % 1.0 < 0.01 {
                    target_direction = rand::rng().random_range(PI..2.0 * PI) as f32;
                }
                velocity = cancel_normal(velocity, Vector2::new(0.0, -1.0));
            }

            sprite.rotate(angular_velocity * rl.get_frame_time());
            if velocity.length() <= 100.0 {
                sprite.move_feet_constrained(velocity.scale(rl.get_frame_time()));
            }
        }

        position.x = position.x
            .clamp(-waddle_bounds.0.x, screen_dimensions.x - waddle_bounds.1.x);
        position.y = position.y
            .clamp(-waddle_bounds.0.y, screen_dimensions.y - waddle_bounds.1.y);

        rl.set_window_position(position.x as i32, position.y as i32);

        {
            // begin drawing
            let mut draw_handle = rl.begin_drawing(&thread);

            draw_handle.clear_background(Color::new(0, 0, 0, 0));
            sprite.draw(&mut draw_handle, direction);
            draw_handle.draw_line_v(
                sprite.get_sprite_center(),
                sprite.get_sprite_center() + velocity,
                Color::RED,
            );
            draw_handle.draw_line_v(
                sprite.get_sprite_center(),
                sprite.get_sprite_center() + target_velocity,
                Color::GREEN,
            );
        }
    }
}

fn rotate_about_point(target: Vector2, center: Vector2, angle: f32) -> Vector2 {
    let mut transformed = target - center;
    transformed = transformed.rotate(angle);
    transformed + center
}

fn from_angle(angle: f32) -> Vector2 {
    Vector2::new(angle.cos(), angle.sin())
}

fn correct_angle(angle: f32) -> f32 {
    if angle > 2.0 * PI as f32 {
        return angle - 2.0 * PI as f32;
    }
    if angle < 0.0 {
        return angle + 2.0 * PI as f32;
    }
    angle
}

fn cancel_normal(vector: Vector2, normal: Vector2) -> Vector2 {
    if vector.dot(normal) < 0.0 {
        vector - normal
            .scale(vector.length() * vector.angle(normal).cos())
    } else {
        vector
    }
}
