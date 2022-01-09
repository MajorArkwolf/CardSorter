import display_picture
from MTGLibrary import *
from pytesseract import Output
from Network import *
from Streams import *
import logging
import asyncio

logging.basicConfig(filename='logging.log', filemode='a',format='%(asctime)s %(message)s', datefmt='%m/%d/%Y %I:%M:%S %p', level=logging.DEBUG)
logging.info('Began running MTG sorting listening server...')

library = MTGLibrary("A:\\Coding\\MTGSorter\\Data\\default-cards.json")

def is_socket_closed(sock: socket.socket):
    try:
        data = sock.recv(16, socket.MSG_DONTWAIT | socket.MSG_PEEK)
        if len(data) == 0:
            return True
    except ConnectionResetError:
        return True
    except BlockingIOError:
        return False
    except Exception:
        return False
    return False

class ListeningConnection:
    def __init__(self, reader, writer, peer_name):
        self.reader = reader
        self.writer = writer
        self.connection_active = True
        self.peer_name = peer_name
        self.socket = self.writer.get_extra_info('socket')

    async def get_request(self):
        request = await recieve(self.reader)
        if request is not None:
            logging.info("Request recieved, processing.")
            response = await self.handle_request(request)
            logging.info("Request completed, sending response.")
        if is_socket_closed(self.socket) == True or self.connection_active == False:
            logging.info("Connection has been closed.")
            await self.close_connection()

    async def handle_request(self, request):
        try:
            if request['CardData'] is not None:
                request = request['CardData']
                response = await self.card_request_from_binary(request)
                await send(self.writer, response)
                return
            elif request['EndConnection'] is not None:
                self.close_connection()
                response = Response(1, 0)
                await send(self.writer, response)
                return
            else: 
                await send(self.writer, Response(-1, 0))
                return
        except:
            logging.exception('')
            await send(self.writer, Response(-2, 0))
            await self.close_connection()
            return 

    async def close_connection(self):
        logging.info("Writer is closing, terminating this connection.")
        self.connection_active = False
        self.writer.close()
        await self.writer.wait_closed()

async def handle_client(reader, writer):
    peer_name = writer.get_extra_info('peername')
    logging.info(f"New connection opened from {peer_name}.")
    listener = ListeningConnection(reader, writer, peer_name)
    while listener.connection_active == True:
        await listener.get_request()

async def run_server_listening():
    try:
        logging.info("Starting async server listen...")
        server = await asyncio.start_server(handle_client, 'localhost', 10000)
        async with server:
            await server.serve_forever()
    except:
        logging.exception('')

async def main():
    loop = asyncio.get_event_loop()
    server = await asyncio.start_server(handle_client, 'localhost', 10000)
    while True:
        await server.start_serving()
    

asyncio.run(main())