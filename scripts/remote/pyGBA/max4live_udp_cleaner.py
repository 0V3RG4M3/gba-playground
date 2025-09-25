

def clean_udp_message(data: bytes) -> bytes:
    """
    Cleans UDP messages by removing trailing null bytes and commas.
    Example:
        b'4268 WRITE16 0x4000064 0xc689\x00\x00\x00,\x00\x00\x00'
    becomes
        b'4268 WRITE16 0x4000064 0xc689'
    """
    return data.rstrip(b'\x00').rstrip(b',').rstrip(b'\x00')
