# Card Sorting System
## High Level System
### Basic Flow
1. Card/s Enter Tray.
2. Feeder pulls single card into the system.
3. Card transitions into the capture system.
4. Card gets caught by the blocker in the capture system.
5. Capture system determines what type of payload it has and requests a destination.
6. Distribution system alters blocks to move the payload to the correct location.
7. Distribution system notifies capture system that its ready to recieve.
8. Capture system releases the card by lowering the blocker.
9. Capture system raises the block back up.
10. Capture system notifies the feeder its ready for another card.

### Interupt Flow
- No cards are in the tray. (Halt System)
- A card does not enter the capture system. (Halt System)
- A card is jammed in the distribution system (Kill system)
- An expensive card has been found (Notify owner)


## Stages
### Feeder
#### Pre Condition
Cards present in feeder
#### Post Condition
A card succesfully moved into the Capture
#### Sensors
- Feed tray empty sensor
#### Input & Output
- Card feeding rollers

### Capture
#### Pre Condition
Feeder gives one card, face up for analysis
#### Post Condition
Capture system releases the payload
#### Sensor
- Card sensor to know if a card is ready.
#### Input & Output
- Variable block to hold card.

### Distributor
#### Pre Condition
Cards present in Capture system and has been given a destination.
#### Post Condition
A card succesfully transitioned to a container
#### Sensors
- Several Ambient light sensors to detect card transitions
#### Input & Output
- Several redirects based on payloads destination.

### Container
#### Pre Condition
Cards present in distributor system and has a valid a destination.
#### Post Condition
Card is stored into the appropriate containers
#### Sensors
- Container full sensor

## Design Concepts
### Overseer
The overseer will manage the state transitions based on a rule system. Each circuit will be able to register itself and its preconditions and post conditions. The overseer will see to it that notifications on other circuits are passed along the chain.

### Circuit
Each part of the circuit will be modular and inherit from a abstract circuit object. This will allow circuits to have defined state like blocked, running, or waiting while also having pre and post recruitment checks.

### Definitions
All parts of this project should be able to be configured in an json object and should be expandable to via a form of scription language.

## Hardware
- 1x Raspberry Pi
- 1x Compatiable Camera
- 1x Arduino
- 2-4x White LEDs
- 4x Photovoltic sensors
- 3x buttons (kill switch, continue switch, pause switch)
- Several Servos (these will change the distribution paths of the payloads)
### Purpose
#### Raspberry Pi
The Pi will handle the execution of the overall tasks such as moving circuit transitions and handling the card information.
#### Arduino
Servers as the real time operating system handling the sensor management and IO. The kill switch will be wired into the arduino to server as a safety mechanism. 