#tamarac.py

import chess
import sys
from chess.engine import PlayResult
from chess.polyglot import zobrist_hash

"""
class node():
    def __init__(self, board, parent=None):
        self.parent = parent
        self.board = board
        self.times_visited = 1
    
    def create_best_child:
"""    

class engine:
    def __init__(self):
        self.hash_table = {}
        #self.quiescent_hash_table = {}
        self.nodes = 0
        self.revisit = 0
        #self.node_table = {}

    
    def search(self, board, *args):
        self.nodes = 0
        self.revisit = 0
        result = PlayResult(self.root_pvs(board, -10000000, 10000000, 4), None)
        log = open("log.txt","a")
        #fin_nodes = str(self.revisit)
        log.write("nodes:" + str(self.nodes) + " revisits:" + str(self.revisit) + "\n")
        log.close()
        for entry in list(self.hash_table):
            depth = self.hash_table[entry][1]
            depth = depth + 1
            self.hash_table[entry][1] = depth
            if depth >= 4:
                del self.hash_table[entry]
        return result
    
    def convert_to_int(self, board):
        l = [None] * 64
        for sq in chess.scan_reversed(board.occupied_co[chess.WHITE]):  # Check if white
            l[sq] = board.piece_type_at(sq)
        for sq in chess.scan_reversed(board.occupied_co[chess.BLACK]):  # Check if black
            l[sq] = -board.piece_type_at(sq)
        return [0 if v is None else v for v in l]
    
    def legal_captures(self, board):
        captures = []
        for move in list(board.legal_moves):
            if board.is_capture(move):
                captures.append(move)
        return captures
                
   
    def evaluate(self, board):  #interrim score function
        weights = [-10000,-1000,-525,-350,-350,-100,0,100,350,350,525,1000,10000] #Scores of various pieces
        board_int = self.convert_to_int(board)
        score = 0
        for x in board_int:
            score = score + (weights[x+6])
        if not board.turn:
            score = -score
       
        return score + len(list(board.legal_moves))
    
    
    def pvs(self, board, alpha, beta, depth):
        self.nodes = self.nodes+1
        
        
        if zobrist_hash(board) in self.hash_table:
            if  self.hash_table[zobrist_hash(board)][1] <= depth:#check to make sure we haven't been here before
                self.revisit = self.revisit +1
                return self.hash_table[zobrist_hash(board)][0]
        
        if depth == 0:
            result = self.evaluate(board)
            self.hash_table[zobrist_hash(board)] = [result, depth,5]
            return result
        if board.is_stalemate() or board.is_repetition(): #this should prevent the bot from drawing the game if it's ahead but encourage it if it's losing
            return 0
        bsearchpv = True
        for move in list(board.legal_moves):
            board.push(move)
            if bsearchpv:
                score = -self.pvs(board,-beta,-alpha,depth-1)
            else:
                score = -self.pvs(board,-alpha-1, -alpha,depth-1)
                if score > alpha:
                    score = -self.pvs(board,-beta,-alpha,depth-1)
            board.pop()
            if score >= beta:
                self.hash_table[zobrist_hash(board)] = [beta,depth,5]
                return beta
            if score > alpha:
                alpha = score
                bsearchpv = False
        self.hash_table[zobrist_hash(board)] = [alpha,depth,5]
        return alpha
        
    
    def root_pvs(self, board, alpha, beta, depth):
        
        bsearchpv = True
        bestmove = list(board.legal_moves)[0]
        for move in list(board.legal_moves):
            board.push(move)
            if bsearchpv:
                score = -self.pvs(board,-beta,-alpha,depth-1)
            else:
                score = -self.pvs(board,-alpha-1, -alpha,depth-1)
                if score > alpha:
                    score = -self.pvs(board,-beta,-alpha,depth-1)
            board.pop()
            if score >= beta:
                return move
            if score > alpha:
                alpha = score
                bestmove = move
                bsearchpv = False
        return bestmove
    
    def quiesce(self, board, alpha, beta): #quiescence search
        self.nodes = self.nodes + 1
        #if zobrist_hash(board) in self.hash_table:        #check to make sure we haven't been here before
        #    return self.hash_table[zobrist_hash(board)]
        stand_pat = self.evaluate(board)
        if stand_pat >= beta:
            return beta
        if alpha < stand_pat:
            alpha = stand_pat
        for move in list(board.legal_moves):
            if board.is_capture(move):
                board.push(move)
                score = -self.quiesce(board, -beta, -alpha)
                if score >= beta:
                    board.pop()
                    return beta
                if score > alpha:
                    alpha = score
                board.pop()  
        return alpha
        
    
    """
    
    def negamax(self, board, depth):
        if depth == 0:
            return self.evaluate(board)
        if board.is_stalemate() or board.is_repetition():
            return 0
        max = -1000000
        for move in list(board.legal_moves):
            board.push(move)
            score = -self.negamax(board, depth-1)
            if score > max:
                max = score
            board.pop()
        return max
   
    def root_negamax(self, board, depth):
        max = -1000000
        for move in list(board.legal_moves):
            board.push(move)
            score = -self.negamax(board, depth-1)
            #best_move = list(board.legal_moves)[0]
            if score > max:
                max = score
                best_move = move
            board.pop()
        try:
            return best_move
        except:
            return list(board.legal_moves)[0]
    """
    """
    
    def root_alphabeta(self, board, alpha, beta, depth):
        for move in list(board.legal_moves):
            board.push(move)
            score = -self.alphabeta(board, -beta, -alpha, depth-1)
            if score >= beta:
                best_move = move
                board.pop()
                return best_move
            if score > alpha:
                alpha = score  
                best_move = move
            board.pop()
        try:
            return best_move
        except:
            return list(board.legal_moves)[0]
    
    
    
    def alphabeta(self, board, alpha, beta, depth):
        if depth == 0:
            return self.evaluate(board)
        if board.is_stalemate() or board.is_repetition():
            return 0
        for move in list(board.legal_moves):
            board.push(move)
            score = -self.alphabeta(board, -beta, -alpha, depth-1)
            if score >= beta:
                board.pop()
                return beta
            if score > alpha:
                alpha = score
            board.pop()                
        return alpha    
    
    """
    