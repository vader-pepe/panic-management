extends Button


func _ready() -> void:
	var btn = $"."
	if btn:
		btn.pressed.connect(self._goto_main_menu)
	
func _goto_main_menu():
	get_tree().change_scene_to_file("res://scenes/main_menu.tscn")
