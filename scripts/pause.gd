extends Button


func _ready() -> void:
	var btn = $"."
	btn.pressed.connect(self._toggle_pause_scene)
	
func _toggle_pause_scene():
	var pause_state = get_tree().paused
	#get_tree().paused = !pause_state
	print("pause state:", pause_state)
