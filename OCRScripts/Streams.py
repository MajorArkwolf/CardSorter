import asyncio
import socket
import pickle


class Response:
    def __init__(self, error, value):
        self.error = error
        self.value = value


async def recieve(reader):
    if reader.at_eof() == False:
        header = await reader.read(4)
        payload_size = int.from_bytes(header, byteorder='big')
        if payload_size == 0:
            return None
        data = await reader.readexactly(payload_size)
        if len(data) > 0 and len(data) == payload_size:
            output = pickle.loads(data) 
            return output
    return None

async def send(writer, response):
    bytes_out = pickle.dumps(response)
    payload_size = len(bytes_out)
    output_stream = payload_size.to_bytes(4, 'big') + bytes_out
    writer.write(output_stream)
    await writer.drain()
