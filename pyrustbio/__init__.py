from . import _pyrustbio

def pairwise(query, ref, g_o, g_e, match=5, mm=-4):
    score, pretty = _pyrustbio.affine(query, ref, g_o, g_e, match, mm)
    return score, pretty.split('\n')[0:3]

def levenshtein(a, b):
    return _pyrustbio.levenshtein(a, b)

def align_it(ref, query, g_o, g_e, terminal_p):
    score, pretty = _pyrustbio.affine(query, ref, -1 * g_o, -1 * g_e, 5, -4)
    query_a, _, ref_a = pretty.split('\n')[0:3]
    return ref_a.replace(' ', '-'), query_a.replace(' ', '-'), score
