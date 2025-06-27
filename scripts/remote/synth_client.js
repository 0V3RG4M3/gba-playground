function ToneFrequency(frequency_rate, stop_when_expired, enabled) {
  if (frequency_rate < 0 || frequency_rate > 2047) {
    throw new Error("Frequency must be between 0 and 2047");
  }
  this.frequency_rate = frequency_rate || 0;
  this.stop_when_expired = !!stop_when_expired;
  this.enabled = !!enabled;
}

ToneFrequency.prototype.value = function() {
  return (this.frequency_rate << 0) |
         (this.stop_when_expired ? (1 << 14) : 0) |
         (this.enabled ? (1 << 15) : 0);
};


function LeftRightVolume(opts) {
  opts = opts || {};
  this.right_volume = opts.right_volume || 0;
  this.left_volume = opts.left_volume || 0;

  this.tone1_right = !!opts.tone1_right;
  this.tone2_right = !!opts.tone2_right;
  this.wave_right = !!opts.wave_right;
  this.noise_right = !!opts.noise_right;

  this.tone1_left = !!opts.tone1_left;
  this.tone2_left = !!opts.tone2_left;
  this.wave_left = !!opts.wave_left;
  this.noise_left = !!opts.noise_left;

  if (this.right_volume > 7 || this.left_volume > 7) {
    throw new Error("Volumes must be in [0, 7]");
  }
}

LeftRightVolume.prototype.value = function() {
  return (this.right_volume << 0) | (this.left_volume << 4) |
         (this.tone1_right << 8) | (this.tone2_right << 9) |
         (this.wave_right << 10) | (this.noise_right << 11) |
         (this.tone1_left << 12) | (this.tone2_left << 13) |
         (this.wave_left << 14) | (this.noise_left << 15);
};


function SoundMix(opts) {
  opts = opts || {};
  this.psg = opts.psg || 0; // 0: 25%, 1: 50%, 2: 100%
  this.sound_a_full = !!opts.sound_a_full;
  this.sound_b_full = !!opts.sound_b_full;
  this.sound_a_right = !!opts.sound_a_right;
  this.sound_a_left = !!opts.sound_a_left;
  this.sound_a_timer = !!opts.sound_a_timer;
  this.sound_a_reset = !!opts.sound_a_reset;
  this.sound_b_right = !!opts.sound_b_right;
  this.sound_b_left = !!opts.sound_b_left;
  this.sound_b_timer = !!opts.sound_b_timer;
  this.sound_b_reset = !!opts.sound_b_reset;
}

SoundMix.prototype.value = function() {
  return (this.psg << 0) |
         (this.sound_a_full << 2) |
         (this.sound_b_full << 3) |
         (this.sound_a_right << 4) |
         (this.sound_a_left << 5) |
         (this.sound_a_timer << 6) |
         (this.sound_a_reset << 7) |
         (this.sound_b_right << 8) |
         (this.sound_b_left << 9) |
         (this.sound_b_timer << 10) |
         (this.sound_b_reset << 11);
};


function SoundEnable(opts) {
  opts = opts || {};
  this.tone1_playing = !!opts.tone1_playing;
  this.tone2_playing = !!opts.tone2_playing;
  this.wave_playing = !!opts.wave_playing;
  this.noise_playing = !!opts.noise_playing;
  this.enabled = !!opts.enabled;
}

SoundEnable.prototype.value = function() {
  return (this.tone1_playing << 0) |
         (this.tone2_playing << 1) |
         (this.wave_playing << 2) |
         (this.noise_playing << 3) |
         (this.enabled << 7);
};


function SoundBias(bias_level, sample_cycle) {
  if (bias_level < 1 || bias_level > 9) {
    throw new Error("Bias level must be between 1 and 9");
  }
  if (sample_cycle < 0 || sample_cycle > 3) {
    throw new Error("Sample cycle must be between 0 and 3");
  }
  this.bias_level = bias_level || 0;
  this.sample_cycle = sample_cycle || 0;
}

SoundBias.prototype.value = function() {
  return (this.bias_level << 1) | (this.sample_cycle << 14);
};
