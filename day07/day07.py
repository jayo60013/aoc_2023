CARD_ORDER = {
    "A": 12,
    "K": 11,
    "Q": 10,
    "J": 9,
    "T": 8,
    "9": 7,
    "8": 6,
    "7": 5,
    "6": 4,
    "5": 3,
    "4": 2,
    "3": 1,
    "2": 0,
}


class Hand:
    cards: str
    bid: int
    hand: int

    def __init__(self, cards, bid):
        self.cards = cards
        self.bid = bid
        self.hand = determineHand(cards)

    def __lt__(self, other) -> bool:
        if self.hand != other.hand:
            return self.hand < other.hand
        for lhs, rhs in zip(self.cards, other.cards):
            if lhs != rhs:
                return CARD_ORDER[lhs] < CARD_ORDER[rhs]
        return self.bid < other.bid


def determineHand(cards) -> int:
    t = list(map(cards.count, cards))
    if 5 in t:
        return 6
    elif 4 in t:
        return 5
    elif 3 in t and 2 in t:
        return 4
    elif 3 in t:
        return 3
    elif t.count(2) == 4:
        return 2
    elif 2 in t:
        return 1
    else:
        return 0


def part1(file) -> int:
    hands = []
    for line in file.readlines():
        cards, bid = line.split(" ")
        hands.append(Hand(cards, int(bid.strip())))
    hands.sort()
    return sum((i + 1) * hand.bid for i, hand in enumerate(hands))


file = open("input")
print(f"Part 1: {part1(file)}")
