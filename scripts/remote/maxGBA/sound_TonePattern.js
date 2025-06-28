include("utils.js");

log("sound_TonePattern.js: I N I T I A L I Z E");

var ctx = {
  length: 0,  // L in [0, 63]. Resulting length is: (64−val)/256 second. So L=0 -> 250 ms, and L=63 -> 3.9 ms
  duty: 0,  // Duty cycle    0: 12.5%, 1: 25%, 2: 50%, 3: 75%
  step_time: 0,  // envelope decay time in [0, 7]. 0: inf, 1: shortest 7: long
  step_increasing: false,
  volume: 0,  // Volume value in [0, 15]

  SIZE: 2, // Number of registers used by this object
}

function sendRegData() {
  if (ctx.length < 0 || ctx.frequency_rate > 63) {
    throw new Error("Length must be between 0 and 63");
  }
  if (ctx.duty < 0 || ctx.duty > 3) {
    throw new Error("Duty must be between 0 and 3");
  }
  if (ctx.step_time < 0 || ctx.step_time > 7) {
    throw new Error("Step time must be between 0 and 7");
  }
  if (ctx.volume < 0 || ctx.volume > 15) {
    throw new Error("Volume must be between 0 and 15");
  }

  const regData = (ctx.length << 0) |
                  (ctx.duty << 6) |
                  (ctx.step_time << 8) |
                  (ctx.step_increasing ? (1 << 11) : 0) |
                  (ctx.volume << 12);

  log("sendRegData(): size", ctx.SIZE);
  outlet(0, "reg_data", ctx.SIZE, regData);
}

function set_value(key, value){
  ctx[key] = value;
  log("set_value(): set", key, "to", value);
  sendRegData();
}
