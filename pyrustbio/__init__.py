from . import _pyrustbio

def pairwise(query, ref, g_o, g_e):
    score, pretty = _pyrustbio.affine(query, ref, g_o, g_e)
    return score, pretty.split('\n')[0:3]

def levenshtein(a, b):
    return _pyrustbio.levenshtein(a, b)
