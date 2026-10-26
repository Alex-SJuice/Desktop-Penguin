use raylib::prelude::*;

pub struct Penguin {
	sprite_center: Vector2,
    torso_position: Vector2,
    head_position: Vector2,

    right_foot_position: Vector2,
    left_foot_position: Vector2,
    right_foot_target: Vector2,
    left_foot_target: Vector2, 

    foot_scale: f32,
    torso_scale: f32,
    head_scale: f32,

    foot_texture: Option<Texture2D>,
    torso_texture: Option<Texture2D>,
    head_texture: Option<Texture2D>,
}

impl Penguin {
	pub fn new(
			sprite_center: Vector2,
	   	 	torso_position: Vector2,
	    	head_position: Vector2,

	        right_foot_position: Vector2,
	        left_foot_position: Vector2,
	        right_foot_target: Vector2,
	        left_foot_target: Vector2, 

	        foot_scale: f32,
	        torso_scale: f32,
	        head_scale: f32
	        ) -> Penguin {
		Penguin {
			sprite_center,
	        torso_position,
	        head_position,

	        right_foot_position,
	        left_foot_position,
	        right_foot_target,
	        left_foot_target, 

	        foot_scale,
	        torso_scale,
	        head_scale,

	        foot_texture: None,
	        torso_texture: None,
	        head_texture: None,
		}
	}

	pub fn load_textures(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, foot_path: &str, torso_path: &str, head_path: &str) {
		let foot_image = Image::load_image(foot_path).expect("Failed to load image: foot");
	    let torso_image = Image::load_image(torso_path).expect("Failed to load image: torso");
	    let head_image = Image::load_image(head_path).expect("Failed to load image: head");

	    self.foot_texture  = Some(rl.load_texture_from_image(&thread, &foot_image).expect("Failed to load texture: foot"));
		self.torso_texture = Some(rl.load_texture_from_image(&thread, &torso_image).expect("Failed to load texture: torso"));
    	self.head_texture  = Some(rl.load_texture_from_image(&thread, &head_image).expect("Failed to load texture: head"));
	}

	pub fn draw(&self, draw_handle: &mut RaylibDrawHandle, direction: f32) {
		draw_handle.draw_texture_ex(&self.foot_texture.as_ref().expect("Textures weren't loaded!"),  self.left_foot_position, direction * 180.0 / PI as f32, 	self.foot_scale, Color::WHITE);
        draw_handle.draw_texture_ex(&self.foot_texture.as_ref().expect("Textures weren't loaded!"),  self.right_foot_position, direction * 180.0 / PI as f32, self.foot_scale, Color::WHITE);
        draw_handle.draw_texture_ex(&self.torso_texture.as_ref().expect("Textures weren't loaded!"), self.torso_position, direction * 180.0 / PI as f32, 		self.torso_scale,Color::WHITE);
        draw_handle.draw_texture_ex(&self.head_texture.as_ref().expect("Textures weren't loaded!"),  self.head_position, direction * 180.0 / PI as f32, 		self.head_scale, Color::WHITE);
	}

	pub fn rotate(&mut self, angle: f32) {
		self.torso_position = super::rotate_about_point(self.torso_position, self.sprite_center, angle);
        self.head_position = super::rotate_about_point(self.head_position, self.sprite_center, angle);
        self.left_foot_position = super::rotate_about_point(self.left_foot_position, self.sprite_center, angle);
        self.right_foot_position = super::rotate_about_point(self.right_foot_position, self.sprite_center, angle);
        self.left_foot_target = super::rotate_about_point(self.left_foot_target, self.sprite_center, angle);
        self.right_foot_target = super::rotate_about_point(self.right_foot_target, self.sprite_center, angle);
	}

	pub fn move_feet_constrained(&mut self, delta: Vector2) {
		self.left_foot_position -= delta;
        self.right_foot_position -= delta;

        if self.left_foot_position.distance(self.left_foot_target) > 10.0 {
            self.left_foot_position = self.left_foot_target + delta.normalize().multiply(Vector2::new(10.0, 10.0));
        }
        if self.right_foot_position.distance(self.right_foot_target) > 10.0 {
            self.right_foot_position = self.right_foot_target + delta.normalize().multiply(Vector2::new(10.0, 10.0));
        }
	}

	#[allow(unused)]
	pub fn get_sprite_center(&self) -> Vector2 {
		self.sprite_center
	}
}