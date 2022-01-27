import display_picture
from dotenv import dotenv_values
from MTGLibrary import *
from pytesseract import Output
from Network import *
from Streams import *
from card_request import card_request
import logging
import asyncio

logging.basicConfig(filename='logging.log', filemode='a',format='%(asctime)s %(message)s', datefmt='%m/%d/%Y %I:%M:%S %p', level=logging.DEBUG)
logging.info('Began running MTG sorting listening server...')
logging.info('Loading .env file values')
config = dotenv_values("OCRScripts/.env")
logging.info('Loading card data into memory')
library = MTGLibrary(config["CARDDATA"])

async def go():
    request = ['type_of']['type'] = "TakePicture"
    response = await card_request(library, request)

asyncio.run(go())