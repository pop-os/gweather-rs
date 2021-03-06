// Generated by gir (https://github.com/gtk-rs/gir @ 350409c)
// from .. (@ ed21eb4+)
// from ../gir-files (@ 38b7451)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GWEATHER_DISTANCE_UNIT_DEFAULT);
    PRINT_CONSTANT((gint) GWEATHER_DISTANCE_UNIT_INVALID);
    PRINT_CONSTANT((gint) GWEATHER_DISTANCE_UNIT_KM);
    PRINT_CONSTANT((gint) GWEATHER_DISTANCE_UNIT_METERS);
    PRINT_CONSTANT((gint) GWEATHER_DISTANCE_UNIT_MILES);
    PRINT_CONSTANT((guint) GWEATHER_FORMAT_OPTION_DEFAULT);
    PRINT_CONSTANT((guint) GWEATHER_FORMAT_OPTION_NO_CAPITALIZATION);
    PRINT_CONSTANT((guint) GWEATHER_FORMAT_OPTION_SENTENCE_CAPITALIZATION);
    PRINT_CONSTANT((gint) GWEATHER_LOCATION_ADM1);
    PRINT_CONSTANT((gint) GWEATHER_LOCATION_CITY);
    PRINT_CONSTANT((gint) GWEATHER_LOCATION_COUNTRY);
    PRINT_CONSTANT((gint) GWEATHER_LOCATION_DETACHED);
    PRINT_CONSTANT(GWEATHER_LOCATION_ENTRY_H);
    PRINT_CONSTANT((gint) GWEATHER_LOCATION_NAMED_TIMEZONE);
    PRINT_CONSTANT((gint) GWEATHER_LOCATION_REGION);
    PRINT_CONSTANT((gint) GWEATHER_LOCATION_WEATHER_STATION);
    PRINT_CONSTANT((gint) GWEATHER_LOCATION_WORLD);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_DRIZZLE);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_DUST);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_DUSTSTORM);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_DUST_WHIRLS);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_FOG);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_FUNNEL_CLOUD);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_HAIL);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_HAZE);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_ICE_CRYSTALS);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_ICE_PELLETS);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_INVALID);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_LAST);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_MIST);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_NONE);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_RAIN);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_SAND);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_SANDSTORM);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_SMALL_HAIL);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_SMOKE);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_SNOW);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_SNOW_GRAINS);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_SPRAY);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_SQUALL);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_TORNADO);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_UNKNOWN_PRECIPITATION);
    PRINT_CONSTANT((gint) GWEATHER_PHENOMENON_VOLCANIC_ASH);
    PRINT_CONSTANT((gint) GWEATHER_PRESSURE_UNIT_ATM);
    PRINT_CONSTANT((gint) GWEATHER_PRESSURE_UNIT_DEFAULT);
    PRINT_CONSTANT((gint) GWEATHER_PRESSURE_UNIT_HPA);
    PRINT_CONSTANT((gint) GWEATHER_PRESSURE_UNIT_INCH_HG);
    PRINT_CONSTANT((gint) GWEATHER_PRESSURE_UNIT_INVALID);
    PRINT_CONSTANT((gint) GWEATHER_PRESSURE_UNIT_KPA);
    PRINT_CONSTANT((gint) GWEATHER_PRESSURE_UNIT_MB);
    PRINT_CONSTANT((gint) GWEATHER_PRESSURE_UNIT_MM_HG);
    PRINT_CONSTANT((guint) GWEATHER_PROVIDER_ALL);
    PRINT_CONSTANT((guint) GWEATHER_PROVIDER_IWIN);
    PRINT_CONSTANT((guint) GWEATHER_PROVIDER_METAR);
    PRINT_CONSTANT((guint) GWEATHER_PROVIDER_MET_NO);
    PRINT_CONSTANT((guint) GWEATHER_PROVIDER_NONE);
    PRINT_CONSTANT((guint) GWEATHER_PROVIDER_OWM);
    PRINT_CONSTANT((guint) GWEATHER_PROVIDER_YAHOO);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_BLOWING);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_DRIFTING);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_FREEZING);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_HEAVY);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_INVALID);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_LAST);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_LIGHT);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_MODERATE);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_NONE);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_PARTIAL);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_PATCHES);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_SHALLOW);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_SHOWERS);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_THUNDERSTORM);
    PRINT_CONSTANT((gint) GWEATHER_QUALIFIER_VICINITY);
    PRINT_CONSTANT((gint) GWEATHER_SKY_BROKEN);
    PRINT_CONSTANT((gint) GWEATHER_SKY_CLEAR);
    PRINT_CONSTANT((gint) GWEATHER_SKY_FEW);
    PRINT_CONSTANT((gint) GWEATHER_SKY_INVALID);
    PRINT_CONSTANT((gint) GWEATHER_SKY_LAST);
    PRINT_CONSTANT((gint) GWEATHER_SKY_OVERCAST);
    PRINT_CONSTANT((gint) GWEATHER_SKY_SCATTERED);
    PRINT_CONSTANT((gint) GWEATHER_SPEED_UNIT_BFT);
    PRINT_CONSTANT((gint) GWEATHER_SPEED_UNIT_DEFAULT);
    PRINT_CONSTANT((gint) GWEATHER_SPEED_UNIT_INVALID);
    PRINT_CONSTANT((gint) GWEATHER_SPEED_UNIT_KNOTS);
    PRINT_CONSTANT((gint) GWEATHER_SPEED_UNIT_KPH);
    PRINT_CONSTANT((gint) GWEATHER_SPEED_UNIT_MPH);
    PRINT_CONSTANT((gint) GWEATHER_SPEED_UNIT_MS);
    PRINT_CONSTANT((gint) GWEATHER_TEMP_UNIT_CENTIGRADE);
    PRINT_CONSTANT((gint) GWEATHER_TEMP_UNIT_DEFAULT);
    PRINT_CONSTANT((gint) GWEATHER_TEMP_UNIT_FAHRENHEIT);
    PRINT_CONSTANT((gint) GWEATHER_TEMP_UNIT_INVALID);
    PRINT_CONSTANT((gint) GWEATHER_TEMP_UNIT_KELVIN);
    PRINT_CONSTANT(GWEATHER_TIMEZONE_MENU_H);
    PRINT_CONSTANT((gint) GWEATHER_WIND_E);
    PRINT_CONSTANT((gint) GWEATHER_WIND_ENE);
    PRINT_CONSTANT((gint) GWEATHER_WIND_ESE);
    PRINT_CONSTANT((gint) GWEATHER_WIND_INVALID);
    PRINT_CONSTANT((gint) GWEATHER_WIND_LAST);
    PRINT_CONSTANT((gint) GWEATHER_WIND_N);
    PRINT_CONSTANT((gint) GWEATHER_WIND_NE);
    PRINT_CONSTANT((gint) GWEATHER_WIND_NNE);
    PRINT_CONSTANT((gint) GWEATHER_WIND_NNW);
    PRINT_CONSTANT((gint) GWEATHER_WIND_NW);
    PRINT_CONSTANT((gint) GWEATHER_WIND_S);
    PRINT_CONSTANT((gint) GWEATHER_WIND_SE);
    PRINT_CONSTANT((gint) GWEATHER_WIND_SSE);
    PRINT_CONSTANT((gint) GWEATHER_WIND_SSW);
    PRINT_CONSTANT((gint) GWEATHER_WIND_SW);
    PRINT_CONSTANT((gint) GWEATHER_WIND_VARIABLE);
    PRINT_CONSTANT((gint) GWEATHER_WIND_W);
    PRINT_CONSTANT((gint) GWEATHER_WIND_WNW);
    PRINT_CONSTANT((gint) GWEATHER_WIND_WSW);
    return 0;
}
