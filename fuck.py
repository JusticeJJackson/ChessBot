def print_binary_with_one_indices_reversed(binary_string):
    """
    Prints each bit of the binary string and beneath each '1' bit,
    prints its index in reverse order. '0' bits have blank spaces beneath them.
    
    Args:
        binary_string (str): The binary string to process.
    """
    # Validate that the input contains only binary digits
    if not all(bit in "01" for bit in binary_string):
        print("Error: Input must be a binary string containing only '0' and '1'.")
        return

    # Define the width for each column to ensure alignment
    width = 3

    # Generate the line of bits with fixed width
    bits_line = ''.join(f"{bit:^{width}}" for bit in binary_string)
    print(bits_line)

    # Calculate the reversed indices
    reversed_indices = list(range(len(binary_string)-1, -1, -1))

    # Generate the line of indices for '1' bits only, in reverse order
    indices_line = ''.join(
        f"{reversed_indices[i]:^{width}}" if bit == '1' else ' ' * width 
        for i, bit in enumerate(binary_string)
    )
    print(indices_line)


if __name__ == "__main__":
    # Prompt the user for a binary string input
    binary_input = input("Enter a binary string: ").strip()
    # Call the function to print bits and their '1' indices in reverse order
    print_binary_with_one_indices_reversed(binary_input)