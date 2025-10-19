
/**
 * Base class for sound register controllers in Max/MSP
 * Provides common functionality for managing GBA sound registers
 */

/**
 * Field definition class
 * @param {number} bitSize - Number of bits for this field
 * @param {number} min - Minimum value (default: 0)
 * @param {number} max - Maximum value (default: 2^bitSize - 1)
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
function createRegisterController(size, fields) {
  // Initialize context with defaults (all values start at 0)
  var ctx = {
    SIZE: size,
    is_new: false
  };
  
  // Compute bit positions and initialize fields
  var bitPositions = {};
  var currentPos = 0;
  
  for (var fieldName in fields) {
    if (fields.hasOwnProperty(fieldName)) {
      var field = fields[fieldName];
      
      // Store bit position for this field
      bitPositions[fieldName] = currentPos;
      
      // Initialize non-unused fields to 0 in context
      // Unused fields are identified by name starting with "unused_"
      if (fieldName.indexOf("unused_") !== 0) {
        ctx[fieldName] = 0;
      }
      
      // Move position forward by this field's size
      currentPos += field.bitSize;
    }
  }

  /**
   * Validates all context values according to validation rules
   * @throws {Error} If any validation fails
   */
  function validate() {
    for (var fieldName in fields) {
      if (fields.hasOwnProperty(fieldName)) {
        // Skip validation for unused bits
        if (fieldName.indexOf("unused_") === 0) {
          continue;
        }
        
        var field = fields[fieldName];
        var value = ctx[fieldName];
        
        if (value < field.min || value > field.max) {
          throw new Error(fieldName + " must be between " + field.min + " and " + field.max);
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
    
    for (var fieldName in fields) {
      if (fields.hasOwnProperty(fieldName)) {
        // Skip unused bits (they remain 0)
        if (fieldName.indexOf("_") === 0) {
          continue;
        }
        
        var value = ctx[fieldName];
        var bitPos = bitPositions[fieldName];
        
        // Handle boolean values (convert true/false to 1/0)
        if (typeof value === "boolean") {
          value = value ? 1 : 0;
        }
        
        // Pack the value at the computed bit position
        regData |= (value << bitPos);
      }
    }
    
    return regData;
  }

  /**
   * Sends register data to Max outlet
   */
  function sendRegData() {
    validate();
    var regData = packRegData();
    outlet(0, "reg_data", ctx.SIZE, regData);
  }

  /**
   * Sets a value in the context and marks it as new
   * @param {string} key - The context key to set
   * @param {*} value - The value to set
   */
  function set_value(key, value) {
    ctx[key] = value;
    ctx.is_new = true;
  }

  /**
   * Sends register data if context has been updated
   */
  function bang() {
    if (!ctx.is_new) {
      return;
    }
    
    sendRegData();
    ctx.is_new = false;
  }

  // Return the public interface
  return {
    set_value: set_value,
    bang: bang
  };
}
