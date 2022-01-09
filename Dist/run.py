import git
import subprocess
import os
import os.path
import logging
import sys
import time
import shutil
from pathlib import Path
from dotenv import load_dotenv

logging.basicConfig(filename='runner.log', filemode='a',format='%(asctime)s %(message)s', datefmt='%m/%d/%Y %I:%M:%S %p', level=logging.DEBUG)
logging.info('Began running service...')

load_dotenv()

root_dir = Path(os.getcwd())
ocr_script_path = root_dir / Path("OCRScripts")
operation_program_path = root_dir / Path("Operation")
ocr_program = None
operation_program = None
# Hardcoded this path since cargo relies on relative paths
toml_location = "./Operation/Cargo.toml"

def check_git_for_updates() -> bool:
    repo = git.Repo(root_dir)
    current = repo.head.commit
    repo.remotes.origin.pull()
    if current != repo.head.commit:
        logging.info('Found new updates on git')
        return True
    return False

def pull_updates() -> bool:
    repo = git.Repo(root_dir)
    repo.remotes.origin.pull()

# We should rebuild whenever we get updates to ensure we have the latest changes in our binary.
def build_operation() -> bool:
    return_code = subprocess.run(['cargo', 'build', '--manifest-path', toml_location, '--release'], 
                         stdout=subprocess.PIPE, 
                         universal_newlines=True)
    if return_code == 0:
        return True
    return False

def send_interupt_signal():
    if ocr_program is not None:
        logging.info('Sending SIGINT to OCR Program')
        ocr_program.send_signal(signal.SIGINT)
        ocr_program.wait()
    if operation_program is not None:
        logging.info('Sending SIGINT to Operation Program')
        operation_program.send_signal(signal.SIGINT)
        operation_program.wait()

def copy_system_json() -> bool:
    try: 
        root_file = root_dir
        if os.path.isfile(root_file):
            return True
        operation_file = operation_program_path / Path("system.json")
        if os.path.isfile(operation_file):
            shutil.copy(operation_file, root_dir)
        else:
            return False
    except:
        logging.exception('')
        return False

def main():
    logging.info('Starting main service')
    if check_git_for_updates() == True:
        logging.info('New updates found on launch, pulling and building')
        pull_updates()
        if build_operation() == False:
            logging.info('Build failed, killing runner.')
            return

    if copy_system_json() == False:
        logging.error("Failed to find system.json file and therefor can not proceed")
        return

    ocr_main_script = ocr_script_path / "main.py"
    logging.info('Starting OCR program')
    ocr_program = subprocess.Popen(['python', ocr_main_script], 
                           stdout=subprocess.PIPE,
                           universal_newlines=True)

    logging.info('Starting Operation program')
    operation_program = subprocess.Popen(['cargo', 'run', '--manifest-path', toml_location, '--release'], 
                           stdout=subprocess.PIPE,
                           universal_newlines=True)

    logging.info('Beginning main loop')
    while True:
        if check_git_for_updates() == True:
            logging.info('New updates found in main loop, performing update procedure')
            # perform update here once everything is implemented
            send_interupt_signal()
            break

        ocr_status = ocr_program.poll()

        if ocr_status is not None:
            logging.info('OCR reported a return code, killing services')
            send_interupt_signal()
            break

        operation_status = operation_program.poll()

        if operation_status is not None:
            logging.info('Operation program reported a return code, killing services')
            send_interupt_signal()
            break
        
        # Sleep for 5 seconds to avoid constant polling
        time.sleep(5)
        


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print('Interrupted')
        logging.info('Interrupted signal detected')
        send_interupt_signal()
        try:
            sys.exit(0)
        except SystemExit:
            os._exit(0)