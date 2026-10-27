#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use raylib::consts::MouseButton;
use raylib::prelude::*;
use penguin::Penguin;
use pid::pid_controller;

mod penguin;
mod pid;

fn main() {
    let window_size = Vector2::new(100.0, 100.0);

    let mut position = Vector2::new(100.0, 100.0);

    let mut target_velocity = Vector2::new(50.0, 0.0);
    let mut velocity = Vector2::ZERO; // pixels/second

    let mut direction_pid = pid_controller::new(0.1, 0.0, 2.0);
    let mut target_direction = 0.0;
    let mut direction = 0.0;
    let mut angular_velocity = 0.0; // radians/second

    let mut grabbed = false;
    let mut grab_offset = Vector2::ZERO;

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
            velocity += (rl.get_mouse_position() - grab_offset)
                .normalize()
                .scale(800.0);
            angular_velocity += 30.0;
        }

        if !grabbed { // update position velocity and direction from angular velocity
            target_direction = (rl.get_time() / 20.0).sin() as f32 * 2.0 * PI as f32;
            target_velocity = from_angle(target_direction).scale(50.0);

            angular_velocity *= 0.99;
            angular_velocity += direction_pid.update(from_angle(corrected_direction).angle(from_angle(target_direction)));

            direction += angular_velocity * rl.get_frame_time();
            if direction > 2.0 * PI as f32 {
                direction -= 2.0 * PI as f32;
            }
            if direction < 0.0 {
                direction += 2.0 * PI as f32;
            }

            velocity *= 0.99;
            velocity += 
                from_angle(corrected_direction)
                .scale( (target_velocity - velocity).normalize()
                        .dot(from_angle(corrected_direction))
                        * (target_velocity.length() - velocity.length()).clamp(0.0, 20.0));

            position += velocity.scale(rl.get_frame_time());
            position.x = position.x.clamp(0.0, screen_dimensions.x - window_size.x);
            position.y = position.y.clamp(0.0, screen_dimensions.y - window_size.y);

            sprite.rotate(angular_velocity * rl.get_frame_time());
            sprite.move_feet_constrained(velocity.scale(rl.get_frame_time()));
        } else {
            position += rl.get_mouse_position() - grab_offset;
        }

        rl.set_window_position(position.x as i32, position.y as i32);

        {
            // begin drawing
            let mut draw_handle = rl.begin_drawing(&thread);

            draw_handle.clear_background(Color::new(0, 0, 0, 0));
            sprite.draw(&mut draw_handle, direction);
            draw_handle.draw_line_v(sprite.get_sprite_center(), sprite.get_sprite_center() + velocity, Color::RED);
            draw_handle.draw_line_v(sprite.get_sprite_center(), sprite.get_sprite_center() + target_velocity, Color::GREEN);
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
