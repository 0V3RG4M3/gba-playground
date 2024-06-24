def main():
    inverse_freq = [
        8013, 7566, 7144, 6742,  # C, C#, D, D#
        6362, 6005, 5666, 5346,  # E, F, F#, G
        5048, 4766, 4499, 4246,  # G#, A, A#, B
    ]

    rates = []
    for i, note in enumerate(range(-24, -24+127-35)):
        ratef = 2048 - (inverse_freq[(note + 144) % 12] / 2 ** (4 + note // 12))
        ratei = int(round(ratef))
        rates.append(ratei)

    rates = [str(r) for r in rates]
    print(f"const PITCH2RATE_MAP: [u16; {len(rates)}] = [{', '.join(rates)}];")


if __name__ == '__main__':
    main()
