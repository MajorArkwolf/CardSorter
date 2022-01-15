import json

# Used to strip uneeded fields from Scryfall json to reduce overhead in ram

input_file_location = ""
output_file_location = ""
data = None
remove_keys = ["object", "oracle_id", "uri", "finishes", "scryfall_uri", "multiverse_ids", "mtgo_id", "mtgo_foil_id", "tcgplayer_id", "cardmarket_id", "lang", "highres_image", "image_status", "image_uris", "legalities", "games", "oversized", "set_uri", "set_search_uri", "scryfall_set_uri", "rulings_uri", "prints_search_uri", "card_back_id", "related_uris", "artist_ids", "illustration_id"]

with open(input_file_location, "r", encoding='utf8') as read_file:
    data = json.load(read_file)

for card in data:
    for key in remove_keys:
        card.pop(key, None)

with open(output_file_location, "w", encoding='utf8') as output_file:
    json.dump(data, output_file)
