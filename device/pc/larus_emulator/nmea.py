import datetime
import math

def gprmc(row):
    return create_nmea_sentence(
        "$GPRMC",
        f"{row['hour']:02.0f}{row['minute']:02.0f}{row['second']:02.0f}.{row['nanoseconds']:09.0f}"[:-7],
        'A',
        lat(row['Lat']),
        lon(row['Long']),
        f"{row['speed GNSS']*0.539957:0.1f}",
        track(row['track GNSS']),
        f"{row['day']:02}{row['month']:02}{row['year']:02}",
        '','','A'
    )

def gpgga(row):
    return create_nmea_sentence(
        "$GPGGA",
        f"{row['hour']:02}{row['minute']:02}{row['second']:02}.{row['nanoseconds']:09}"[:-7],
        lat(row['Lat']),
        lon(row['Long']),
        f"{row['sat fix type']}",
        f"{row['sat number']}",
        "1.0",
    )

def create_nmea_sentence(*args):
    result = ','.join(args)
    cs = 0
    for c in result[1:]:
        cs ^= ord(c)
    return bytes(f"{result}*{cs:02X}\r\n", "ascii")

def lat(coord: float) -> str:
    sign = 'S'
    if coord > 0.0:
        sign = 'N'
    fract, deg = math.modf(abs(coord)*180.0/math.pi)
    print(deg, fract)
    return f"{deg:0.0f}{fract*60.0:08.5f},{sign}"

def lon(coord: float) -> str:
    return lat(coord).replace('N', 'E').replace('S', 'W')

def track(track: float) -> str:
    if track < 0.0:
        track += 2*math.pi
    return f"{track*180.0/math.pi:0.1f}"


