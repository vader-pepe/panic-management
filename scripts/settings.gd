extends Button


func _ready() -> void:
	var btn = $"."
	btn.pressed.connect(self._goto_settings_menu)

func _goto_settings_menu():
	get_tree().change_scene_to_file("res://scenes/settings_menu.tscn")
