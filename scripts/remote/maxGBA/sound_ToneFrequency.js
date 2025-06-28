include("utils.js");

log("sound_ToneFrequency.js: I N I T I A L I Z E");
var ctx = {
  frequency_rate: 0,
  stop_when_expired: false,
  enabled: false,

  SIZE: 2, // Number of registers used by this object
}

function sendRegData() {
  if (ctx.frequency_rate < 0 || ctx.frequency_rate > 2047) {
    throw new Error("Frequency must be between 0 and 2047");
  }

  const regData = (ctx.frequency_rate << 0) |
         (ctx.stop_when_expired ? (1 << 14) : 0) |
         (ctx.enabled ? (1 << 15) : 0);

  log("sendRegData(): size", ctx.SIZE);
  outlet(0, "reg_data", ctx.SIZE, regData);
}

function set_value(key, value){
  ctx[key] = value;
  log("set_value(): set", key, "to", value);
  sendRegData();
}
