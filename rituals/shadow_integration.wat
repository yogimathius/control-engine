(module
    ;; Shadow Integration Ritual - WebAssembly Implementation
    ;; This ritual works with shadow aspects of the psyche

    ;; Import host functions for state manipulation
    (import "codex" "log" (func $log (param i32 i32)))
    (import "codex" "get_archetype_activation" (func $get_archetype_activation (param i32 i32) (result f64)))
    (import "codex" "set_archetype_activation" (func $set_archetype_activation (param i32 i32 f64)))
    (import "codex" "add_symbol" (func $add_symbol (param i32 i32)))
    (import "codex" "get_random" (func $get_random (result f64)))

    ;; Memory for string data
    (memory $memory 1)
    (export "memory" (memory $memory))

    ;; String constants
    (data (i32.const 0) "Shadow")
    (data (i32.const 7) "Light")
    (data (i32.const 13) "Starting shadow integration ritual...")
    (data (i32.const 51) "Shadow work complete. Integration achieved.")
    (data (i32.const 95) "◯●◯")    ;; Shadow integration symbol
    (data (i32.const 104) "🌑")    ;; New moon symbol
    (data (i32.const 108) "⚡")    ;; Energy symbol

    ;; Main ritual execution function
    (func $execute_ritual (export "execute_ritual") (result i32)
        (local $shadow_level f64)
        (local $light_level f64)
        (local $integration_factor f64)
        (local $new_shadow_level f64)
        (local $new_light_level f64)

        ;; Log ritual start
        (call $log (i32.const 13) (i32.const 38))

        ;; Get current shadow archetype activation
        (local.set $shadow_level
            (call $get_archetype_activation (i32.const 0) (i32.const 6))) ;; "Shadow"

        ;; Get current light archetype activation  
        (local.set $light_level
            (call $get_archetype_activation (i32.const 7) (i32.const 5))) ;; "Light"

        ;; Calculate integration factor based on imbalance
        (local.set $integration_factor
            (f64.mul
                (f64.add
                    (f64.const 0.2)
                    (f64.mul (call $get_random) (f64.const 0.3)))
                (f64.add 
                    (f64.const 1.0)
                    (f64.abs (f64.sub (local.get $shadow_level) (local.get $light_level))))))

        ;; Calculate new shadow level (increase by integration factor)
        (local.set $new_shadow_level
            (f64.min
                (f64.const 1.0)
                (f64.add (local.get $shadow_level) (local.get $integration_factor))))

        ;; Calculate new light level (slight increase for balance)
        (local.set $new_light_level
            (f64.min
                (f64.const 1.0)
                (f64.add (local.get $light_level) (f64.mul (local.get $integration_factor) (f64.const 0.3)))))

        ;; Update archetype activations
        (call $set_archetype_activation (i32.const 0) (i32.const 6) (local.get $new_shadow_level))
        (call $set_archetype_activation (i32.const 7) (i32.const 5) (local.get $new_light_level))

        ;; Add integration symbols
        (call $add_symbol (i32.const 95) (i32.const 9))   ;; ◯●◯
        (call $add_symbol (i32.const 104) (i32.const 4))  ;; 🌑

        ;; Add energy symbol if high integration
        (if (f64.gt (local.get $integration_factor) (f64.const 0.4))
            (then
                (call $add_symbol (i32.const 108) (i32.const 4)))) ;; ⚡

        ;; Log completion
        (call $log (i32.const 51) (i32.const 44))

        ;; Return success code
        (i32.const 0)
    )

    ;; Helper function to get ritual resonance (0.0 to 1.0)
    (func $get_resonance (export "get_resonance") (result f64)
        (local $shadow_level f64)
        (local $light_level f64)
        (local $balance f64)

        ;; Get current levels
        (local.set $shadow_level
            (call $get_archetype_activation (i32.const 0) (i32.const 6)))
        (local.set $light_level
            (call $get_archetype_activation (i32.const 7) (i32.const 5)))

        ;; Calculate balance (closer values = higher resonance)
        (local.set $balance
            (f64.sub (f64.const 1.0) 
                (f64.abs (f64.sub (local.get $shadow_level) (local.get $light_level)))))

        ;; Return resonance based on shadow activation and balance
        (f64.mul
            (f64.add (local.get $shadow_level) (local.get $balance))
            (f64.const 0.5))
    )

    ;; Cleanup function (optional)
    (func $cleanup (export "cleanup")
        ;; No cleanup needed for this ritual
        nop
    )
)