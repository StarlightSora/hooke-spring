extends Node3D # You should attach this script to a Node3D.

# We should have a Camera3D as a child of this Node3D.
@onready var my_camera = $Camera3D
# We make a new Vector3-based RSHookeSpring.
# Note that matrix types (Quaternion, Transform3D etc) cannot be supported natively.
# Use `Quaternion.from_euler(some_vector3)` to get around this.
var my_spring: RSHookeSpring = RSHookeSpring.new_vector3(0.7, 15.0, RSStopwatch.new_running())
# These are for printing performance metrics.
var total_passed: float = 0.0
var last_printed: float = 0.0
var total_time_usec: int = 0
var total_frames: int = 0

func _ready() -> void:
	pass

func _process(delta: float) -> void:
	var start = Time.get_ticks_usec()
	# You would bind pitch_up to W, pitch_dn to S, yaw_up to D, yaw_down to A,
	# roll_up to E and roll_dn to Q.
	# Go to Project > Project Settings > Input Map to do this.
	# We impulse the spring based on input.
	if Input.is_action_just_pressed("pitch_up"):
		my_spring.impulse(Vector3(TAU/10, 0, 0))
	if Input.is_action_just_pressed("pitch_dn"):
		my_spring.impulse(Vector3(-TAU/10, 0, 0))
	if Input.is_action_just_pressed("yaw_up"):
		my_spring.impulse(Vector3(0, -TAU/10, 0))
	if Input.is_action_just_pressed("yaw_dn"):
		my_spring.impulse(Vector3(0, TAU/10, 0))
	if Input.is_action_just_pressed("roll_up"):
		my_spring.impulse(Vector3(0, 0, -TAU/10))
	if Input.is_action_just_pressed("roll_dn"):
		my_spring.impulse(Vector3(0, 0, TAU/10))
	# Get the position of the spring.
	var qt = my_spring.get_position()
	# We position the camera so that it's at (0.0, 1.5, 0.0), facing -Z.
	# Then we offset the orientation of the camera according to the spring's position. 
	my_camera.transform = Transform3D(
		Basis.IDENTITY, 
		Vector3(0, 1.5, 0),
		# We multiply by 15.0 to accomodate for setting the spring's speed to 15.0.
	) * Transform3D(Quaternion.from_euler(qt*15.0))
	var end = Time.get_ticks_usec()
	total_time_usec += end - start
	total_frames += 1
	last_printed += delta
	total_passed += delta
	# Print out performance metrics every second.
	# Note that most of the computation time is spent *outside* of the spring simulation itself.
	# If you want a closer representation of the actual computation time spent on the springs, see `binding_performance_test.gd`.
	if last_printed > 1.0:
		var fps: float = total_frames/total_passed
		var process_time_usec: float = (total_time_usec as float)/(total_frames as float)
		print(str(floorf(total_passed*1000.0)/1000.0) + "s | " + str(total_frames) + "f -> "\
		 + str(floorf(fps*1000.0)/1000.0) + "FPS (" + str(floorf(1.0/fps*1000.0*1000.0*1000.0)/1000.0) + " usec/f)")
		print("calc time: " + str(floor(process_time_usec*1000.0)/1000.0) + " usec/f -> "\
		+ str(floorf((process_time_usec/1000.0/1000.0)/(1/fps)*100*1000.0)/1000.0) + "% of frame budget")
		last_printed = fmod(last_printed, 1.0)
