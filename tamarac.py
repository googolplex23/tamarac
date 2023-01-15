"""
tamarac by Leo Fisher. some UCI code from https://github.com/healeycodes/andoma.
"""
import chess
from engine import tamarac

def talk():
    """
    The main input/output loop.
    This implements a slice of the UCI protocol.
    """
    board = chess.Board()
    depth = 4

    while True:
        msg = input()
        command(depth, board, msg)


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
        print("id name Andoma")  # Andrew/Roma -> And/oma
        print("id author Andrew Healey & Roma Parramore")
        print("uciok")
        return

    if msg == "isready":
        print("readyok")
        return

    if msg == "ucinewgame":
        del tamarac
        engine = tamarac()
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
        _move = engine.search(depth, board)
        print(f"bestmove {_move}")
        return

talk()
        