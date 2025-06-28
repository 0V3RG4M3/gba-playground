include("utils.js");

var ctx = {
  frequency_rate: 0,
  stop_when_expired: false,
  enabled: false,

  SIZE: 2, // Number of registers used by this object
}

function ToneFrequency(frequency_rate, stop_when_expired, enabled) {
  if (frequency_rate < 0 || frequency_rate > 2047) {
    throw new Error("Frequency must be between 0 and 2047");
  }
  ctx.frequency_rate = frequency_rate || 0;
  ctx.stop_when_expired = !!stop_when_expired;
  ctx.enabled = !!enabled;
}

function sendRegData() {
  const regData = (ctx.frequency_rate << 0) |
         (ctx.stop_when_expired ? (1 << 14) : 0) |
         (ctx.enabled ? (1 << 15) : 0);

  log("sendRegData(): size", ctx.SIZE);
  outlet(0, "reg_data", ctx.SIZE, regData);
}

function frequency_rate(value) {
  log("frequency_rate(): received command:", value);
  if (value < 0 || value > 2047) {
    throw new Error("Frequency must be between 0 and 2047");
  }
  ctx.frequency_rate = value;
  sendRegData();
}

function stop_when_expired(value) {
  log("stop_when_expired(): received command:", value);
  ctx.stop_when_expired = !!value;
  sendRegData();
}

function enabled(value) {
  log("enabled(): received command:", value);
  ctx.enabled = !!value;
  sendRegData();
}

function initialize() {
  // get current file name
  log("sound_ToneFrequency.js:", "I N I T I A L I Z E");
}
initialize();