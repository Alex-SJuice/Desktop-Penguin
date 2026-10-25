use raylib::prelude::*;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
use windows::Win32::Foundation::POINT;

fn main() {
    let mut position = Vector2::new(100.0, 100.0);
    let mut velocity = Vector2::new(0.0, 0.0); // pixels/second
    let mut angular_velocity = 0.0;
    let mut direction = 0.0;

    let sprite_center = Vector2::new(50.0, 50.0);
    let mut torso_position = Vector2::new(0.0, 0.0);
    let mut head_position = Vector2::new(-23.0, -15.0);
    let mut right_foot_position = Vector2::new(-5.0, 45.0);
    let mut left_foot_position = Vector2::new(32.0, 35.0);

    let mut right_foot_target = Vector2::new(-5.0, 40.0);
    let mut left_foot_target = Vector2::new(32.0, 40.0);  


    let (mut rl, thread) = raylib::init()
        .size(100,100)
        .title("pet")
        .transparent()
        .undecorated()
        .topmost()
        //.mouse_passthrough()
        .borderless_windowed_mode()
        .build();

    let screen_dimensions = 
        Vector2::new(
            get_monitor_width(get_current_monitor_index()) as f32, 
            get_monitor_height(get_current_monitor_index()) as f32
        );

    let mut foot_image = Image::load_image("foot.png").unwrap();
    let mut torso_image = Image::load_image("torso.png").unwrap();
    let mut head_image = Image::load_image("head.png").unwrap();

    let foot_texture = rl.load_texture_from_image(&thread, &foot_image).unwrap();
    let torso_texture = rl.load_texture_from_image(&thread, &torso_image).unwrap();
    let head_texture = rl.load_texture_from_image(&thread, &head_image).unwrap();

    while !rl.window_should_close() {
        let mut mouse_position = POINT{ x:0, y:0 };
        unsafe {
            GetCursorPos(&mut mouse_position).unwrap();
        }

        let mouse_position = Vector2::new(mouse_position.x as f32, mouse_position.y as f32);

        angular_velocity = 
            from_angle(direction + PI as f32 / 2.0).angle(mouse_position - position - sprite_center) * 100.0;

        direction += angular_velocity * rl.get_frame_time();
        if direction > 2.0 * PI as f32 {
            direction -= 2.0 * PI as f32;
        }
        if direction < 0.0 {
            direction += 2.0 * PI as f32;
        }

        velocity = from_angle(direction + PI as f32 / 2.0).scale(50.0);

        position += velocity.scale(rl.get_frame_time());
        position.x = position.x.clamp(0.0, screen_dimensions.x - 100.0);
        position.y = position.y.clamp(0.0, screen_dimensions.y - 100.0);

        torso_position = rotate_about_point(torso_position, sprite_center, angular_velocity * rl.get_frame_time());
        head_position = rotate_about_point(head_position, sprite_center, angular_velocity * rl.get_frame_time());
        left_foot_position = rotate_about_point(left_foot_position, sprite_center, angular_velocity * rl.get_frame_time());
        right_foot_position = rotate_about_point(right_foot_position, sprite_center, angular_velocity * rl.get_frame_time());
        left_foot_target = rotate_about_point(left_foot_target, sprite_center, angular_velocity * rl.get_frame_time());
        right_foot_target = rotate_about_point(right_foot_target, sprite_center, angular_velocity * rl.get_frame_time());
        
       

        left_foot_position -= velocity.scale(rl.get_frame_time());
        right_foot_position -= velocity.scale(rl.get_frame_time());

        if left_foot_position.distance(left_foot_target) > 10.0 {
            left_foot_position = left_foot_target + velocity.normalize().multiply(Vector2::new(10.0, 10.0));
        }
        if right_foot_position.distance(right_foot_target) > 10.0 {
            right_foot_position = right_foot_target + velocity.normalize().multiply(Vector2::new(10.0, 10.0));
        }
        
        rl.set_window_position(position.x as i32, position.y as i32);

        {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::new(0,0,0,0));
            d.draw_texture_ex(&foot_texture, left_foot_position, direction * 180.0 / PI as f32, 0.06, Color::WHITE);
            d.draw_texture_ex(&foot_texture, right_foot_position, direction * 180.0 / PI as f32, 0.06, Color::WHITE);
            d.draw_texture_ex(&torso_texture, torso_position, direction * 180.0 / PI as f32, 0.08, Color::WHITE);
            d.draw_texture_ex(&head_texture, head_position, direction * 180.0 / PI as f32, 0.13, Color::WHITE);
            
        }
    }
}

fn rotate_about_point(target: Vector2, center: Vector2, angle: f32) -> Vector2 {
    let mut transformed  = target - center;
    transformed = transformed.rotate(angle);
    transformed + center
}

fn from_angle(angle: f32) -> Vector2 {
    Vector2::new(angle.cos(), angle.sin())
}
