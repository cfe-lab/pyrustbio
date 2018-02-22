from . import _pyrustbio

def pairwise(query, ref, g_o, g_e):
    score, pretty = _pyrustbio.affine(query, ref, g_o, g_e)
    return score, pretty.split('\n')[0:3]

def levenshtein(a, b):
    return _pyrustbio.levenshtein(a, b)

def align_it(ref, query, g_o, g_e, terminal_p):
    score, pretty = _pyrustbio.affine(query, ref, -1 * g_o, -1 * g_e)
    ref_a, _, query_a = pretty.split('\n')[0:3]
    return ref_a.replace(' ', '-'), query_a.replace(' ', '-'), score
