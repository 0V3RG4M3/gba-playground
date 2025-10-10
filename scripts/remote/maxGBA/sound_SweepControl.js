include("utils.js");

log("sound_SweepControl.js: I N I T I A L I Z E");

var ctx = {
  sweep_num: 0,  // Sweep number in [0, 7]
  sweep_increasing: 0, // True if the sweep is increasing
  sweep_time: 0,  // Sweep time in [0, 7]


  SIZE: 1, // Number of registers used by this object
  is_new: false,
}

function sendRegData() {
  if (0 > ctx.sweep_num || ctx.sweep_num > 7) {
    throw new Error("sweep_num must be between 0 and 7");
  }
  
  if (0 > ctx.sweep_increasing || ctx.sweep_increasing > 1) {
    throw new Error("sweep_increasing must be between 0 and 1");
  }

  if (0 > ctx.sweep_time || ctx.sweep_time > 7) {
    throw new Error("sweep_time must be between 0 and 7");
  }

  const regData = (ctx.sweep_num << 0) |
                  (ctx.sweep_increasing << 3) |
                  (ctx.sweep_time << 4);

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
