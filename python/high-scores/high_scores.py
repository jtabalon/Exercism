def latest(scores):
    if scores:
        return scores[-1]

def personal_best(scores):
    if scores:
        return max(scores)

def personal_top_three(scores):
    scores = sorted(scores, reverse=True)
    if scores:
        return scores[0:3]
