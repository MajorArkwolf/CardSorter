
# Datastore
## Info
This is the schema used to implement tracking of data acquired by the system and helps provide insight into both valid and invalid input.

### Terminology
- PK: Primary Key, implicitly not null
- FK: Foreign Key
- NN: Not null (Lack of this keyword means it can be null)

## Design
### Set
Stores the name of the set allowing cards to be manually sorted afterwards into piles based on ID.
#### Schema
- id: Int, PK
- Name: String, NN (Name of the set)
- Year: Date, NN (The year the set released)

### Cards
Gives each card a unique identifier that is used to track information in the system.
#### Schema
- id: Int, PK
- Name: String, NN (The name of the card)
- Set: Int, FK, NN (Foreign key to the set the card belongs to)

### Stock
Stores a running quanity of the amount of times the given card has been detected.
#### Schema
- id: Int, PK, FK(Cards) (Relates the stock to a given card)
- Quantity: Int, NN (Tracks how many of a given card has been identified)

### Pile
Pile represents the output of the distribution circuit and the end result of a scanned card. These will most likely be Valuable, Unknown, Trash.
#### Schema
- id: Int, PK (Relates to an output pile, limited by the distributor circuit)
- Name: String, NN (Name of the pile for human readability)


### ScannedCard
Represents a log of each card scanned, since price can flucate depending on the day and the data used the output can vary. Picture is a string path to the photo that was parsed. This gives the oppertunity to look back on debugging if required.
#### Schema
- id: Int, PK
- CardID: Int, FK(Cards) (If succesfully discovered then links to a card OR null if unable to determine)
- PileID: Int, FK(Pile), NN (The pile the card was sent to)
- Price: Number (The price of the card it was scanned in at, since this number could vary its important to track, can be null if failed to determine which card it was)

### Debug
Used when a card has been identified as unknown, this is used to help tune/retune the system to help reduce failure rate.
#### Schema
- id: Int, PK
- ScannedCardID: FK(ScannedCard)
- OCROutput: String
- ErrorMessage: String
