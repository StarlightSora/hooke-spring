extends Node

func _ready() -> void:
	## RSHookeSpring ##

	# Constructors #
	# Make a float-based spring.
	var _spring_float = RSHookeSpring.new_float(1.0, 1.0, RSStopwatch.new_running())
	# Make a Vector2-based spring.
	var _spring_vector2 = RSHookeSpring.new_vector2(1.0, 1.0, RSStopwatch.new_running())
	# Make a Vector3-based spring.
	var _spring_vector3 = RSHookeSpring.new_vector3(1.0, 1.0, RSStopwatch.new_running())
	# Make a Vector4-based spring.
	var _spring_vector4 = RSHookeSpring.new_vector4(1.0, 1.0, RSStopwatch.new_running())
	
	var spr = RSHookeSpring.new_vector3(1.0, 1.0, RSStopwatch.new_manual())
	# Getters #
	# Gets the position.
	var _pos = spr.get_position()
	# Gets the velocity.
	var _vel = spr.get_velocity()
	# Gets the position and velocity.
	var _pos_vel = spr.get_position_and_velocity()
	var _pos0 = _pos_vel[0]
	var _vel0 = _pos_vel[1]
	# Gets the target.
	var _target = spr.get_target()
	# Gets the damper.
	var _damper = spr.get_damper()
	# Gets the speed.
	var _speed = spr.get_speed()
	# Gets how long the spring has been simulating for.
	var _elapsed_time = spr.get_elapsed_time()
	# Get a copy of the RSStopwatch inside the spring.
	var _clock = spr.get_clock_copy()
	# Gets the time scale.
	var _time_scale = spr.get_time_scale()
	# Get how long the spring existed according to engine time.
	var _engine_elapsed_time = spr.get_engine_elapsed_time()
	# Get when the spring was created according to engine time.
	var _created_time = spr.get_created_time()
	# Get if the spring is paused or not.
	var _is_paused = spr.is_paused()
	
	# Setters #
	# Increments the velocity.
	spr.impulse(Vector3(-2.0, 3.0, 5.0))
	# Increments the position.
	spr.shift(Vector3(-2.0, 3.0, 5.0))
	# Sets the target of the spring.
	# If do_not_animate is false, this simply sets the target.
	# If do_not_animate is true, it sets the target and position, and resets the velocity.
	spr.set_target(Vector3(-2.0, 3.0, 5.0), false)
	spr.set_target(Vector3(-2.0, 3.0, 5.0), true)
	# Sets the damper.
	spr.set_damper(1.0)
	# Sets the speed.
	spr.set_speed(1.0)
	# Sets the damper and speed.
	spr.set_damper_speed(1.0, 1.0)
	# Sets the position.
	spr.set_position(Vector3(-2.0, 3.0, 5.0))
	# Sets the velocity.
	spr.set_velocity(Vector3(-2.0, 3.0, 5.0))
	# Sets the position and velocity.
	spr.set_position_velocity(Vector3(-2.0, 3.0, 5.0), Vector3(-2.0, 3.0, 5.0))
	# Forcibly advances the elapsed time, with respect to time dilation by `time_scale`.
	spr.time_skip(1.0)
	# Sets the `time_scale`.
	spr.time_dilate(1.0)
	# Forcibly advances the elapsed time, ignoring time dilation by `time_scale`.
	spr.time_skip_raw(1.0)
	# Pauses the spring's internal clock.
	spr.pause()
	# Resumes the spring's internal clock.
	spr.resume()

	## RSStopwatch ##
	# Constructors #
	# Create a stopwatch that is running according to engine time.
	var _sw_running = RSStopwatch.new_running()
	# Create a stopwatch that is paused.
	var _sw_stopped = RSStopwatch.new_manual()
	
	var sw = RSStopwatch.new_manual()
	# Getters #
	# Gets how long the stopwatch has been running for.
	var _elapsed_time0 = sw.evaluate_elapsed()
	# Gets when the stopwatch was created according to engine time.
	var _created_time0 = sw.get_created_time()
	# Gets the `time_scale`.
	var _time_scale0 = sw.get_time_scale()
	# Gets how much engine time passed since creation.
	var _real_elapsed = sw.get_real_elapsed()
	# Gets if the stopwatch is paused or not.
	var _is_paused0 = sw.is_paused()
	
	# Setters #
	# Forcibly advances the elapsed time, with respect to time dilation by `time_scale`.
	sw.time_skip(1.0)
	# Pauses the stopwatch.
	sw.pause()
	# Resumes the stopwatch.
	sw.resume()
	# Sets the `time_scale`.
	sw.time_dilate(1.0)
	# Forcibly advances the elapsed time, ignoring time dilation by `time_scale`.
	sw.time_skip_raw(1.0)
	# Resets the `elapsed_time`.
	sw.reset()

func process() -> void:
	pass
