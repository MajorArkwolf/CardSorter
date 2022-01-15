from MTGFactory import Generate_Magic_Card, Get_Card_From_Bytes, Get_Card_From_File
from MTGLibrary import *
from Network import Response
from camera import Camera

def from_binary(byte_stream):
    image = Get_Card_From_Bytes(byte_stream)
    return image

def take_photo():
    camera = Camera()
    if camera.loaded == True:
        return camera.capture_opencv()
    else:
        raise Exception("Camera module not loaded")

def load_from_file(file_location):
    return Get_Card_From_File(file_location)

async def card_request(library, request):
    image = None
    if request['type_of']['type'] == "Binary":
        image = from_binary(request["data"])
    elif request['type_of']['type'] == "FileLocation":
        image = load_from_file(request["data"])
    elif request['type_of']['type'] == "TakePicture":
        image = take_photo()
    else:
        return Response(-2, 0)

    card = Generate_Magic_Card(image)
    if card != None:
        card_search = library.Look_Up_Card(card)
        return Response(0, 1)
    else:
        return Response(0, 0)