def two_fer(name=""):
    if name=="":
        return "One for you, one for me."
    else:
        return f"One for {name}, one for me."

if __name__=="__main__":
    two_fer()

