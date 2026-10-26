extends Node2D

@onready var sprite = $AnimatedSprite2D
@onready var collider = $Area2D

var walk_speed = 100
var idle_trigger = 1.0

var current_speed = walk_speed
var direction = Vector2(1,0)

var idle_timer = 0.0
var current_idle_trigger = idle_trigger
var is_idle = false

var is_dragging = false
var drag_offset = Vector2()
var mouse_position = Vector2()
var prev_mouse_pos = Vector2()
var flying = false

var sprite_size = Vector2(73,48)
var window_offset = Vector2(50,50)
var screen_size = Vector2()

func _ready() -> void:
	screen_size = Vector2(DisplayServer.screen_get_size())
	collider.input_event.connect(_on_area_input)
	
func _physics_process(delta: float) -> void:
	if is_dragging:
		prev_mouse_pos = mouse_position
		mouse_position = Vector2(DisplayServer.mouse_get_position())
		var new_window_position = mouse_position - drag_offset
		DisplayServer.window_set_position(Vector2i(new_window_position))
		return
	if flying:
		current_speed *= 0.95
		if current_speed < 50:
			sprite.frame = 1
			flying = false
			current_speed = walk_speed
			is_idle = true
			idle_timer = 1.0
			
	else:
		if is_idle:
			idle_timer -= delta
			if idle_timer < 0:
				is_idle = false
				current_speed = walk_speed
				random_direction(0.0, 2 * PI)
			return
		else:
			current_idle_trigger -= delta
			if current_idle_trigger < 0:
				current_idle_trigger = idle_trigger
				if randf() < 0.3:
					random_idle()
	var window_position = Vector2(DisplayServer.window_get_position())
	window_position += direction * current_speed * delta + window_offset
	
	if window_position.x <= 0:
		random_direction(3/2.0 * PI, 5/2.0 * PI)
	elif window_position.x >= screen_size.x - sprite_size.x - window_offset.x:
		random_direction(1/2.0 * PI, 3/2.0 * PI)
		
	if window_position.y <= 0:
		random_direction(0.0, PI)
	elif window_position.y >= screen_size.y - sprite_size.y - window_offset.y:
		random_direction(PI, 2 * PI)
	
	#window_position.x = clamp(window_position.x, 0, screen_size.x - sprite_size.x)
	#window_position.y = clamp(window_position.y, 0, screen_size.x - sprite_size.x)
	
	DisplayServer.window_set_position(Vector2i(window_position - window_offset))

func random_idle():
	is_idle = true
	idle_timer = randf_range(1.0, 3.0)
	current_speed = 0
		
func _on_area_input(_viewport, event, _shape_idx):
	if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT:
		if event.pressed:
			is_dragging = true
			mouse_position = Vector2(DisplayServer.mouse_get_position())
			var window_pos = Vector2(DisplayServer.window_get_position())
			drag_offset = mouse_position - window_pos
		else:
			is_dragging = false
			flying = true
			current_speed = mouse_position.distance_to(prev_mouse_pos) * 50
			direction = (mouse_position - prev_mouse_pos).normalized()
			rotation = atan2(direction.y, direction.x)
			sprite.frame = 0
						
func random_direction (low, high):
	var rand = randf_range(low, high)
	direction = Vector2(cos(rand), sin(rand))
	rotation = atan2(direction.y, direction.x)
func reflect_direction (normal):
	direction.reflect(normal)
	rotation = atan2(direction.y, direction.x)
