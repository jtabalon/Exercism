def latest(scores):
    if not scores:
        raise ValueError("Expected non-empty sequence")
    return scores[-1]

def personal_best(scores):
    if not scores:
        raise ValueError("Expected non-empty sequence")
    return max(scores)

def personal_top_three(scores):
    if not scores:
        raise ValueError("Expected non-empty sequence")
    scores = sorted(scores, reverse=True)
    return scores[:3]
