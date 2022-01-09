import socket
import sys
import pickle


class Response:
    def __init__(self, error, value):
        self.error = error
        self.value = value

class Network:
    def __init__(self, ip, port):
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.server_address = (ip, port)

    def connect(self):
        self.sock.bind(self.server_address)
        self.sock.listen(1)
        self.wait_connection()

    def wait_connection(self):
        while True:
            # Wait for a connection
            print("waiting for a connection")
            self.connection, self.client_address = self.sock.accept()
            return

    def recieve(self):
        data_size = 0
        try:
            while data_size == 0:
                data = b''
                payload_size = int.from_bytes(self.connection.recv(4), byteorder='big')
                data_size = payload_size
                while data_size > 0:
                    temp_data = self.connection.recv(data_size)
                    data_size -= len(temp_data)
                    data += temp_data
                if payload_size == len(data) and payload_size != 0:
                    output = pickle.loads(data) 
                    return output
        except:
            self.connection.close()

    def send(self, response):
        bytes_out = pickle.dumps(response)
        payload_size = len(bytes_out)
        data_sent = 0
        output_stream = payload_size.to_bytes(4, 'big') + bytes_out
        while data_sent < payload_size: 
            data_sent += self.connection.send(output_stream[data_sent:])
