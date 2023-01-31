"""
tamarac by Leo Fisher. some UCI code from https://github.com/healeycodes/andoma.
"""
import chess
from engine import tamarac
import argparse





def command(depth: int, board: chess.Board, msg: str):
    """
    Accept UCI commands and respond.
    The board state is also updated.
    """
    msg = msg.strip()
    tokens = msg.split(" ")
    while "" in tokens:
        tokens.remove("")

    if msg == "quit":
        sys.exit()

    if msg == "uci":
        print("id name tamarac")
        print("id author Leo Fisher")
        print("uciok")
        return

    if msg == "isready":
        print("readyok")
        return

    if msg == "ucinewgame":
        #del engine
        return

    if msg.startswith("position"):
        if len(tokens) < 2:
            return

        # Set starting position
        if tokens[1] == "startpos":
            board.reset()
            moves_start = 2
        elif tokens[1] == "fen":
            fen = " ".join(tokens[2:8])
            board.set_fen(fen)
            moves_start = 8
        else:
            return

        # Apply moves
        if len(tokens) <= moves_start or tokens[moves_start] != "moves":
            return

        for move in tokens[(moves_start+1):]:
            board.push_uci(move)

    if msg == "d":
        # Non-standard command, but supported by Stockfish and helps debugging
        print(board)
        print(board.fen())

    if msg[0:2] == "go":
        _move = eng.search(time, board)
        print(f"bestmove {_move}")
        return

def get_time() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--time", default=5000000000, help="time to search for a move, in nanonseconds. (default: 5000000000)")
    args = parser.parse_args()
    return max([1, int(args.time)])


board = chess.Board()
time = get_time()
eng = tamarac()

while True:
        msg = input()
        command(time, board, msg)
    

        
