import time
import asyncio
camera_lib_loaded = None
try:
    import picamera
    camera_lib_loaded = True
except ModuleNotFoundError as err:
    camera_lib_loaded = False

import numpy as np
import cv2

class Camera:
    def __init__(self):
        if camera_lib_loaded == True:
            self.camera = picamera.PiCamera()
            self.camera.framerate = 24
            self.camera.resolution = (1920, 1440)
            self.camera.rotation = 180
            self.lock = asyncio.Lock()
        self.loaded = camera_lib_loaded

    async def capture_opencv(self):
        if camera_lib_loaded == True:
            image = np.empty((self.camera.resolution[1] * self.camera.resolution[0] * 3), dtype=np.uint8)
            async with self.lock:
                self.camera.capture(image, 'bgr')
            image = image.reshape((self.camera.resolution[1], self.camera.resolution[0], 3))
            return image
        else:
            return None
