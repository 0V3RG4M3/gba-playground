
include("utils.js");

var ctx = {
    REGISTERS: {
        // only address ans size are useful here, the rest is for documentation
        TONE1_SWEEP: {ADDRESS:0x04000060, SIZE:1, CNAME:"SOUND1CNT_L", DATA_TYPE:"SweepControl", DESCRIPTION:"Tone 1 Sweep control"},
        TONE1_PATTERN: {ADDRESS:0x04000062, SIZE:2, CNAME:"SOUND1CNT_H", DATA_TYPE:"TonePattern", DESCRIPTION:"Tone 1 Duty/Length/Envelope"},
        TONE1_FREQUENCY: {ADDRESS:0x04000064, SIZE:2, CNAME:"SOUND1CNT_X", DATA_TYPE:"ToneFrequency", DESCRIPTION:"Tone 1 Frequency/Control"},

        TONE2_PATTERN: {ADDRESS:0x04000068, SIZE:2, CNAME:"SOUND2CNT_L", DATA_TYPE:"TonePattern", DESCRIPTION:"Tone 2 Duty/Length/Envelope"},   
        TONE2_FREQUENCY: {ADDRESS:0x0400006C, SIZE:2, CNAME:"SOUND2CNT_H", DATA_TYPE:"ToneFrequency", DESCRIPTION:"Tone 2 Frequency/Control"},     
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