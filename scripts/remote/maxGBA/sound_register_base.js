
/**
 * Base class for sound register controllers in Max/MSP
 * Provides common functionality for managing GBA sound registers
 */

/**
 * Field definition class
 * @param {number} bitSize - Number of bits for this field
 * @param {number} min - Minimum value (default: 0)
 * @param {number} max - Maximum value (default: 2^bitSize - 1)
 * @param {number} value - Initial value (default: 0)
 */
function Field(bitSize, min, max) {
    this.bitSize = bitSize;
    this.min = (typeof min !== "undefined") ? min : 0;
    this.max = (typeof max !== "undefined") ? max : (Math.pow(2, bitSize) - 1);
}

/**
 * Creates a sound register controller with validation and register data packing
 * @param {number} size - Number of registers used by this object
 * @param {Object} fields - Field definitions in order
 *                          Format: { 
 *                            fieldName: Field(bitSize, min, max)
 *                          }
 *                          For unused bits, use: unused_X_Y: Field(bitSize)
 */
function createRegisterController(fields) {
    // Compute register bit size by summing field bit sizes
    var registerBitSize = 0;
    for (var fieldName in fields) {
        if (fields.hasOwnProperty(fieldName)) {
            var field = fields[fieldName];
            registerBitSize += field.bitSize;
        }
    }
    // Convert bit size to byte size (rounding up)
    // throw new Error if not multiple of 8
    if (registerBitSize % 8 !== 0) {
        throw new Error("Register bit size must be a multiple of 8. Got: " + registerBitSize);
    }

    var registerByteSize = Math.ceil(registerBitSize / 8);


    var state = {
        SIZE: registerByteSize,  // to be computed
        is_new: false,
        values: {}
    };

    // Initialize field values
    for (var fieldName in fields) {
        if (fields.hasOwnProperty(fieldName)) {
            state.values[fieldName] = 0;
        }
    }


    /**
     * Validates all context values according to validation rules
     * @throws {Error} If any validation fails
     */
    function validate() {
        for (var fieldName in fields) {
            if (fields.hasOwnProperty(fieldName)) {
                var field = fields[fieldName];
                var value = state.values[fieldName];
                if (value < field.min || value > field.max) {
                    throw new Error(fieldName + " must be between " + field.min + " and " + field.max + ". Got: " + value);
                }
            }
        }
    }

    /**
     * Automatically packs register data based on field definitions
     * @returns {number} The packed register data
     */
    function packRegData() {
        var regData = 0;
        var bitPos = 0;

        for (var fieldName in fields) {
            if (!fields.hasOwnProperty(fieldName)) {
                continue;
            }
            var field = fields[fieldName];

            var value = state.values[fieldName];
            // Handle boolean values (convert true/false to 1/0)
            if (typeof value === "boolean") {
                value = value ? 1 : 0;
            }

            // Pack the value at the computed bit position
            regData |= (value << bitPos);

            bitPos += field.bitSize;
        }

        return regData;
    }

    /**
     * Sets a value in the context and marks it as new
     * @param {string} key - The context key to set
     * @param {*} value - The value to set
     */
    function set_value(key, value) {
        state.values[key] = value;
        state.is_new = true;
    }

    /**
     * Sends register data if context has been updated
     */
    function get_reg_data() {
        if (!state.is_new) {
            return;
        }

        validate();
        var regData = packRegData();
        state.is_new = false;
        return regData;
    }

    function help() {
        // return list of field names (except "_" fields) and their min/max values
        var helpText = "Register Controller Help:\n";
        helpText += "Size: " + state.SIZE + " bytes\n";
        helpText += "Fields:\n";
        for (var fieldName in fields) {
            if (!fields.hasOwnProperty(fieldName)) {
                continue;
            }
            var field = fields[fieldName];
            if (fieldName === "_") {
                helpText += "- _unused_: " + field.bitSize + " bits\n";
                continue;
            }
            helpText += "- " + fieldName + ": " + field.bitSize + " bits in [" + field.min + ", " + field.max + "]\n";

        }
        return helpText;
    }

    // Return the public interface
    return {
        help: help,
        set_value: set_value,
        get_reg_data: get_reg_data,
        SIZE: state.SIZE
    };
}
