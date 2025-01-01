extends Button

func _ready() -> void:
	var btn = $"."
	btn.pressed.connect(self._goto_level_scene)
	
func _goto_level_scene():
	get_tree().change_scene_to_file("res://scenes/level1.tscn")
