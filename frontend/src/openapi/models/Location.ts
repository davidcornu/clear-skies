/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { CanadaTz } from './CanadaTz';
import type { Coordinates } from './Coordinates';
import type { ProvinceOrTerritory } from './ProvinceOrTerritory';

/**
 * A location for which weather data is available
 */
export type Location = {
    /**
     * The latitude and longitude for the center of this location
     */
    coordinates: Coordinates;
    /**
     * The URL for the Environment Canada RSS feed for this location
     */
    feed_url: string;
    /**
     * The Environment Canada web page for this location
     */
    html_url: string;
    /**
     * The location name as listed by Environment Canada
     */
    name: string;
    /**
     * The path to retrieve the weather for this location
     */
    path: string;
    /**
     * The province or territory where the location is
     */
    province_or_territory: ProvinceOrTerritory;
    /**
     * The URL [slug](https://developer.mozilla.org/en-US/docs/Glossary/Slug) for this location. Location slugs are unique by province/territory.
     */
    slug: string;
    /**
     * The location's local timezone
     */
    tz: CanadaTz;
};

