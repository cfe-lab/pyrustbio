from . import _pyrustbio


def pairwise(query, ref, g_o, g_e):
    score, query_a, ref_a = _pyrustbio.affine(query, ref, g_o, g_e)
    return score, query_a, ref_a


def levenshtein(a, b):
    return _pyrustbio.levenshtein(a, b)


def align_it(ref, query, g_o, g_e, terminal_p):
    score, query_a, ref_a = _pyrustbio.affine(query, ref, -1 * g_o, -1 * g_e)
    return ref_a, query_a, score
