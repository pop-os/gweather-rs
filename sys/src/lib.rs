// Generated by gir (https://github.com/gtk-rs/gir @ 350409c)
// from .. (@ ???)
// from ../gir-files (@ 38b7451)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use gdk_pixbuf_sys as gdk_pixbuf;
use gio_sys as gio;
use glib_sys as glib;
use gobject_sys as gobject;
use gtk_sys as gtk;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Aliases
pub type GWeatherMoonLatitude = c_double;
pub type GWeatherMoonPhase = c_double;

// Enums
pub type GWeatherConditionPhenomenon = c_int;
pub const GWEATHER_PHENOMENON_INVALID: GWeatherConditionPhenomenon = -1;
pub const GWEATHER_PHENOMENON_NONE: GWeatherConditionPhenomenon = 0;
pub const GWEATHER_PHENOMENON_DRIZZLE: GWeatherConditionPhenomenon = 1;
pub const GWEATHER_PHENOMENON_RAIN: GWeatherConditionPhenomenon = 2;
pub const GWEATHER_PHENOMENON_SNOW: GWeatherConditionPhenomenon = 3;
pub const GWEATHER_PHENOMENON_SNOW_GRAINS: GWeatherConditionPhenomenon = 4;
pub const GWEATHER_PHENOMENON_ICE_CRYSTALS: GWeatherConditionPhenomenon = 5;
pub const GWEATHER_PHENOMENON_ICE_PELLETS: GWeatherConditionPhenomenon = 6;
pub const GWEATHER_PHENOMENON_HAIL: GWeatherConditionPhenomenon = 7;
pub const GWEATHER_PHENOMENON_SMALL_HAIL: GWeatherConditionPhenomenon = 8;
pub const GWEATHER_PHENOMENON_UNKNOWN_PRECIPITATION: GWeatherConditionPhenomenon = 9;
pub const GWEATHER_PHENOMENON_MIST: GWeatherConditionPhenomenon = 10;
pub const GWEATHER_PHENOMENON_FOG: GWeatherConditionPhenomenon = 11;
pub const GWEATHER_PHENOMENON_SMOKE: GWeatherConditionPhenomenon = 12;
pub const GWEATHER_PHENOMENON_VOLCANIC_ASH: GWeatherConditionPhenomenon = 13;
pub const GWEATHER_PHENOMENON_SAND: GWeatherConditionPhenomenon = 14;
pub const GWEATHER_PHENOMENON_HAZE: GWeatherConditionPhenomenon = 15;
pub const GWEATHER_PHENOMENON_SPRAY: GWeatherConditionPhenomenon = 16;
pub const GWEATHER_PHENOMENON_DUST: GWeatherConditionPhenomenon = 17;
pub const GWEATHER_PHENOMENON_SQUALL: GWeatherConditionPhenomenon = 18;
pub const GWEATHER_PHENOMENON_SANDSTORM: GWeatherConditionPhenomenon = 19;
pub const GWEATHER_PHENOMENON_DUSTSTORM: GWeatherConditionPhenomenon = 20;
pub const GWEATHER_PHENOMENON_FUNNEL_CLOUD: GWeatherConditionPhenomenon = 21;
pub const GWEATHER_PHENOMENON_TORNADO: GWeatherConditionPhenomenon = 22;
pub const GWEATHER_PHENOMENON_DUST_WHIRLS: GWeatherConditionPhenomenon = 23;
pub const GWEATHER_PHENOMENON_LAST: GWeatherConditionPhenomenon = 24;

pub type GWeatherConditionQualifier = c_int;
pub const GWEATHER_QUALIFIER_INVALID: GWeatherConditionQualifier = -1;
pub const GWEATHER_QUALIFIER_NONE: GWeatherConditionQualifier = 0;
pub const GWEATHER_QUALIFIER_VICINITY: GWeatherConditionQualifier = 1;
pub const GWEATHER_QUALIFIER_LIGHT: GWeatherConditionQualifier = 2;
pub const GWEATHER_QUALIFIER_MODERATE: GWeatherConditionQualifier = 3;
pub const GWEATHER_QUALIFIER_HEAVY: GWeatherConditionQualifier = 4;
pub const GWEATHER_QUALIFIER_SHALLOW: GWeatherConditionQualifier = 5;
pub const GWEATHER_QUALIFIER_PATCHES: GWeatherConditionQualifier = 6;
pub const GWEATHER_QUALIFIER_PARTIAL: GWeatherConditionQualifier = 7;
pub const GWEATHER_QUALIFIER_THUNDERSTORM: GWeatherConditionQualifier = 8;
pub const GWEATHER_QUALIFIER_BLOWING: GWeatherConditionQualifier = 9;
pub const GWEATHER_QUALIFIER_SHOWERS: GWeatherConditionQualifier = 10;
pub const GWEATHER_QUALIFIER_DRIFTING: GWeatherConditionQualifier = 11;
pub const GWEATHER_QUALIFIER_FREEZING: GWeatherConditionQualifier = 12;
pub const GWEATHER_QUALIFIER_LAST: GWeatherConditionQualifier = 13;

pub type GWeatherDistanceUnit = c_int;
pub const GWEATHER_DISTANCE_UNIT_INVALID: GWeatherDistanceUnit = 0;
pub const GWEATHER_DISTANCE_UNIT_DEFAULT: GWeatherDistanceUnit = 1;
pub const GWEATHER_DISTANCE_UNIT_METERS: GWeatherDistanceUnit = 2;
pub const GWEATHER_DISTANCE_UNIT_KM: GWeatherDistanceUnit = 3;
pub const GWEATHER_DISTANCE_UNIT_MILES: GWeatherDistanceUnit = 4;

pub type GWeatherLocationLevel = c_int;
pub const GWEATHER_LOCATION_WORLD: GWeatherLocationLevel = 0;
pub const GWEATHER_LOCATION_REGION: GWeatherLocationLevel = 1;
pub const GWEATHER_LOCATION_COUNTRY: GWeatherLocationLevel = 2;
pub const GWEATHER_LOCATION_ADM1: GWeatherLocationLevel = 3;
pub const GWEATHER_LOCATION_CITY: GWeatherLocationLevel = 4;
pub const GWEATHER_LOCATION_WEATHER_STATION: GWeatherLocationLevel = 5;
pub const GWEATHER_LOCATION_DETACHED: GWeatherLocationLevel = 6;
pub const GWEATHER_LOCATION_NAMED_TIMEZONE: GWeatherLocationLevel = 7;

pub type GWeatherPressureUnit = c_int;
pub const GWEATHER_PRESSURE_UNIT_INVALID: GWeatherPressureUnit = 0;
pub const GWEATHER_PRESSURE_UNIT_DEFAULT: GWeatherPressureUnit = 1;
pub const GWEATHER_PRESSURE_UNIT_KPA: GWeatherPressureUnit = 2;
pub const GWEATHER_PRESSURE_UNIT_HPA: GWeatherPressureUnit = 3;
pub const GWEATHER_PRESSURE_UNIT_MB: GWeatherPressureUnit = 4;
pub const GWEATHER_PRESSURE_UNIT_MM_HG: GWeatherPressureUnit = 5;
pub const GWEATHER_PRESSURE_UNIT_INCH_HG: GWeatherPressureUnit = 6;
pub const GWEATHER_PRESSURE_UNIT_ATM: GWeatherPressureUnit = 7;

pub type GWeatherSky = c_int;
pub const GWEATHER_SKY_INVALID: GWeatherSky = -1;
pub const GWEATHER_SKY_CLEAR: GWeatherSky = 0;
pub const GWEATHER_SKY_BROKEN: GWeatherSky = 1;
pub const GWEATHER_SKY_SCATTERED: GWeatherSky = 2;
pub const GWEATHER_SKY_FEW: GWeatherSky = 3;
pub const GWEATHER_SKY_OVERCAST: GWeatherSky = 4;
pub const GWEATHER_SKY_LAST: GWeatherSky = 5;

pub type GWeatherSpeedUnit = c_int;
pub const GWEATHER_SPEED_UNIT_INVALID: GWeatherSpeedUnit = 0;
pub const GWEATHER_SPEED_UNIT_DEFAULT: GWeatherSpeedUnit = 1;
pub const GWEATHER_SPEED_UNIT_MS: GWeatherSpeedUnit = 2;
pub const GWEATHER_SPEED_UNIT_KPH: GWeatherSpeedUnit = 3;
pub const GWEATHER_SPEED_UNIT_MPH: GWeatherSpeedUnit = 4;
pub const GWEATHER_SPEED_UNIT_KNOTS: GWeatherSpeedUnit = 5;
pub const GWEATHER_SPEED_UNIT_BFT: GWeatherSpeedUnit = 6;

pub type GWeatherTemperatureUnit = c_int;
pub const GWEATHER_TEMP_UNIT_INVALID: GWeatherTemperatureUnit = 0;
pub const GWEATHER_TEMP_UNIT_DEFAULT: GWeatherTemperatureUnit = 1;
pub const GWEATHER_TEMP_UNIT_KELVIN: GWeatherTemperatureUnit = 2;
pub const GWEATHER_TEMP_UNIT_CENTIGRADE: GWeatherTemperatureUnit = 3;
pub const GWEATHER_TEMP_UNIT_FAHRENHEIT: GWeatherTemperatureUnit = 4;

pub type GWeatherWindDirection = c_int;
pub const GWEATHER_WIND_INVALID: GWeatherWindDirection = -1;
pub const GWEATHER_WIND_VARIABLE: GWeatherWindDirection = 0;
pub const GWEATHER_WIND_N: GWeatherWindDirection = 1;
pub const GWEATHER_WIND_NNE: GWeatherWindDirection = 2;
pub const GWEATHER_WIND_NE: GWeatherWindDirection = 3;
pub const GWEATHER_WIND_ENE: GWeatherWindDirection = 4;
pub const GWEATHER_WIND_E: GWeatherWindDirection = 5;
pub const GWEATHER_WIND_ESE: GWeatherWindDirection = 6;
pub const GWEATHER_WIND_SE: GWeatherWindDirection = 7;
pub const GWEATHER_WIND_SSE: GWeatherWindDirection = 8;
pub const GWEATHER_WIND_S: GWeatherWindDirection = 9;
pub const GWEATHER_WIND_SSW: GWeatherWindDirection = 10;
pub const GWEATHER_WIND_SW: GWeatherWindDirection = 11;
pub const GWEATHER_WIND_WSW: GWeatherWindDirection = 12;
pub const GWEATHER_WIND_W: GWeatherWindDirection = 13;
pub const GWEATHER_WIND_WNW: GWeatherWindDirection = 14;
pub const GWEATHER_WIND_NW: GWeatherWindDirection = 15;
pub const GWEATHER_WIND_NNW: GWeatherWindDirection = 16;
pub const GWEATHER_WIND_LAST: GWeatherWindDirection = 17;

// Constants
pub const GWEATHER_LOCATION_ENTRY_H: c_int = 1;
pub const GWEATHER_TIMEZONE_MENU_H: c_int = 1;

// Flags
pub type GWeatherFormatOptions = c_uint;
pub const GWEATHER_FORMAT_OPTION_DEFAULT: GWeatherFormatOptions = 0;
pub const GWEATHER_FORMAT_OPTION_SENTENCE_CAPITALIZATION: GWeatherFormatOptions = 1;
pub const GWEATHER_FORMAT_OPTION_NO_CAPITALIZATION: GWeatherFormatOptions = 2;

pub type GWeatherProvider = c_uint;
pub const GWEATHER_PROVIDER_NONE: GWeatherProvider = 0;
pub const GWEATHER_PROVIDER_METAR: GWeatherProvider = 1;
pub const GWEATHER_PROVIDER_IWIN: GWeatherProvider = 4;
pub const GWEATHER_PROVIDER_YAHOO: GWeatherProvider = 8;
pub const GWEATHER_PROVIDER_MET_NO: GWeatherProvider = 16;
pub const GWEATHER_PROVIDER_OWM: GWeatherProvider = 32;
pub const GWEATHER_PROVIDER_ALL: GWeatherProvider = 61;

// Callbacks
pub type GWeatherFilterFunc = Option<unsafe extern "C" fn(*mut GWeatherLocation, gpointer) -> gboolean>;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GWeatherConditions {
    pub significant: gboolean,
    pub phenomenon: GWeatherConditionPhenomenon,
    pub qualifier: GWeatherConditionQualifier,
}

impl ::std::fmt::Debug for GWeatherConditions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GWeatherConditions @ {:p}", self))
         .field("significant", &self.significant)
         .field("phenomenon", &self.phenomenon)
         .field("qualifier", &self.qualifier)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GWeatherInfoClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for GWeatherInfoClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GWeatherInfoClass @ {:p}", self))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct GWeatherLocation(c_void);

impl ::std::fmt::Debug for GWeatherLocation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GWeatherLocation @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GWeatherLocationEntryClass {
    pub parent_class: gtk::GtkSearchEntryClass,
}

impl ::std::fmt::Debug for GWeatherLocationEntryClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GWeatherLocationEntryClass @ {:p}", self))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _GWeatherLocationEntryPrivate(c_void);

pub type GWeatherLocationEntryPrivate = *mut _GWeatherLocationEntryPrivate;

#[repr(C)]
pub struct GWeatherTimezone(c_void);

impl ::std::fmt::Debug for GWeatherTimezone {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GWeatherTimezone @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GWeatherTimezoneMenuClass {
    pub parent_class: gtk::GtkComboBoxClass,
}

impl ::std::fmt::Debug for GWeatherTimezoneMenuClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GWeatherTimezoneMenuClass @ {:p}", self))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

// Classes
#[repr(C)]
pub struct GWeatherInfo(c_void);

impl ::std::fmt::Debug for GWeatherInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GWeatherInfo @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GWeatherLocationEntry {
    pub parent: gtk::GtkSearchEntry,
    pub priv_: *mut GWeatherLocationEntryPrivate,
}

impl ::std::fmt::Debug for GWeatherLocationEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GWeatherLocationEntry @ {:p}", self))
         .field("parent", &self.parent)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GWeatherTimezoneMenu {
    pub parent: gtk::GtkComboBox,
    pub zone: *mut GWeatherTimezone,
}

impl ::std::fmt::Debug for GWeatherTimezoneMenu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GWeatherTimezoneMenu @ {:p}", self))
         .field("parent", &self.parent)
         .finish()
    }
}

#[link(name = "gweather-3")]
extern "C" {

    //=========================================================================
    // GWeatherConditionPhenomenon
    //=========================================================================
    pub fn gweather_phenomenon_get_type() -> GType;

    //=========================================================================
    // GWeatherConditionQualifier
    //=========================================================================
    pub fn gweather_qualifier_get_type() -> GType;

    //=========================================================================
    // GWeatherDistanceUnit
    //=========================================================================
    pub fn gweather_distance_unit_get_type() -> GType;

    //=========================================================================
    // GWeatherLocationLevel
    //=========================================================================
    pub fn gweather_location_level_get_type() -> GType;
    pub fn gweather_location_level_to_string(level: GWeatherLocationLevel) -> *const c_char;

    //=========================================================================
    // GWeatherPressureUnit
    //=========================================================================
    pub fn gweather_pressure_unit_get_type() -> GType;

    //=========================================================================
    // GWeatherSky
    //=========================================================================
    pub fn gweather_sky_get_type() -> GType;
    pub fn gweather_sky_to_string(sky: GWeatherSky) -> *const c_char;
    pub fn gweather_sky_to_string_full(sky: GWeatherSky, options: GWeatherFormatOptions) -> *const c_char;

    //=========================================================================
    // GWeatherSpeedUnit
    //=========================================================================
    pub fn gweather_speed_unit_get_type() -> GType;
    pub fn gweather_speed_unit_to_string(unit: GWeatherSpeedUnit) -> *const c_char;

    //=========================================================================
    // GWeatherTemperatureUnit
    //=========================================================================
    pub fn gweather_temperature_unit_get_type() -> GType;
    pub fn gweather_temperature_unit_to_real(unit: GWeatherTemperatureUnit) -> GWeatherTemperatureUnit;

    //=========================================================================
    // GWeatherWindDirection
    //=========================================================================
    pub fn gweather_wind_direction_get_type() -> GType;
    pub fn gweather_wind_direction_to_string(wind: GWeatherWindDirection) -> *const c_char;
    pub fn gweather_wind_direction_to_string_full(wind: GWeatherWindDirection, options: GWeatherFormatOptions) -> *const c_char;

    //=========================================================================
    // GWeatherFormatOptions
    //=========================================================================
    pub fn gweather_format_options_get_type() -> GType;

    //=========================================================================
    // GWeatherProvider
    //=========================================================================
    pub fn gweather_provider_get_type() -> GType;

    //=========================================================================
    // GWeatherConditions
    //=========================================================================
    pub fn gweather_conditions_to_string(conditions: *mut GWeatherConditions) -> *const c_char;
    pub fn gweather_conditions_to_string_full(conditions: *mut GWeatherConditions, options: GWeatherFormatOptions) -> *const c_char;

    //=========================================================================
    // GWeatherLocation
    //=========================================================================
    pub fn gweather_location_get_type() -> GType;
    pub fn gweather_location_new_detached(name: *const c_char, icao: *const c_char, latitude: c_double, longitude: c_double) -> *mut GWeatherLocation;
    pub fn gweather_location_deserialize(world: *mut GWeatherLocation, serialized: *mut glib::GVariant) -> *mut GWeatherLocation;
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_12")))]
    pub fn gweather_location_detect_nearest_city(loc: *mut GWeatherLocation, lat: c_double, lon: c_double, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    pub fn gweather_location_equal(one: *mut GWeatherLocation, two: *mut GWeatherLocation) -> gboolean;
    pub fn gweather_location_find_by_country_code(world: *mut GWeatherLocation, country_code: *const c_char) -> *mut GWeatherLocation;
    pub fn gweather_location_find_by_station_code(world: *mut GWeatherLocation, station_code: *const c_char) -> *mut GWeatherLocation;
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_12")))]
    pub fn gweather_location_find_nearest_city(loc: *mut GWeatherLocation, lat: c_double, lon: c_double) -> *mut GWeatherLocation;
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_12")))]
    pub fn gweather_location_find_nearest_city_full(loc: *mut GWeatherLocation, lat: c_double, lon: c_double, func: GWeatherFilterFunc, user_data: gpointer, destroy: glib::GDestroyNotify) -> *mut GWeatherLocation;
    pub fn gweather_location_free_timezones(loc: *mut GWeatherLocation, zones: *mut *mut GWeatherTimezone);
    pub fn gweather_location_get_children(loc: *mut GWeatherLocation) -> *mut *mut GWeatherLocation;
    pub fn gweather_location_get_city_name(loc: *mut GWeatherLocation) -> *mut c_char;
    pub fn gweather_location_get_code(loc: *mut GWeatherLocation) -> *const c_char;
    pub fn gweather_location_get_coords(loc: *mut GWeatherLocation, latitude: *mut c_double, longitude: *mut c_double);
    pub fn gweather_location_get_country(loc: *mut GWeatherLocation) -> *const c_char;
    pub fn gweather_location_get_country_name(loc: *mut GWeatherLocation) -> *mut c_char;
    pub fn gweather_location_get_distance(loc: *mut GWeatherLocation, loc2: *mut GWeatherLocation) -> c_double;
    #[cfg(any(feature = "v3_36", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_36")))]
    pub fn gweather_location_get_english_name(loc: *mut GWeatherLocation) -> *const c_char;
    #[cfg(any(feature = "v3_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_38")))]
    pub fn gweather_location_get_english_sort_name(loc: *mut GWeatherLocation) -> *const c_char;
    pub fn gweather_location_get_level(loc: *mut GWeatherLocation) -> GWeatherLocationLevel;
    pub fn gweather_location_get_name(loc: *mut GWeatherLocation) -> *const c_char;
    pub fn gweather_location_get_parent(loc: *mut GWeatherLocation) -> *mut GWeatherLocation;
    pub fn gweather_location_get_sort_name(loc: *mut GWeatherLocation) -> *const c_char;
    pub fn gweather_location_get_timezone(loc: *mut GWeatherLocation) -> *mut GWeatherTimezone;
    pub fn gweather_location_get_timezone_str(loc: *mut GWeatherLocation) -> *const c_char;
    pub fn gweather_location_get_timezones(loc: *mut GWeatherLocation) -> *mut *mut GWeatherTimezone;
    pub fn gweather_location_has_coords(loc: *mut GWeatherLocation) -> gboolean;
    #[cfg(any(feature = "v40", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v40")))]
    pub fn gweather_location_next_child(loc: *mut GWeatherLocation, child: *mut GWeatherLocation) -> *mut GWeatherLocation;
    pub fn gweather_location_ref(loc: *mut GWeatherLocation) -> *mut GWeatherLocation;
    pub fn gweather_location_serialize(loc: *mut GWeatherLocation) -> *mut glib::GVariant;
    pub fn gweather_location_unref(loc: *mut GWeatherLocation);
    pub fn gweather_location_detect_nearest_city_finish(result: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> *mut GWeatherLocation;
    pub fn gweather_location_get_world() -> *mut GWeatherLocation;

    //=========================================================================
    // GWeatherTimezone
    //=========================================================================
    pub fn gweather_timezone_get_type() -> GType;
    pub fn gweather_timezone_get_dst_offset(zone: *mut GWeatherTimezone) -> c_int;
    pub fn gweather_timezone_get_name(zone: *mut GWeatherTimezone) -> *const c_char;
    pub fn gweather_timezone_get_offset(zone: *mut GWeatherTimezone) -> c_int;
    pub fn gweather_timezone_get_tzid(zone: *mut GWeatherTimezone) -> *const c_char;
    pub fn gweather_timezone_has_dst(zone: *mut GWeatherTimezone) -> gboolean;
    pub fn gweather_timezone_ref(zone: *mut GWeatherTimezone) -> *mut GWeatherTimezone;
    pub fn gweather_timezone_unref(zone: *mut GWeatherTimezone);
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_12")))]
    pub fn gweather_timezone_get_by_tzid(tzid: *const c_char) -> *mut GWeatherTimezone;
    pub fn gweather_timezone_get_utc() -> *mut GWeatherTimezone;

    //=========================================================================
    // GWeatherInfo
    //=========================================================================
    pub fn gweather_info_get_type() -> GType;
    pub fn gweather_info_new(location: *mut GWeatherLocation) -> *mut GWeatherInfo;
    pub fn gweather_info_store_cache();
    pub fn gweather_info_abort(info: *mut GWeatherInfo);
    pub fn gweather_info_get_apparent(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_application_id(info: *mut GWeatherInfo) -> *const c_char;
    pub fn gweather_info_get_attribution(info: *mut GWeatherInfo) -> *const c_char;
    pub fn gweather_info_get_conditions(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_contact_info(info: *mut GWeatherInfo) -> *const c_char;
    pub fn gweather_info_get_dew(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_enabled_providers(info: *mut GWeatherInfo) -> GWeatherProvider;
    pub fn gweather_info_get_forecast_list(info: *mut GWeatherInfo) -> *mut glib::GSList;
    pub fn gweather_info_get_humidity(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_icon_name(info: *mut GWeatherInfo) -> *const c_char;
    pub fn gweather_info_get_location(info: *mut GWeatherInfo) -> *const GWeatherLocation;
    pub fn gweather_info_get_location_name(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_pressure(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_radar(info: *mut GWeatherInfo) -> *mut gdk_pixbuf::GdkPixbufAnimation;
    pub fn gweather_info_get_sky(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_sunrise(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_sunset(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_symbolic_icon_name(info: *mut GWeatherInfo) -> *const c_char;
    pub fn gweather_info_get_temp(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_temp_max(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_temp_min(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_temp_summary(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_upcoming_moonphases(info: *mut GWeatherInfo, phases: *mut [c_ulong; 4]) -> gboolean;
    pub fn gweather_info_get_update(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_value_apparent(info: *mut GWeatherInfo, unit: GWeatherTemperatureUnit, value: *mut c_double) -> gboolean;
    pub fn gweather_info_get_value_conditions(info: *mut GWeatherInfo, phenomenon: *mut GWeatherConditionPhenomenon, qualifier: *mut GWeatherConditionQualifier) -> gboolean;
    pub fn gweather_info_get_value_dew(info: *mut GWeatherInfo, unit: GWeatherTemperatureUnit, value: *mut c_double) -> gboolean;
    pub fn gweather_info_get_value_moonphase(info: *mut GWeatherInfo, value: *mut GWeatherMoonPhase, lat: *mut GWeatherMoonLatitude) -> gboolean;
    pub fn gweather_info_get_value_pressure(info: *mut GWeatherInfo, unit: GWeatherPressureUnit, value: *mut c_double) -> gboolean;
    pub fn gweather_info_get_value_sky(info: *mut GWeatherInfo, sky: *mut GWeatherSky) -> gboolean;
    pub fn gweather_info_get_value_sunrise(info: *mut GWeatherInfo, value: *mut c_ulong) -> gboolean;
    pub fn gweather_info_get_value_sunset(info: *mut GWeatherInfo, value: *mut c_ulong) -> gboolean;
    pub fn gweather_info_get_value_temp(info: *mut GWeatherInfo, unit: GWeatherTemperatureUnit, value: *mut c_double) -> gboolean;
    pub fn gweather_info_get_value_temp_max(info: *mut GWeatherInfo, unit: GWeatherTemperatureUnit, value: *mut c_double) -> gboolean;
    pub fn gweather_info_get_value_temp_min(info: *mut GWeatherInfo, unit: GWeatherTemperatureUnit, value: *mut c_double) -> gboolean;
    pub fn gweather_info_get_value_update(info: *mut GWeatherInfo, value: *mut c_long) -> gboolean;
    pub fn gweather_info_get_value_visibility(info: *mut GWeatherInfo, unit: GWeatherDistanceUnit, value: *mut c_double) -> gboolean;
    pub fn gweather_info_get_value_wind(info: *mut GWeatherInfo, unit: GWeatherSpeedUnit, speed: *mut c_double, direction: *mut GWeatherWindDirection) -> gboolean;
    pub fn gweather_info_get_visibility(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_weather_summary(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_get_wind(info: *mut GWeatherInfo) -> *mut c_char;
    pub fn gweather_info_is_daytime(info: *mut GWeatherInfo) -> gboolean;
    pub fn gweather_info_is_valid(info: *mut GWeatherInfo) -> gboolean;
    pub fn gweather_info_network_error(info: *mut GWeatherInfo) -> gboolean;
    pub fn gweather_info_next_sun_event(info: *mut GWeatherInfo) -> c_int;
    pub fn gweather_info_set_application_id(info: *mut GWeatherInfo, application_id: *const c_char);
    pub fn gweather_info_set_contact_info(info: *mut GWeatherInfo, contact_info: *const c_char);
    pub fn gweather_info_set_enabled_providers(info: *mut GWeatherInfo, providers: GWeatherProvider);
    pub fn gweather_info_set_location(info: *mut GWeatherInfo, location: *mut GWeatherLocation);
    pub fn gweather_info_update(info: *mut GWeatherInfo);

    //=========================================================================
    // GWeatherLocationEntry
    //=========================================================================
    pub fn gweather_location_entry_get_type() -> GType;
    pub fn gweather_location_entry_new(top: *mut GWeatherLocation) -> *mut gtk::GtkWidget;
    pub fn gweather_location_entry_get_location(entry: *mut GWeatherLocationEntry) -> *mut GWeatherLocation;
    pub fn gweather_location_entry_has_custom_text(entry: *mut GWeatherLocationEntry) -> gboolean;
    pub fn gweather_location_entry_set_city(entry: *mut GWeatherLocationEntry, city_name: *const c_char, code: *const c_char) -> gboolean;
    pub fn gweather_location_entry_set_location(entry: *mut GWeatherLocationEntry, loc: *mut GWeatherLocation);

    //=========================================================================
    // GWeatherTimezoneMenu
    //=========================================================================
    pub fn gweather_timezone_menu_get_type() -> GType;
    pub fn gweather_timezone_menu_new(top: *mut GWeatherLocation) -> *mut gtk::GtkWidget;
    pub fn gweather_timezone_menu_get_tzid(menu: *mut GWeatherTimezoneMenu) -> *const c_char;
    pub fn gweather_timezone_menu_set_tzid(menu: *mut GWeatherTimezoneMenu, tzid: *const c_char);

}
