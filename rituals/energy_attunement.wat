(module
    ;; Energy Attunement Ritual - WebAssembly Implementation
    ;; Balances and harmonizes elemental energies

    ;; Import host functions
    (import "codex" "log" (func $log (param i32 i32)))
    (import "codex" "get_energy_amplitude" (func $get_energy_amplitude (param i32 i32) (result f64)))
    (import "codex" "set_energy_amplitude" (func $set_energy_amplitude (param i32 i32 f64)))
    (import "codex" "add_symbol" (func $add_symbol (param i32 i32)))
    (import "codex" "get_random" (func $get_random (result f64)))

    ;; Memory
    (memory $memory 1)
    (export "memory" (memory $memory))

    ;; String constants
    (data (i32.const 0) "Fire")
    (data (i32.const 5) "Water") 
    (data (i32.const 11) "Earth")
    (data (i32.const 17) "Air")
    (data (i32.const 21) "Beginning energy attunement...")
    (data (i32.const 51) "Energy harmonization complete.")
    (data (i32.const 82) "∿∿∿")    ;; Wave symbol for energy
    (data (i32.const 91) "⚡")     ;; Energy spark
    (data (i32.const 95) "🔥")     ;; Fire symbol
    (data (i32.const 99) "💧")     ;; Water symbol

    (func $execute_ritual (export "execute_ritual") (result i32)
        (local $fire_amp f64)
        (local $water_amp f64) 
        (local $earth_amp f64)
        (local $air_amp f64)
        (local $total_energy f64)
        (local $target_level f64)
        (local $adjustment f64)

        ;; Log start
        (call $log (i32.const 21) (i32.const 30))

        ;; Get current energy amplitudes
        (local.set $fire_amp (call $get_energy_amplitude (i32.const 0) (i32.const 4)))   ;; Fire
        (local.set $water_amp (call $get_energy_amplitude (i32.const 5) (i32.const 5)))  ;; Water  
        (local.set $earth_amp (call $get_energy_amplitude (i32.const 11) (i32.const 5))) ;; Earth
        (local.set $air_amp (call $get_energy_amplitude (i32.const 17) (i32.const 3)))   ;; Air

        ;; Calculate total energy
        (local.set $total_energy
            (f64.add
                (f64.add (local.get $fire_amp) (local.get $water_amp))
                (f64.add (local.get $earth_amp) (local.get $air_amp))))

        ;; Calculate target balanced level (average + small random variation)
        (local.set $target_level
            (f64.add
                (f64.div (local.get $total_energy) (f64.const 4.0))
                (f64.mul (call $get_random) (f64.const 0.1))))

        ;; Attune each energy toward balance with gentle adjustment
        (local.set $adjustment (f64.const 0.3))

        ;; Adjust Fire
        (call $set_energy_amplitude (i32.const 0) (i32.const 4)
            (f64.add (local.get $fire_amp)
                (f64.mul (f64.sub (local.get $target_level) (local.get $fire_amp)) (local.get $adjustment))))

        ;; Adjust Water  
        (call $set_energy_amplitude (i32.const 5) (i32.const 5)
            (f64.add (local.get $water_amp)
                (f64.mul (f64.sub (local.get $target_level) (local.get $water_amp)) (local.get $adjustment))))

        ;; Adjust Earth
        (call $set_energy_amplitude (i32.const 11) (i32.const 5)
            (f64.add (local.get $earth_amp)
                (f64.mul (f64.sub (local.get $target_level) (local.get $earth_amp)) (local.get $adjustment))))

        ;; Adjust Air
        (call $set_energy_amplitude (i32.const 17) (i32.const 3)
            (f64.add (local.get $air_amp)
                (f64.mul (f64.sub (local.get $target_level) (local.get $air_amp)) (local.get $adjustment))))

        ;; Add harmonic symbols
        (call $add_symbol (i32.const 82) (i32.const 9))  ;; ∿∿∿
        (call $add_symbol (i32.const 91) (i32.const 4))  ;; ⚡

        ;; Add elemental symbols if high energy
        (if (f64.gt (local.get $total_energy) (f64.const 2.0))
            (then
                (call $add_symbol (i32.const 95) (i32.const 4))  ;; 🔥
                (call $add_symbol (i32.const 99) (i32.const 4)))) ;; 💧

        ;; Log completion
        (call $log (i32.const 51) (i32.const 31))

        ;; Return success
        (i32.const 0)
    )

    (func $get_resonance (export "get_resonance") (result f64)
        (local $fire_amp f64)
        (local $water_amp f64)
        (local $earth_amp f64) 
        (local $air_amp f64)
        (local $variance f64)
        (local $mean f64)

        ;; Get amplitudes
        (local.set $fire_amp (call $get_energy_amplitude (i32.const 0) (i32.const 4)))
        (local.set $water_amp (call $get_energy_amplitude (i32.const 5) (i32.const 5)))
        (local.set $earth_amp (call $get_energy_amplitude (i32.const 11) (i32.const 5)))
        (local.set $air_amp (call $get_energy_amplitude (i32.const 17) (i32.const 3)))

        ;; Calculate mean
        (local.set $mean 
            (f64.div
                (f64.add
                    (f64.add (local.get $fire_amp) (local.get $water_amp))
                    (f64.add (local.get $earth_amp) (local.get $air_amp)))
                (f64.const 4.0)))

        ;; Calculate variance (simplified)
        (local.set $variance
            (f64.add
                (f64.add
                    (f64.abs (f64.sub (local.get $fire_amp) (local.get $mean)))
                    (f64.abs (f64.sub (local.get $water_amp) (local.get $mean))))
                (f64.add
                    (f64.abs (f64.sub (local.get $earth_amp) (local.get $mean)))
                    (f64.abs (f64.sub (local.get $air_amp) (local.get $mean))))))

        ;; Return resonance (lower variance = higher resonance)
        (f64.sub (f64.const 1.0) (f64.min (local.get $variance) (f64.const 0.8)))
    )

    (func $cleanup (export "cleanup")
        nop
    )
)