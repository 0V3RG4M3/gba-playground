
def clean_udp_message(data: bytes) -> bytes:
    return data.rstrip(b'\x00').rstrip(b',').rstrip(b'\x00')
