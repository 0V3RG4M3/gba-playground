autowatch = 1;

var frame_id = -1;
var storage = [];

function anything() {
    // "messagename" contains received symbols (string)
    storage.push(messagename);
}

function clear() {
    storage = [];
}

function flush() {
    if (storage.length === 0) {
        return;
    }

    var output = frame_id + "\n" + storage.join("\n") + "\n";
    clear();

    outlet(0, output);
}

function set_frame_id(value) {
    frame_id = value;
}

function bang() {
    flush();
}

function msg_int(value) {
    set_frame_id(value);
}

