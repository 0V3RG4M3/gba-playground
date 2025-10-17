include("utils.js");

log("sound_NoiseFrequency.js: I N I T I A L I Z E");

var ctx = {
  rate: 0,  // r in [0, 7] divisor code
  counter7: 0,  // true for 7 bits LFSR, false for 15 bits
  shift: 0,  // s in [0, 15] clock shift
  stop_when_expired: 0,  // True if the sound should stop when the length expires
  enabled: 0,  // True if the sound is enabled

  SIZE: 2, // Number of registers used by this object
  is_new: false,
}

function sendRegData() {
  if (ctx.rate < 0 || ctx.rate > 7) {
    throw new Error("Rate must be between 0 and 7");
  }
  if (ctx.counter7 < 0 || ctx.counter7 > 1) {
    throw new Error("counter7 must be 0 or 1");
  }
  if (ctx.shift < 0 || ctx.shift > 15) {
    throw new Error("Shift must be between 0 and 15");
  }
  if (ctx.stop_when_expired < 0 || ctx.stop_when_expired > 1) {
    throw new Error("stop_when_expired must be 0 or 1");
  }
  if (ctx.enabled < 0 || ctx.enabled > 1) {
    throw new Error("enabled must be 0 or 1");
  }

  const regData = (ctx.rate << 0) |  // 3 bits
                  (ctx.counter7 << 3) |  // 1 bit
                  (ctx.shift << 4) |  // 4 bits
                  // 6 unused bits
                  (ctx.stop_when_expired << 14) |  // 1 bit
                  (ctx.enabled << 15);  // 1 bit

  // Es------ SSSSCRRR
  outlet(0, "reg_data", ctx.SIZE, regData);
}

function set_value(key, value){
  ctx[key] = value;
  ctx.is_new = true;
}

function bang(){
  if (!ctx.is_new)
    return;

  sendRegData();
  ctx.is_new = false;
}
