extends Button

func _ready():
	var button = $"."
	button.pressed.connect(self._button_pressed)

func _button_pressed():
	get_tree().quit()
