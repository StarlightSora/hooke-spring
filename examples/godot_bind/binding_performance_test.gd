extends Node2D # You should attach this script to a Node2D.

# How many springs to simulate.
const SPRINGS_TO_SIMULATE: int = 100 # In this example we will simulate 100 springs at once.
# The FPS of the simulation.
const SIMULATION_FPS: float = 240.0 # In this example our target FPS is 240.
# How many frames to simulate.
const SIMULATION_STEPS: int = floori(10.0*SIMULATION_FPS) # We will simulate 10 seconds worth of frames. 2400 in this case.

const SIMULATION_DT: float = 1.0/SIMULATION_FPS
func _ready() -> void:
	var springs: Array[RSHookeSpring] = []
	springs.resize(SPRINGS_TO_SIMULATE)
	for i in range(SPRINGS_TO_SIMULATE):
		# Make SPRINGS_TO_SIMULATE amount of Vector3-based RSHookeSprings.
		springs[i] = RSHookeSpring.new_vector3(0.5, PI, RSStopwatch.new_manual())
	
	# Let the engine breathe before running the test.
    # This ensures that our metrics don't get skewed from engine initialization overhead.
	await get_tree().create_timer(2.0).timeout
	
	var start: int = Time.get_ticks_usec()
	for frame in range(SIMULATION_STEPS):
		for i in range(SPRINGS_TO_SIMULATE):
			var spring = springs[i] # Grab a mutable reference.
			# Force the spring to update itself and intentionally discard the return value.
			var _p = spring.get_position()
			# Advance the time in the spring.
			spring.time_skip(SIMULATION_DT)
	var end: int = Time.get_ticks_usec()
	# Calculate and print out performance metrics.
	var total_time_usec: int = end - start
	var simulation_time_sec: float = SIMULATION_STEPS/SIMULATION_FPS
	var process_time_usec: float = (total_time_usec as float)/(SIMULATION_STEPS as float)
	print(str(floorf(simulation_time_sec*1000.0)/1000.0) + "s | " + str(SIMULATION_STEPS) + "f -> "\
	 + str(floorf(SIMULATION_FPS*1000.0)/1000.0) + "FPS (" + str(floorf(1.0/SIMULATION_FPS*1000.0*1000.0*1000.0)/1000.0) + " usec/f)")
	print("calc time: " + str(floor(process_time_usec*1000.0)/1000.0) + " usec/f -> "\
	+ str(floorf((process_time_usec/1000.0/1000.0)/(1/SIMULATION_FPS)*100*1000.0)/1000.0) + "% of frame budget")
