def find_middle_digits(number):
    str_number = str(number)
    nb_size = len(str_number)
    if nb_size % 2 == 0:
        middle_digits = str_number[nb_size//2 - 1:nb_size//2 + 1]
    else:
        middle_digits = str_number[nb_size//2 - 1: nb_size//2 + 2]
    return middle_digits
