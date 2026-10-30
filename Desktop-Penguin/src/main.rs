#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use penguin::Penguin;
use pid::PidController;
use rand::prelude::*;
use raylib::consts::MouseButton;
use raylib::prelude::*;
use behaviors::Pause;
use behaviors::DirectionChange;
use behaviors::build_randomizer;

mod behaviors;
mod penguin;
mod pid;

fn main() {
    let window_size = Vector2::new(200.0, 200.0);
    let waddle_bounds = (Vector2::new(15.0, 15.0), Vector2::new(100.0, 100.0));

    let mut position = Vector2::new(100.0, 100.0);

    let mut target_velocity = Vector2::new(50.0, 0.0);
    let mut velocity = Vector2::ZERO; // pixels/second

    let mut direction_pid = PidController::new(0.1, 0.0, 2.0);
    let mut target_direction = 0.0;
    let mut direction = 0.0;
    let mut angular_velocity = 0.0; // radians/second

    let mut grabbed = false;
    let mut grab_offset = Vector2::ZERO;
    let mut last_moving_mouse_position = Vector2::ZERO;

    let mut sliding = false;

    let pause_randomizer = build_randomizer(0.3);
    let mut pause = Pause::new();

    let direction_change_randomizer = build_randomizer(0.3);
    let mut direction_change = DirectionChange::new();

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
        let mut corrected_direction = direction + PI as f32 / 2.0;
        if corrected_direction > 2.0 * PI as f32 {
            corrected_direction -= 2.0 * PI as f32;
        }
        if corrected_direction < 0.0 {
            corrected_direction += 2.0 * PI as f32;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) && !grabbed {
            grabbed = true;
            grab_offset = rl.get_mouse_position();
        }
        if !rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) && grabbed {
            grabbed = false;
            velocity = (last_moving_mouse_position - grab_offset)
                .normalize()
                .scale(800.0);
            angular_velocity = 20.0;
        }

        if grabbed {
            position += rl.get_mouse_position() - grab_offset;
            if (rl.get_mouse_position() - last_moving_mouse_position).length() > 0.1 {
                last_moving_mouse_position = rl.get_mouse_position();
            }
        } else {
            pause.update(rl.get_frame_time());
            direction_change.update();

            if rl.get_time() % 3.0 < 0.01 && pause.trigger(&pause_randomizer) {
                target_velocity = from_angle(target_direction).scale(0.0);
            }
            if rl.get_time() % 3.0 < 0.01 && direction_change.trigger(&direction_change_randomizer) {
                target_direction = rand::rng().random_range(0.0..2.0 * PI) as f32;
            }
            // update position velocity and direction from angular velocity
            //random direction change
            if !pause.running() {
                if sliding {
                    target_velocity = from_angle(target_direction).scale(150.0);
                } else {
                    target_velocity = from_angle(target_direction).scale(50.0);
                }
            }

            //random pausing
            

            angular_velocity *= 0.99;
            angular_velocity += direction_pid
                .update(from_angle(corrected_direction).angle(from_angle(target_direction)));

            direction += angular_velocity * rl.get_frame_time();
            if direction > 2.0 * PI as f32 {
                direction -= 2.0 * PI as f32;
            }
            if direction < 0.0 {
                direction += 2.0 * PI as f32;
            }

            velocity *= 0.99;
            velocity += from_angle(corrected_direction).scale(
                (target_velocity - velocity)
                    .normalize()
                    .dot(from_angle(corrected_direction))
                    * (target_velocity.length() - velocity.length()).clamp(0.0, 20.0),
            );

            position += velocity.scale(rl.get_frame_time());
            if position.x < -waddle_bounds.0.x {
                target_direction = rand::rng().random_range(0.0..2.0 * PI) as f32;
                if velocity.dot(Vector2::new(1.0, 0.0)) < 0.0 {
                    velocity -= Vector2::new(1.0, 0.0)
                        .scale(velocity.length() * velocity.angle(Vector2::new(1.0, 0.0)).cos());
                }
            }
            if position.x > screen_dimensions.x - waddle_bounds.1.x {
                target_direction = rand::rng().random_range(0.0..2.0 * PI) as f32;
                if velocity.dot(Vector2::new(-1.0, 0.0)) < 0.0 {
                    velocity -= Vector2::new(-1.0, 0.0)
                        .scale(velocity.length() * velocity.angle(Vector2::new(-1.0, 0.0)).cos());
                }
            }
            if position.y < -waddle_bounds.0.y {
                target_direction = rand::rng().random_range(0.0..2.0 * PI) as f32;
                if velocity.dot(Vector2::new(0.0, 1.0)) < 0.0 {
                    velocity -= Vector2::new(0.0, 1.0)
                        .scale(velocity.length() * velocity.angle(Vector2::new(0.0, 1.0)).cos());
                }
            }
            if position.y > screen_dimensions.y - waddle_bounds.1.y {
                target_direction = rand::rng().random_range(0.0..2.0 * PI) as f32;
                if velocity.dot(Vector2::new(0.0, -1.0)) < 0.0 {
                    velocity -= Vector2::new(0.0, -1.0)
                        .scale(velocity.length() * velocity.angle(Vector2::new(0.0, -1.0)).cos());
                }
            }
            position.x = position
                .x
                .clamp(-waddle_bounds.0.x, screen_dimensions.x - waddle_bounds.1.x);
            position.y = position
                .y
                .clamp(-waddle_bounds.0.y, screen_dimensions.y - waddle_bounds.1.y);

            sprite.rotate(angular_velocity * rl.get_frame_time());
            if velocity.length() <= 100.0 {
                sprite.move_feet_constrained(velocity.scale(rl.get_frame_time()));
            }
        }

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
