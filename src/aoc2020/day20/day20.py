from enum import Enum, unique

@unique
class Side(Enum):
    TOP = 0
    BOTTOM = 1
    LEFT = 2
    RIGHT = 3

class Photo:
    def __init__(self):
        self.tiles = {}

    def add_tile(self, idx, tile):
        self.tiles[idx] = tile

    def get_tiles(self):
        return list(self.tiles.values())

    def get(self, idx):
        return self.tiles[idx]

class Tile:
    def __init__(self, id, top, bot, left, right, data):
        self.id = id
        self.top = top
        self.bot = bot
        self.left = left
        self.right = right
        self.data = data

        self.top_border = None
        self.bot_border = None
        self.left_border = None
        self.right_border = None

    def set_borders(self, other):
        my_sides = self.get_sides()
        other_sides = other.get_sides()

        found = False
        for s in my_sides:
            for o in other_sides:
                if s[0] == o[0] or s[0][::-1] == o[0]:
                    self.set_border(other.id, s[1])
                    other.rotate_and_set(self.id, o[1], s[1], s[0] == o[0])
                    return other.id
        return None

    def get_sides(self):
        sides = []
        sides.append((self.top, Side.TOP))
        sides.append((self.bot, Side.BOTTOM))
        sides.append((self.left, Side.LEFT))
        sides.append((self.right, Side.RIGHT))
        return sides

    def count_borders(self):
        sum = 0
        if self.top_border:
            sum += 1
        if self.bot_border:
            sum += 1
        if self.left_border:
            sum += 1
        if self.right_border:
            sum += 1

        return sum

    def set_border(self, id, side):
        if side == Side.TOP:
            self.top_border = id
        elif side == Side.BOTTOM:
            self.bot_border = id
        elif side == Side.LEFT:
            self.left_border = id
        elif side == Side.RIGHT:
            self.right_border = id

    def get_border(self, side):
        if side == Side.TOP:
            return self.top_border
        elif side == Side.BOTTOM:
            return self.bot_border
        elif side == Side.LEFT:
            return self.left_border
        elif side == Side.RIGHT:
            return self.right_border

    # We've found a match, rotate/flip ourself to line up with them
    def rotate_and_set(self, id, curr_side, their_side, rev):
        want_side = reverse_side(their_side)

        if curr_side == Side.TOP:
            if want_side == Side.TOP:
                if rev:
                    self.flip_horz()
            elif want_side == Side.BOTTOM:
                self.rot_180()
                if not rev:
                    self.flip_horz()
            elif want_side == Side.LEFT:
                self.rot_270()
                if not rev:
                    self.flip_vert()
            elif want_side == Side.RIGHT:
                self.rot_90()
                if rev:
                    self.flip_vert()
        elif curr_side == Side.BOTTOM:
            if want_side == Side.TOP:
                self.rot_180()
                if not rev:
                    self.flip_horz()
            elif want_side == Side.BOTTOM:
                if rev:
                    self.flip_horz()
            elif want_side == Side.LEFT:
                self.rot_270()
                if rev:
                    self.flip_vert()
            elif want_side == Side.RIGHT:
                self.rot_90()
                if not rev:
                    self.flip_vert()
        elif curr_side == Side.LEFT:
            if want_side == Side.TOP:
                self.rot_90()
                if not rev:
                    self.flip_horz()
            elif want_side == Side.BOTTOM:
                self.rot_270()
                if rev:
                    self.flip_horz()
            elif want_side == Side.LEFT:
                if rev:
                    self.flip_vert()
            elif want_side == Side.RIGHT:
                self.rot_180()
                if not rev:
                    self.flip_vert()
        elif curr_side == Side.RIGHT:
            if want_side == Side.TOP:
                self.rot_270()
                if rev:
                    self.flip_horz()
            elif want_side == Side.BOTTOM:
                self.rot_90()
                if not rev:
                    self.flip_horz()
            elif want_side == Side.LEFT:
                self.rot_180()
                if not rev:
                    self.flip_vert()
            elif want_side == Side.RIGHT:
                if rev:
                    self.flip_vert()

    def rot_90(self):
        new_topb = self.left_border
        new_leftb = self.bot_border
        new_botb = self.right_border
        new_rightb = self.top_border
        self.top_border = new_topb
        self.bot_border = new_botb
        self.left_border = new_leftb
        self.right_border = new_rightb

        new_top = self.left[::-1]
        new_left = self.bot
        new_bot = self.right[::-1]
        new_right = self.top
        self.top = new_top
        self.left = new_left
        self.bot = new_bot
        self.right = new_right

    def rot_180(self):
        self.top_border, self.bot_border = self.bot_border, self.top_border
        self.left_border, self.right_border = self.right_border, self.left_border
        new_top = self.bot[::-1]
        new_bot = self.top[::-1]
        new_left = self.left[::-1]
        new_right = self.right[::-1]
        self.top = new_top
        self.left = new_left
        self.bot = new_bot
        self.right = new_right

    def rot_270(self):
        new_topb = self.right_border
        new_leftb = self.top_border
        new_botb = self.left_border
        new_rightb = self.bot_border
        self.top_border = new_topb
        self.bot_border = new_botb
        self.left_border = new_leftb
        self.right_border = new_rightb

        new_left = self.top[::-1]
        new_bot = self.left
        new_right = self.bot[::-1]
        new_top = self.right
        self.top = new_top
        self.left = new_left
        self.bot = new_bot
        self.right = new_right

    # Flips across the Y axis
    def flip_horz(self):
        self.left_border, self.right_border = self.right_border, self.left_border
        self.left, self.right = self.right, self.left
        self.top = self.top[::-1]
        self.bot = self.bot[::-1]

    # Flips across the X axis
    def flip_vert(self):
        self.top_border, self.bot_border = self.bot_border, self.top_border
        self.top, self.bot = self.bot, self.top
        self.left = self.left[::-1]
        self.right = self.right[::-1]

def main():
    with open("test.txt") as f:
        lines = f.read()
        raw_tiles = lines.split("\n\n")
        photo = parse_tiles(raw_tiles)
        # part1(photo.get_tiles())
        part2(photo)

def part1(tiles):
    corners = find_corners(tiles)

    output = 1
    for c in corners:
        output *= int(c)

    print(output)

def part2(photo):
    arrange_tiles(photo)
    corners = find_corners(photo.get_tiles())
    curr = corners[0]
    print(curr)
    while curr:
        curr = photo.get(curr).right_border

def parse_tiles(raw_tiles):
    tiles = Photo()
    for t in raw_tiles:
        lines = t.rstrip().split('\n')
        idx = lines[0][5:-1]
        top = lines[1]
        bot = lines[-1]
        left = ""
        right = ""
        for i in range(1, len(lines)):
            left += lines[i][0]
            right += lines[i][-1]

        img = '\n'.join(lines[1:])
        tile = Tile(idx, top, bot, left, right, img)
        tiles.add_tile(idx, tile)
    return tiles

def find_corners(tiles):
    corners = []
    for my_tile in tiles:
        for other_tile in tiles:
            if my_tile != other_tile:
                my_tile.set_borders(other_tile)

        if my_tile.count_borders() == 2:
            corners.append(my_tile.id)

    return corners

def arrange_tiles(photo):
    tiles = photo.get_tiles()
    queue = [tiles[0].id]
    while queue:
        curr_id = queue.pop(0)
        curr_tile = photo.get(curr_id)
        for other_tile in tiles:
            if curr_tile != other_tile:
                id_match = curr_tile.set_borders(other_tile)
                if id_match and id_match not in queue:
                    queue.append(id_match)


def reverse_side(side):
    if side == Side.TOP:
        return Side.BOTTOM
    elif side == Side.BOTTOM:
        return Side.TOP
    elif side == Side.LEFT:
        return Side.RIGHT
    elif side == Side.RIGHT:
        return Side.LEFT

if __name__ == "__main__":
    main()
