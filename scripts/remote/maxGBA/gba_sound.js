include("sound_register_base.js");
include("utils.js");

// Register definitions - all available register types
var registerDefinitions = {
    "SweepControl": {
        fields: {
            sweep_num: new Field(3),         // Sweep number in [0, 7]
            sweep_increasing: new Field(1),  // 1 bit: 0-1
            sweep_time: new Field(3),        // Sweep time in [0, 7]
            _: new Field(1)                  // 1 unused bit
        }
    },

    "TonePattern": {
        fields: {
            length: new Field(6),            // L in [0, 63]. Resulting length is: (64−val)/256 second
            duty: new Field(2),              // duty (Duty cycle) 0: 12.5%, 1: 25%, 2: 50%, 3: 75%
            step_time: new Field(3),         // step_time: envelope decay time in [0, 7]
            step_increasing: new Field(1),   // 1 bit: 0-1
            volume: new Field(4)             // Volume in [0, 15]
        }
    },

    "ToneFrequency": {
        fields: {
            frequency_rate: new Field(11),   // 11 bits: 0-2047
            _: new Field(3),                 // 3 unused bits
            stop_when_expired: new Field(1), // 1 bit: 0-1
            enabled: new Field(1)            // 1 bit: 0-1
        }
    },

    "NoiseLenEnvelope": {
        fields: {
            length: new Field(6),            // L in [0, 63]. Resulting length is: (64−val)/256 second
            _: new Field(2),                 // 2 unused bits (for missing duty)
            step_time: new Field(3),         // step_time: envelope decay time in [0, 7]
            step_increasing: new Field(1),   // 1 bit: 0-1
            volume: new Field(4)             // Volume in [0, 15]
        }
    },

    "NoiseFrequency": {
        fields: {
            rate: new Field(3),              // r in [0, 7] divisor code
            counter7: new Field(1),          // 1 bit: 0-1
            shift: new Field(4),             // s in [0, 15] clock shift
            _: new Field(6),                 // 6 unused bits
            stop_when_expired: new Field(1), // 1 bit: 0-1
            enabled: new Field(1)            // 1 bit: 0-1
        }
    },

    "LeftRightVolume": {
        fields: {
            right_volume: new Field(3),      // Right volume in [0, 7]
            _unused_3: new Field(1),         // 1 unused bit
            left_volume: new Field(3),       // Left volume in [0, 7]
            _unused_7: new Field(1),         // 1 unused bit
            tone1_right: new Field(1),       // True if Tone 1 is enabled on the right channel
            tone2_right: new Field(1),       // True if Tone 2 is enabled on the right channel
            wave_right: new Field(1),        // True if Wave is enabled on the right channel
            noise_right: new Field(1),       // True if Noise is enabled on the right channel
            tone1_left: new Field(1),        // True if Tone 1 is enabled on the left channel
            tone2_left: new Field(1),        // True if Tone 2 is enabled on the left channel
            wave_left: new Field(1),         // True if Wave is enabled on the left channel
            noise_left: new Field(1)         // True if Noise is enabled on the left channel
        }
    },

    "SoundMix": {
        fields: {
            psg: new Field(2),               // PSG output level. 0: 25%, 1: 50%, 2: 100%, 3: not used
            sound_a_full: new Field(1),      // True if Sound A buffer is full
            sound_b_full: new Field(1),      // True if Sound B buffer is full
            sound_a_right: new Field(1),     // True if Sound A is enabled on the right channel
            sound_a_left: new Field(1),      // True if Sound A is enabled on the left channel
            sound_a_timer: new Field(1),     // True if Sound A timer is enabled
            sound_a_reset: new Field(1),     // True if Sound A is reset
            sound_b_right: new Field(1),     // True if Sound B is enabled on the right channel
            sound_b_left: new Field(1),      // True if Sound B is enabled on the left channel
            sound_b_timer: new Field(1),     // True if Sound B timer is enabled
            sound_b_reset: new Field(1)      // True if Sound B is reset
        }
    },

    "SoundEnable": {
        fields: {
            tone1_playing: new Field(1),
            tone2_playing: new Field(1),
            wave_playing: new Field(1),
            noise_playing: new Field(1),
            _: new Field(3),                 // 3 unused bits
            enabled: new Field(1)            // 1 bit: 0-1
        }
    },

    "SoundBias": {
        fields: {
            _unused_0: new Field(1),         // 1 unused bit
            bias_level: new Field(9),        // Bias level in [0, 511]
            _unused_10_13: new Field(4),     // 4 unused bits
            sample_cycle: new Field(2)       // Sample cycle in [0, 3] - 0: 9bit, 1: 8bit, 2: 7bit, 3: 6bit
        }
    }
};

// Global controller variable
var controller;

function initialize() {
    // Get register type from arguments
    var regType;

    if (jsarguments.length > 1) {
        regType = jsarguments[1];
    }

    log("gba_sound.js: Initializing register type:", regType);

    // Check if the register type exists
    if (!registerDefinitions[regType]) {
        throw new Error("Unknown register type: " + regType + ". Available types: " +
            Object.keys(registerDefinitions).join(", "));
    }

    // Create the controller for the specified register type
    var regDef = registerDefinitions[regType];
    controller = createRegisterController(regDef.fields);

    outlets = 2;

    log("gba_sound.js: Successfully initialized", regType);
}

// Initialize the controller
initialize();

// Expose public interface to global scope (required for Max/MSP)
// These need to be function declarations that delegate to the controller
function set_value(key, value) {
    controller.set_value(key, value);
}

function help() {
    outlet(1, "help", controller.help());
}

function bang() {
    var reg_data = controller.get_reg_data();
    if (reg_data !== undefined) {
        outlet(0, "reg_data", controller.SIZE, reg_data);
    }
}

