/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Condition } from './Condition';
import type { ObservedAt } from './ObservedAt';
import type { Pressure } from './Pressure';
import type { Wind } from './Wind';

/**
 * Current weather conditions
 */
export type CurrentConditions = {
    /**
     * [Air quality health index](https://www.canada.ca/en/environment-climate-change/services/air-quality-health-index/about.html)
     */
    aqhi?: number | null;
    /**
     * Description of current conditions (e.g. "Light Rainshower")
     */
    condition?: string | null;
    /**
     * [Dewpoint](https://climate.weather.gc.ca/glossary_e.html#dewPnt) in degrees Celsius
     */
    dewpoint_c?: number | null;
    /**
     * [Humidex](https://climate.weather.gc.ca/glossary_e.html#humidex) in degrees Celsius
     */
    humidex_c?: number | null;
    /**
     * [Relative humidity](https://climate.weather.gc.ca/glossary_e.html#r_humidity) as a percentage
     */
    humidity_pct?: number | null;
    /**
     * Best-effort attempt to convert the provided conditions into something that can be used to show an icon.
     */
    normalized_condition?: Array<Condition> | null;
    /**
     * Time and place where conditions were observed
     */
    observed_at: ObservedAt;
    /**
     * [Atmospheric pressure](https://climate.weather.gc.ca/glossary_e.html#AB_pressure)
     */
    pressure?: Pressure | null;
    /**
     * Temperature in degrees Celsius
     */
    temperature_c?: number | null;
    /**
     * [Visibility](https://climate.weather.gc.ca/glossary_e.html#visibility) in kilometers
     */
    visibility_km?: number | null;
    /**
     * Wind speed and direction
     */
    wind?: Wind | null;
    /**
     * [Wind chill](https://climate.weather.gc.ca/glossary_e.html#windChill) in degrees Celsius
     */
    wind_chill_c?: number | null;
};

