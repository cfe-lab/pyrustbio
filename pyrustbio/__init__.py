from . import _pyrustbio

def pairwise(ref, alt, g_o, g_e):
    return _pyrustbio.affine(ref, alt, g_o, g_e)

def levenshtein(ref, alt):
    return _pyrustbio.levenshtein(ref, alt)
