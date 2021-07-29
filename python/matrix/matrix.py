class Matrix:
    def __init__(self, matrix_string):
        self.matrix_string = matrix_string
        self.split_line = self.matrix_string.split("\n")
        self.rows = [row.split(" ") for row in split_line]
        self.row = []
        self.col = 

    def row(self, index):
        # self.split_line = self.matrix_string.split("\n")
        # self.rows = [row.split(" ") for row in split_line]
        return self.rows[index]
        

    def column(self, index):
        return self.col[index]

        
