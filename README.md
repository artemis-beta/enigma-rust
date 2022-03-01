# Enigma

This is a small application written in python which simulates both the M3 and M4, 3 and 4 rotor variants of the Enigma machine which was utilised by German forces during WWII to encode information.

The code is the third part in a family of enigma simulation codes, the first being the Python version [here](https://github.com/artemis-beta/enigma), and the second a C++ version [here](https://github.com/artemis-beta/enigma-cpp).

The application provides a user interface in which the user can:

* Choose between M4 and M3 variants.
* Set a key to use for encoding.
* Choose any 3 or 4 unique rotors from all 8 variants.
* Set "ringstellung" (internal wire rotation within the chosen rotors).
* Encode a phrase which is automatically encoded and group into the classic 5 character cipher.
