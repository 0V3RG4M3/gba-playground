
include("utils.js");

var ctx = {
    REGISTERS: {
        // only address ans size are useful here, the rest is for documentation
        TONE1_SWEEP: {ADDRESS:0x04000060, SIZE:1, CNAME:"SOUND1CNT_L", DATA_TYPE:"SweepControl", DESCRIPTION:"Tone 1 Sweep control"},
        TONE1_PATTERN: {ADDRESS:0x04000062, SIZE:2, CNAME:"SOUND1CNT_H", DATA_TYPE:"TonePattern", DESCRIPTION:"Tone 1 Duty/Length/Envelope"},
        TONE1_FREQUENCY: {ADDRESS:0x04000064, SIZE:2, CNAME:"SOUND1CNT_X", DATA_TYPE:"ToneFrequency", DESCRIPTION:"Tone 1 Frequency/Control"},

        //def_mmio!(0x0400_0068 = TONE2_PATTERN/["SOUND2CNT_L","NR21","NR22"]: VolAddress<TonePattern, Safe, Safe>; "Tone 2 Duty/Len/Envelope");
        TONE2_PATTERN: {ADDRESS:0x04000068, SIZE:2, CNAME:"SOUND2CNT_L", DATA_TYPE:"TonePattern", DESCRIPTION:"Tone 2 Duty/Length/Envelope"},   
        //def_mmio!(0x0400_006C = TONE2_FREQUENCY/["SOUND2CNT_H","NR23","NR24"]: VolAddress<ToneFrequency, Safe, Safe>; "Tone 2 Frequency/Control");
        TONE2_FREQUENCY: {ADDRESS:0x0400006C, SIZE:2, CNAME:"SOUND2CNT_H", DATA_TYPE:"ToneFrequency", DESCRIPTION:"Tone 2 Frequency/Control"},     

        //def_mmio!(0x0400_0078 = NOISE_LEN_ENV/["SOUND4CNT_L","NR41","NR42"]: VolAddress<NoiseLenEnvelope, Safe, Safe>; "Noise Length/Envelope");
        NOISE_LEN_ENV: {ADDRESS:0x04000078, SIZE:2, CNAME:"SOUND4CNT_L", DATA_TYPE:"NoiseLenEnvelope", DESCRIPTION:"Noise Length/Envelope"},
        //def_mmio!(0x0400_007C = NOISE_FREQ/["SOUND4CNT_H","NR43","NR44"]: VolAddress<NoiseFrequency, Safe, Safe>; "Noise Frequency/Control");
        NOISE_FREQ: {ADDRESS:0x0400007C, SIZE:2, CNAME:"SOUND4CNT_H", DATA_TYPE:"NoiseFrequency", DESCRIPTION:"Noise Frequency/Control"},


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