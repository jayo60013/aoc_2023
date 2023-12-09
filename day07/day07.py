from collections import Counter

PART1_ORDER = "23456789TJQKA"
PART2_ORDER = "J23456789TQKA"

CARD_ORDER = PART1_ORDER


class Hand:
    cards: str
    bid: int
    hand: int

    def __init__(self, cards, bid, hand):
        self.cards = cards
        self.bid = bid
        self.hand = hand

    def __lt__(self, other) -> bool:
        if self.hand != other.hand:
            return self.hand < other.hand
        for lhs, rhs in zip(self.cards, other.cards):
            if lhs != rhs:
                return CARD_ORDER.index(lhs) < CARD_ORDER.index(rhs)
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


def adjustCard(cards) -> str:
    if "J" not in cards:
        return cards

    card_counts = Counter(cards)
    if card_counts["J"] == 5:
        return "AAAAA"

    del card_counts["J"]
    most_common, _ = card_counts.most_common(1)[0]
    return cards.replace("J", most_common)


def part1(lines) -> int:
    hands = []
    for line in lines:
        cards, bid = line.split(" ")
        hands.append(Hand(cards, int(bid.strip()), determineHand(cards)))
    hands.sort()
    return sum((i + 1) * hand.bid for i, hand in enumerate(hands))


def part2(lines) -> int:
    hands = []
    for line in lines:
        cards, bid = line.split(" ")
        hands.append(
            Hand(cards, int(bid.strip()), determineHand(adjustCard(cards)))
        )

    hands.sort()
    return sum((i + 1) * hand.bid for i, hand in enumerate(hands))


lines = []
with open("input") as file:
    for line in file:
        lines.append(line)

print(f"Part 1: {part1(lines)}")
print(f"Part 2: {part2(lines)}")
file.close()
