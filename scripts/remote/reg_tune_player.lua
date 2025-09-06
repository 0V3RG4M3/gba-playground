-- Script pour valider qu'il est possible d'écrire dans les registres de son assez vite pour une musique

console:log("\n---------------------------------------")
console:log("MGBA REG TUNE SCRIPT")


-- list of tuples containing (frame_id (int), size (int), address (int), value (int))
REG_TUNE = {
	{0, 2, 67108962, 648},
    {0, 2, 67108962, 712},
    {0, 2, 67108962, 712},
    {0, 2, 67108968, 580},
    {8, 2, 67108962, 62152},
    {8, 2, 67108964, 34441},
    {15, 2, 67108962, 712},
    {15, 2, 67108964, 34441},
    {23, 2, 67108962, 62152},
    {23, 2, 67108964, 34441},
    {30, 2, 67108962, 712},
    {30, 2, 67108962, 62152},
    {31, 2, 67108964, 34441},
    {31, 2, 67108964, 34482},
    {38, 2, 67108962, 712},
    {38, 2, 67108964, 34482},
    {45, 2, 67108962, 62152},
    {45, 2, 67108964, 34518},
    {53, 2, 67108962, 712},
    {53, 2, 67108964, 34518},
    {60, 2, 67108962, 648},
    {60, 2, 67108962, 62088},
    {60, 2, 67108964, 34593},
    {61, 2, 67108968, 53828},
    {61, 2, 67108972, 34254},
    {68, 2, 67108962, 648},
    {68, 2, 67108968, 580},
    {68, 2, 67108964, 34593},
    {68, 2, 67108972, 34254},
    {68, 2, 67108968, 53828},
    {68, 2, 67108972, 34370},
    {75, 2, 67108962, 62088},
    {75, 2, 67108964, 34593},
    {75, 2, 67108968, 580},
    {76, 2, 67108972, 34370},
    {76, 2, 67108968, 53828},
    {76, 2, 67108972, 34065},
    {83, 2, 67108962, 648},
    {83, 2, 67108964, 34593},
    {83, 2, 67108968, 580},
    {83, 2, 67108972, 34065},
    {83, 2, 67108968, 53828},
    {83, 2, 67108972, 34370},
    {90, 2, 67108962, 62088},
    {90, 2, 67108964, 34593},
    {91, 2, 67108968, 580},
    {91, 2, 67108972, 34370},
    {91, 2, 67108968, 53828},
    {91, 2, 67108972, 34254},
    {98, 2, 67108962, 648},
    {98, 2, 67108968, 580},
    {98, 2, 67108962, 62088},
    {98, 2, 67108964, 34593},
    {98, 2, 67108972, 34254},
    {98, 2, 67108968, 53828},
    {98, 2, 67108972, 34370},
    {98, 2, 67108964, 34566},
}

FRAME_ID = 0
INDEX = 1
IS_RUNNING = true


function ST_onFrame()
	while IS_RUNNING do

		if REG_TUNE[INDEX] == nil then
			IS_RUNNING = false
			console:log("All done!")
			callbacks:remove("frame", ST_onFrame)
			return
		end
		
		if REG_TUNE[INDEX][1] > FRAME_ID then
			console:log(string.format("%i) Frame %i done!", INDEX, FRAME_ID))
			FRAME_ID = FRAME_ID + 1
			return
		end

		local byte_count = REG_TUNE[INDEX][2]
		local address = REG_TUNE[INDEX][3]
		local value = REG_TUNE[INDEX][4]
	
		if byte_count == 1 then
			console:log(string.format("%i:%i) emu:write8(0x%X, 0x%X)", INDEX, FRAME_ID, address, value))
			emu:write8(address, value)
		elseif byte_count == 2 then
			console:log(string.format("%i:%i) emu:write16(0x%X, 0x%X)", INDEX, FRAME_ID, address, value))
			emu:write16(address, value)
		elseif byte_count == 4 then
			console:log(string.format("%i:%i) emu:write32(0x%X, 0x%X)", INDEX, FRAME_ID, address, value))
			emu:write32(address, value)
		else
			console:error(FRAME_ID .. "Unknown command: " .. byte_count)
		end
		
		INDEX = INDEX + 1
	end
end

callbacks:add("frame", ST_onFrame)