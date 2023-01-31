#engine.py

import chess
import sys
from time import perf_counter_ns
#from chess.engine import PlayResult
from chess.polyglot import zobrist_hash
from collections import Counter

"""
class node():
    def __init__(self, board, parent=None):
        self.parent = parent
        self.board = board
        self.times_visited = 1
    
    def create_best_child:
"""    

class tamarac:
    def __init__(self):
        self.hash_table = {}
        self.quiescent_hash_table = {}
        self.outcome_hash_table = {}
        self.nodes = 0
        self.revisit = 0
        self.prev_guess = 0
        self.tunable_values = [[-10000,-1000,-525,-350,-350,-100,0,100,350,350,525,1000,10000],1000]
        #self.node_table = {}

    
    def search(self, time, board, *args):
        self.nodes = 0
        self.revisit = 0
        #result = self.root_pvs(board,-10000000,10000000,3)
        result = self.iterative_deepening(board,time)
        log = open("log.txt","a")
        #fin_nodes = str(self.revisit)
        log.write("nodes:" + str(self.nodes) + " revisits:" + str(self.revisit) + "\n")
        log.close()
        for entry in list(self.hash_table):
            curr_depth = self.hash_table[entry][1]
            curr_depth = curr_depth - 1
            self.hash_table[entry][1] = curr_depth
            if curr_depth < 0:
                del self.hash_table[entry]
        for entry in list(self.quiescent_hash_table):
            curr_depth = self.quiescent_hash_table[entry][1]
            curr_depth = curr_depth + 1
            self.quiescent_hash_table[entry][1] = curr_depth
            if curr_depth >= 3:
                del self.quiescent_hash_table[entry]
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
    
    #def check_threefold_repetition
                
   
    def evaluate(self, board):  #interrim score function
        weights = self.tunable_values[0] #Scores of various pieces
        board_int = self.convert_to_int(board)
        score = 0
        for x in board_int:
            score = score + (weights[x+6])
        if not board.turn:
            score = -score
       
        return score + len(list(board.legal_moves))
    
    def see_capture(self, board, move):
        piece = board.piece_type_at(move.to_square)
        board.push(move)
        value = self.tunable_values[0][piece+6] - self.see(board,move.to_square)
        board.pop()
        return value
    
    def see(self, board, square):
        value = 0
        if board.is_attacked_by(not board.turn,square):
            pieces = board.attackers(not board.turn,square)
            smallest = 100
            best_square = None
            for atk_square in pieces:
                if board.piece_type_at(atk_square) < smallest:
                    best_square = atk_square
                    smallest = board.piece_type_at(atk_square)
            board.push(board.find_move(best_square, square))
            value = max(0,self.tunable_values[0][board.piece_type_at(square)+6] - self.see(board,square))
            board.pop()
        return value
            
            
    
    def order_moves(self, board):
        captures = []
        non_captures = []
        moves = list(board.legal_moves)
        for move in moves:
            if board.is_capture(move):
                captures.append(move)
            else:
                non_captures.append(move)
        return captures + non_captures
        
    def iterative_deepening(self,board,time):
        starttime = perf_counter_ns()
        firstguess = {1:self.prev_guess}
        depth = 0
        while True:
            depth = depth+1
            result = self.mtdf(board, firstguess[depth], depth)
            print("info depth " + str(depth) + " nodes " + str(self.nodes))
            if depth == 1:
                firstguess[2] = 0-result[0]
            firstguess[depth+2] = result[0]
            
            if perf_counter_ns() - starttime > time:
                #self.prev_guess = 0-result[0]
                return result[1]
  
    
    def mtdf(self, board, firstguess, depth):
        g = firstguess
        upperbound = 10000000
        lowerbound = -10000000
        result = []
        while not lowerbound >= upperbound:
            
            if g == lowerbound:
                beta = g + 1
            else:
                beta = g
            result = self.failsoft_alphabeta(board, beta - 1, beta, 0, depth)
            g = result[0]
            if g < beta:
                upperbound = g
            else:
                lowerbound = g
            print("info lowerbound " + str(lowerbound) + " upperbound " + str(upperbound))
        return [g,result[1]]
    
    """
    
    def pvs(self, board, alpha, beta, depth):
        self.nodes = self.nodes+1
        
        #if board.is_stalemate() or board.is_repetition(): #this should prevent the bot from drawing the game if it's ahead but encourage it if it's losing
        #    return 0
        #if board.is_checkmate():
        #    return 1000000
        
        bestmove = list(board.legal_moves)[0]
        ordered_moves = self.order_moves(board)
        if zobrist_hash(board) in self.hash_table:
            ordered_moves = [self.hash_table[zobrist_hash(board)][2]] + ordered_moves
            #if  self.hash_table[zobrist_hash(board)][1] < depth:#check to make sure we haven't been here before
            #    self.revisit = self.revisit +1
            #    return self.hash_table[zobrist_hash(board)][0]
        
        #==================Actual PVS Algorithm===================#
        
        if depth == 0:
            result = self.quiesce(board, alpha, beta)
            #self.hash_table[zobrist_hash(board)] = [result, depth,5]
            return result
        
        bsearchpv = True
        for move in ordered_moves:
            board.push(move)
            if bsearchpv:
                score = -self.pvs(board,-beta,-alpha,depth-1)
            else:
                score = -self.pvs(board,-alpha-1, -alpha,depth-1)
                if score > alpha:
                    score = -self.pvs(board,-beta,-alpha,depth-1)
            board.pop()
            if score >= beta:
                self.hash_table[zobrist_hash(board)] = [beta,depth,move]
                return beta
            if score > alpha:
                alpha = score
                bestmove = move
                bsearchpv = False
        self.hash_table[zobrist_hash(board)] = [alpha,depth,bestmove] #score,depth,age(starts at 5),PV  
        return alpha
        
        
        # New fail-soft implementation:
        
        board.push(ordered_moves[0])
        bestscore = -self.pvs(board, -beta,-alpha, depth-1)
        board.pop()
        if bestscore > alpha:
            if bestscore >= beta:
                self.hash_table[zobrist_hash(board)] = [bestscore,depth,ordered_moves[0]]
                return bestscore
            alpha = bestscore
        ordered_moves.pop(0)
        for move in ordered_moves:
            board.push(move)
            score = -self.pvs(board, alpha-1, alpha, depth-1)
            if score > alpha and score < beta:
                score = -self.pvs(board,-beta,-alpha, depth-1)
                if score > alpha:
                    alpha = score
            board.pop()
            if score > bestscore:
                if score >= beta:
                    self.hash_table[zobrist_hash(board)] = [score,depth,move]
                    return score
                bestscore = score
                bestmove = move
        self.hash_table[zobrist_hash(board)] = [bestscore,depth,bestmove]
        return bestscore 
        
        
    
        
    
    def root_pvs(self, board, alpha, beta, depth):
        
        ordered_moves = self.order_moves(board)
        
        board.push(ordered_moves[0])
        bestmove = ordered_moves[0]
        bestscore = -self.pvs(board, -beta,-alpha, depth-1)
        board.pop()
        if bestscore > alpha:
            if bestscore >= beta:
                #self.hash_table[zobrist_hash(board)] = [bestscore,depth,ordered_moves[0]]
                return [ordered_moves[0],bestscore]
            alpha = bestscore
        ordered_moves.pop(0)
        for move in ordered_moves:
            board.push(move)
            score = -self.pvs(board, alpha-1, alpha, depth-1)
            if score > alpha and score < beta:
                score = -self.pvs(board,-beta,-alpha, depth-1)
                if score > alpha:
                    alpha = score
            board.pop()
            if score > bestscore:
                if score >= beta:
                    #self.hash_table[zobrist_hash(board)] = [score,depth,move]
                    return [move,score]
                bestscore = score
                bestmove = move
        #self.hash_table[zobrist_hash(board)] = [bestscore,depth,bestmove]
        return [bestmove,bestscore]
    
    """
    def quiesce(self, board, alpha, beta): #quiescence search
    
        
        try:
            bestmove = list(board.legal_moves)[0]
        except:
            outcome = board.outcome()
            if outcome.winner:
                if outcome.winner == board.turn:
                    return 100000000
                else:
                    return -100000000
            return 0

        #if board.can_claim_draw():
        #    return -1250
    
        self.nodes = self.nodes + 1
        """
        
        if zobrist_hash(board) in self.quiescent_hash_table:
            self.revisit = self.revisit +1
            return self.hash_table[zobrist_hash(board)][0]   
        
        """
        #if zobrist_hash(board) in self.hash_table:        #check to make sure we haven't been here before
        #    return self.hash_table[zobrist_hash(board)]
        stand_pat = self.evaluate(board)
        if stand_pat >= beta:
            return beta
        if stand_pat < alpha - 1000: # queen value
            return alpha
        if alpha < stand_pat:
            alpha = stand_pat
        for move in list(board.legal_moves):
            if board.is_capture(move):
                if self.see_capture(board, move) > 0:
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
    def root_alphabeta(self, board, alpha, beta, depth):
        best_move = list(board.legal_moves)[0]
        for move in list(board.legal_moves):
            board.push(move)
            score = -self.alphabeta(board, -beta, -alpha, depth-1)
            board.pop()
            if score >= beta:
                best_move = move
                return [best_move,score]
            if score > alpha:
                alpha = score  
                best_move = move
            #board.pop()
        return [best_move,alpha]
    
    
    
    def alphabeta(self, board, alpha, beta, depth):
        self.nodes = self.nodes+1
        
        if board.is_stalemate() or board.is_repetition(): #this should prevent the bot from drawing the game if it's ahead but encourage it if it's losing
            return 0
        if board.is_checkmate():
            return 1000000
        
        bestmove = list(board.legal_moves)[0]
        ordered_moves = self.order_moves(board)
        if zobrist_hash(board) in self.hash_table:
            ordered_moves = [self.hash_table[zobrist_hash(board)][2]] + ordered_moves
            if  self.hash_table[zobrist_hash(board)][1] < depth:#check to make sure we haven't been here before
                self.revisit = self.revisit +1
                return self.hash_table[zobrist_hash(board)][0]
        
        if depth == 0:
            result = self.quiesce(board, alpha, beta)
            self.hash_table[zobrist_hash(board)] = [result, depth,5]
            return result

        if board.is_stalemate() or board.is_repetition():
            return 0
        for move in list(board.legal_moves):
            board.push(move)
            score = -self.alphabeta(board, -beta, -alpha, depth-1)
            board.pop()
            if score >= beta:
                self.hash_table[zobrist_hash(board)] = [beta,depth,move]
                return beta
            if score > alpha:
                alpha = score
                best_move = move
            board.pop()                
        self.hash_table[zobrist_hash(board)] = [alpha,depth,bestmove] #score,depth,age(starts at 5),PV  
        return alpha   
    
    """
    
    def failsoft_alphabeta(self, board, alpha, beta, depth, maxdepth):
        self.nodes = self.nodes + 1
        
        if depth == maxdepth:
            return [self.quiesce(board, alpha, beta)]
        
        try:
            bestmove = list(board.legal_moves)[0]
        except:
            outcome = board.outcome()
            if outcome.winner:
                if outcome.winner == board.turn:
                    return [100000000]
                else:
                    return [-100000000]
            return [-1250]
        """
        if depth == :
            if board.is_repetition():
                return [-1250]
        """
        
        
        ordered_moves = self.order_moves(board)
        if zobrist_hash(board) in self.hash_table:
            ordered_moves.remove(self.hash_table[zobrist_hash(board)][2])
            ordered_moves = [self.hash_table[zobrist_hash(board)][2]] + ordered_moves
        
        bestscore = -10000000
        for move in ordered_moves:
            board.push(move)
            result = self.failsoft_alphabeta(board,-beta,-alpha,depth+1,maxdepth)
            score = 0 - result[0]
            board.pop()
            if score >= beta:
                self.hash_table[zobrist_hash(board)] = [score,depth,move]
                return [score,move]
            if score > bestscore:
                bestscore = score
                if score > alpha:
                    alpha = score
                bestmove = move
        self.hash_table[zobrist_hash(board)] = [alpha,depth,bestmove]
        return [alpha,bestmove]
        
        
    
