console:log("\n---------------------------------------")
console:log("MGBA FRAMED SYNC SERVER SCRIPT")

ST_sockets = {}
nextID = 1
server = nil

COMMANDS_BLOCK = ""

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
    console:log("write_command")

	local cmd, addr, val = line:match("^(%w+)%s+(0x%x+)%s+(0x%x+)$")
	if not cmd then
		console:error("MG: Invalid command: " .. line)
		return
	end

	local address = tonumber(addr)
	local value = tonumber(val)

	if cmd == "WRITE8" then
        -- log lua equivalent of python f"emu:write8({address}, {value})"
        console:log(string.format("emu:write8(0x%X, 0x%X)", address, value))
		emu:write8(address, value)
	elseif cmd == "WRITE16" then
        console:log(string.format("emu:write16(0x%X, 0x%X)", address, value))
		emu:write16(address, value)
	elseif cmd == "WRITE32" then
        console:log(string.format("emu:write32(0x%X, 0x%X)", address, value))
		emu:write32(address, value)
	else
		console:error("Unknown command: " .. cmd)
	end
end

function write_commands_block()
    if COMMANDS_BLOCK ~= "" then
        for line in COMMANDS_BLOCK:gmatch("[^\r\n]+") do
            line = line:match("^(.-)%s*$")
            write_command(line)
        end
        COMMANDS_BLOCK = ""
    end
end

function ST_received(id)
	local sock = ST_sockets[id]
	if not sock then return end
	while true do
		local data, err = sock:receive(1024)
		if data then
            console:log(ST_format(id, data, false))
            console:log("...")
            COMMANDS_BLOCK = COMMANDS_BLOCK .. data
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
    write_commands_block()
    for id, sock in pairs(ST_sockets) do
        sock:send("TRIGGER\n")
    end
end

callbacks:add("frame", CB_onFrame)