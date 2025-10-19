
include("utils.js");

var ctx = {
    REGISTERS: {
        // only address and size are useful here, the rest is for documentation
        
        //def_mmio!(0x0400_0060 = TONE1_SWEEP/["SOUND1CNT_L","NR10"]: VolAddress<SweepControl, Safe, Safe>; "Tone 1 Sweep");
        TONE1_SWEEP: {ADDRESS:0x04000060, SIZE:1, CNAME:"SOUND1CNT_L", DATA_TYPE:"SweepControl", DESCRIPTION:"Tone 1 Sweep control"},
        //def_mmio!(0x0400_0062 = TONE1_PATTERN/["SOUND1CNT_H","NR11","NR12"]: VolAddress<TonePattern, Safe, Safe>; "Tone 1 Duty/Len/Envelope");
        TONE1_PATTERN: {ADDRESS:0x04000062, SIZE:2, CNAME:"SOUND1CNT_H", DATA_TYPE:"TonePattern", DESCRIPTION:"Tone 1 Duty/Length/Envelope"},
        //def_mmio!(0x0400_0064 = TONE1_FREQUENCY/["SOUND1CNT_X","NR13","NR14"]: VolAddress<ToneFrequency, Safe, Safe>; "Tone 1 Frequency/Control");
        TONE1_FREQUENCY: {ADDRESS:0x04000064, SIZE:2, CNAME:"SOUND1CNT_X", DATA_TYPE:"ToneFrequency", DESCRIPTION:"Tone 1 Frequency/Control"},

        //def_mmio!(0x0400_0068 = TONE2_PATTERN/["SOUND2CNT_L","NR21","NR22"]: VolAddress<TonePattern, Safe, Safe>; "Tone 2 Duty/Len/Envelope");
        TONE2_PATTERN: {ADDRESS:0x04000068, SIZE:2, CNAME:"SOUND2CNT_L", DATA_TYPE:"TonePattern", DESCRIPTION:"Tone 2 Duty/Length/Envelope"},   
        //def_mmio!(0x0400_006C = TONE2_FREQUENCY/["SOUND2CNT_H","NR23","NR24"]: VolAddress<ToneFrequency, Safe, Safe>; "Tone 2 Frequency/Control");
        TONE2_FREQUENCY: {ADDRESS:0x0400006C, SIZE:2, CNAME:"SOUND2CNT_H", DATA_TYPE:"ToneFrequency", DESCRIPTION:"Tone 2 Frequency/Control"},     

        //def_mmio!(0x0400_0078 = NOISE_LEN_ENV/["SOUND4CNT_L","NR41","NR42"]: VolAddress<NoiseLenEnvelope, Safe, Safe>; "Noise Length/Envelope");
        NOISE_LEN_ENV: {ADDRESS:0x04000078, SIZE:2, CNAME:"SOUND4CNT_L", DATA_TYPE:"NoiseLenEnvelope", DESCRIPTION:"Noise Length/Envelope"},
        //def_mmio!(0x0400_007C = NOISE_FREQ/["SOUND4CNT_H","NR43","NR44"]: VolAddress<NoiseFrequency, Safe, Safe>; "Noise Frequency/Control");
        NOISE_FREQ: {ADDRESS:0x0400007C, SIZE:2, CNAME:"SOUND4CNT_H", DATA_TYPE:"NoiseFrequency", DESCRIPTION:"Noise Frequency/Control"},

        //def_mmio!(0x0400_0080 = LEFT_RIGHT_VOLUME/["SOUNDCNT_L","NR50","NR51"]: VolAddress<LeftRightVolume, Safe, Safe>;"Left/Right sound control (but GBAs only have one speaker each).");
        LEFT_RIGHT_VOLUME: {ADDRESS:0x04000080, SIZE:2, CNAME:"SOUNDCNT_L", DATA_TYPE:"LeftRightVolume", DESCRIPTION:"Left/Right sound control (but GBAs only have one speaker each)."},
        //def_mmio!(0x0400_0082 = SOUND_MIX/["SOUNDCNT_H"]: VolAddress<SoundMix, Safe, Safe>;"Mixes sound sources out to the left and right");
        SOUND_MIX: {ADDRESS:0x04000082, SIZE:1, CNAME:"SOUNDCNT_H", DATA_TYPE:"SoundMix", DESCRIPTION:"Mixes sound sources out to the left and right"},
        //def_mmio!(0x0400_0084 = SOUND_ENABLED/["SOUNDCNT_X"]: VolAddress<SoundEnable, Safe, Safe>;"Sound active flags (r), as well as the sound primary enable (rw).");
        SOUND_ENABLED: {ADDRESS:0x04000084, SIZE:2, CNAME:"SOUNDCNT_X", DATA_TYPE:"SoundEnable", DESCRIPTION:"Sound active flags (r), as well as the sound primary enable (rw)."},
        //def_mmio!(0x0400_0088 = SOUNDBIAS: VolAddress<SoundBias, Safe, Safe>;"Provides a bias to set the 'middle point' of sound output.");
        SOUNDBIAS: {ADDRESS:0x04000088, SIZE:2, CNAME:"SOUNDBIAS", DATA_TYPE:"SoundBias", DESCRIPTION:"Provides a bias to set the 'middle point' of sound output."},

    },
    regName: ""
}


function sendRegData(size, regData) {
    if (!ctx.regName) {
        log("ERROR: regName is not set. Cannot send register data.");
        return
    }
    
    const register = ctx.REGISTERS[ctx.regName];
    
    // ensure the size is correct. This 
    if (size !== register.SIZE) {
        log("ERROR: Size mismatch. Expected", register.SIZE, ", got", size);
        return; 
    }
    
    const cmd = "WRITE" + size*8 + " 0x" + register.ADDRESS.toString(16) + " 0x" + regData.toString(16);
    outlet(0, cmd);
}


function reg_data(size, regData){
    sendRegData(size, regData);
}

function initialize(){
    // initialization
    log("mmio.js:", "I N I T I A L I Z E");
    if (jsarguments.length>1)
        ctx.regName = jsarguments[1];
}
initialize();