include("sound_register_base.js");

// Register definitions - all available register types
var registerDefinitions = {
  "SweepControl": {
    size: 1,
    fields: {
      sweep_num: new Field(3),         // Sweep number in [0, 7]
      sweep_increasing: new Field(1),  // 1 bit: 0-1
      sweep_time: new Field(3),         // Sweep time in [0, 7]
      _: new Field(1)                  // 1 unused bit
    }
  },
  
  "ToneFrequency": {
    size: 2,
    fields: {
      frequency_rate: new Field(11),   // 11 bits: 0-2047
      _: new Field(3),                 // 3 unused bits
      stop_when_expired: new Field(1), // 1 bit: 0-1
      enabled: new Field(1)            // 1 bit: 0-1
    }
  },
  
  "TonePattern": {
    size: 2,
    fields: {
      length: new Field(6),            // L in [0, 63]. Resulting length is: (64−val)/256 second
      duty: new Field(2),              // duty (Duty cycle) 0: 12.5%, 1: 25%, 2: 50%, 3: 75%
      step_time: new Field(3),         // step_time: envelope decay time in [0, 7]
      step_increasing: new Field(1),   // 1 bit: 0-1
      volume: new Field(4)             // Volume in [0, 15]
    }
  },

  "NoiseFrequency": {
    size: 2,
    fields: {
      rate: new Field(3),              // 3 bits: 0-7 (default min=0, max=7)
      counter7: new Field(1),          // 1 bit: 0-1 (default min=0, max=1)
      shift: new Field(4),             // 4 bits: 0-15 (default min=0, max=15)
      _: new Field(6),                 // 6 unused bits
      stop_when_expired: new Field(1), // 1 bit: 0-1
      enabled: new Field(1)            // 1 bit: 0-1
    }
  },
  
  "NoiseLenEnvelope": {
    size: 2,
    fields: {
      length: new Field(6),            // L in [0, 63]. Resulting length is: (64−val)/256 second
      _: new Field(2),                 // 2 unused bits (for missing duty)
      step_time: new Field(3),         // step_time: envelope decay time in [0, 7]
      step_increasing: new Field(1),   // 1 bit: 0-1
      volume: new Field(4)             // Volume in [0, 15]
    }
  },
};

// Global controller variable
var controller;

function initialize() {
  // Get register type from arguments
  var regType = "NoiseFrequency"; // Default value
  
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
  controller = createRegisterController(regDef.size, regDef.fields);
  
  log("gba_sound.js: Successfully initialized", regType);
}

// Initialize the controller
initialize();

// Expose public interface to global scope (required for Max/MSP)
// These need to be function declarations that delegate to the controller
function set_value(key, value) {
  controller.set_value(key, value);
}

function bang() {
  controller.bang();
}

