console:log("\n---------------------------------------")
console:log("MGBA SYNC SERVER SCRIPT")

ST_sockets = {}
nextID = 1
server = nil

-- Simple command queue with lock
COMMAND_QUEUE = ""
QUEUE_LOCK = false

function wait_for_lock()
    while QUEUE_LOCK do
        emu:sleep(1)  -- Small sleep to prevent busy waiting
    end
    QUEUE_LOCK = true
end

function release_lock()
    QUEUE_LOCK = false
end

function ST_stop(id)
    local sock = ST_sockets[id]
    ST_sockets[id] = nil
    if sock then sock:close() end
end

function ST_format(id, msg, isError)
    local prefix = "Socket " .. id
    prefix = prefix .. (isError and " Error: " or " Received: ")
    return prefix .. msg
end

function ST_error(id, err)
    console:error(ST_format(id, err, true))
    ST_stop(id)
end

function write_command(line)
    console:log("----write_command")

    local cmd, addr, val = line:match("^(%w+)%s+(0x%x+)%s+(0x%x+)$")
    if not cmd then
        console:error("----MG: Invalid command: " .. line)
        return
    end

    local address = tonumber(addr)
    local value = tonumber(val)

    if cmd == "WRITE8" then
        console:log(string.format("----emu:write8(0x%X, 0x%X)", address, value))
        emu:write8(address, value)
    elseif cmd == "WRITE16" then
        console:log(string.format("----emu:write16(0x%X, 0x%X)", address, value))
        emu:write16(address, value)
    elseif cmd == "WRITE32" then
        console:log(string.format("----emu:write32(0x%X, 0x%X)", address, value))
        emu:write32(address, value)
    else
        console:error("----Unknown command: " .. cmd .. ", expecting WRITE8, WRITE16, or WRITE32")
    end
end

function process_command_block(data)
    -- Split the data into lines
    local lines = {}
    for line in data:gmatch("[^\r\n]+") do
        table.insert(lines, line)
    end
    
    -- First line should be the frame_id, skip it
    if #lines < 2 then return end -- Need at least frame_id and one command
    
    -- Get all commands (excluding frame_id) and join them with newlines
    local commands = table.concat(table.move(lines, 2, #lines, 1, {}), "\n")
    
    -- Safely append commands to queue
    wait_for_lock()
    if COMMAND_QUEUE ~= "" then
        COMMAND_QUEUE = COMMAND_QUEUE .. "\n" .. commands
    else
        COMMAND_QUEUE = commands
    end
    release_lock()
end

function execute_commands()
    -- Safely get and clear queue
    wait_for_lock()
    local commands = COMMAND_QUEUE
    COMMAND_QUEUE = ""
    release_lock()
    
    -- Execute commands if any
    if commands and commands ~= "" then
        console:log("----Executing command block")
        for line in commands:gmatch("[^\r\n]+") do
            line = line:match("^(.-)%s*$")
            write_command(line)
        end
    end
end

function ST_received(id)
    local sock = ST_sockets[id]
    if not sock then return end
    while true do
        local data, err = sock:receive(1024)
        if data then
            console:log(ST_format(id, data, false))
            process_command_block(data)
        else
            if err ~= socket.ERRORS.AGAIN then
                console:error(ST_format(id, err, true))
                ST_stop(id)
            end
            return
        end
    end
end

function ST_accept()
    console:log("function ST_accept()")
    local sock, err = server:accept()
    if err then
        console:error(ST_format("Accept", err, true))
        return
    end
    local id = nextID
    nextID = id + 1
    ST_sockets[id] = sock
    sock:add("received", function() ST_received(id) end)
    sock:add("error", function() ST_error(id, true) end)
    console:log(ST_format(id, "Connected"))
end

-- Init server
local port = 8888
while not server do
    server, err = socket.bind(nil, port)
    if err == socket.ERRORS.ADDRESS_IN_USE then
        port = port + 1
    else
        if err then
            console:error("Socket bind error: " .. err)
            break
        end
    end
end

if server then
    server:listen()
    server:add("received", ST_accept)
    console:log("Listening on port " .. port)
end

function CB_onFrame()
    execute_commands()
end

callbacks:add("frame", CB_onFrame)