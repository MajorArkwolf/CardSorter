import json
from MTGCard import *

class MTGLibrary:
    def __init__(self, data_file):
        with open(data_file, "r", encoding='utf8') as read_file:
            self.data = json.load(read_file)
            self.Generate_Lookup_Table()

    def Generate_Lookup_Table(self):
        self.name_lookup = {}
        for row in self.data:
            if row["name"] not in self.name_lookup.keys():
                self.name_lookup[row["name"]] = []
            self.name_lookup[row["name"]].append(row)

    def Look_Up_Card(self, card):
        potential_cards = None
        if card.card_name in self.name_lookup.keys():
            potential_cards = self.name_lookup[card.card_name]
        else:
            return None

        if (card.copyright == 0):
            return potential_cards
        else:
            for potential_card in potential_cards:
                date = potential_card['released_at'][0:4]
                if (card.copyright == int(date)):
                    return potential_card
