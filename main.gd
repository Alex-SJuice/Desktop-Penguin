extends Node2D

@onready var sprite = $Sprite2D
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

var sprite_size = Vector2(64,64)
var screen_size = Vector2()

func _ready() -> void:
	screen_size = Vector2(DisplayServer.screen_get_size())
	collider.input_event.connect(_on_area_input)
	
func _physics_process(delta: float) -> void:
	if is_dragging:
		var mouse_position = Vector2(DisplayServer.mouse_get_position())
		var new_window_position = mouse_position - drag_offset
		DisplayServer.window_set_position(Vector2i(new_window_position))
		return		
	if is_idle:
		idle_timer -= delta
		if idle_timer < 0:
			is_idle = false
			current_speed = walk_speed
			var rand = randf_range(0.0, 2 * PI)
			direction = Vector2(cos(rand), sin(rand))
		return
	else:
		current_idle_trigger -= delta
		if current_idle_trigger < 0:
			current_idle_trigger = idle_trigger
			random_idle()
	var window_position = Vector2(DisplayServer.window_get_position())
	window_position += direction * current_speed * delta
	
	if window_position.x <= 0:
		var rand = randf_range(3/2.0 * PI, 5/2.0 * PI)
		direction = Vector2(cos(rand), sin(rand))
	elif window_position.x >= screen_size.x - sprite_size.x:
		var rand = randf_range(1/2.0 * PI, 3/2.0 * PI)
		direction = Vector2(cos(rand), sin(rand))
		
	if window_position.y <= 0:
		var rand = randf_range(0.0, PI)
		direction = Vector2(cos(rand), sin(rand))
	elif window_position.y >= screen_size.y - sprite_size.y:
		var rand = randf_range(PI, 2 * PI)
		direction = Vector2(cos(rand), sin(rand))
	
	window_position.x = clamp(window_position.x, 0, screen_size.x - sprite_size.x)
	window_position.y = clamp(window_position.y, 0, screen_size.x - sprite_size.x)

	DisplayServer.window_set_position(Vector2i(window_position))

func random_idle():
	if randf() < 0.3:
		is_idle = true
		idle_timer = randf_range(1.0, 3.0)
		current_speed = 0
		
func _on_area_input(_viewport, event, _shape_idx):
	if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT:
		if event.pressed:
			is_dragging = true
			var mouse_pos = Vector2(DisplayServer.mouse_get_position())
			var window_pos = Vector2(DisplayServer.window_get_position())
			drag_offset = mouse_pos - window_pos
		else:
			is_dragging = false
			var rand = randf_range(0.0, 2 * PI)
			direction = Vector2(cos(rand), sin(rand))
