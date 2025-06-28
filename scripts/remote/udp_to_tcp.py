import socket


def udp_to_tcp(udp_port, tcp_host, tcp_port):
    # Create a UDP socket
    udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    udp_socket.bind(('0.0.0.0', udp_port))

    # Create a TCP socket
    tcp_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    tcp_socket.connect((tcp_host, tcp_port))

    while True:
        # Receive data from UDP socket
        data, addr = udp_socket.recvfrom(1024)
        print(f"Received UDP data from {addr}: {data}")

        # Send data to TCP socket
        tcp_socket.sendall(data)

    # Close sockets
    udp_socket.close()
    tcp_socket.close()

if __name__ == "__main__":
    udp_to_tcp(9999, 'localhost', 8888)