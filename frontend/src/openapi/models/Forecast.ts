/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Condition } from './Condition';
import type { LocalDateTime } from './LocalDateTime';
import type { Temperature } from './Temperature';

export type Forecast = {
    /**
     * A short description of forecasted weather condition (e.g. "Chance of showers")
     */
    condition: string;
    /**
     * When the forecast was issued
     */
    issued_at: LocalDateTime;
    /**
     * Best-effort attempt to convert the provided conditions into something that can be used to show an icon.
     */
    normalized_condition?: Array<Condition> | null;
    /**
     * The [probability of precipitation](https://www.canada.ca/en/environment-climate-change/services/sky-watchers/glossary.html#wsDTE9CAF366) as a percentage
     */
    probability_of_precipitation?: number | null;
    /**
     * A longer description of forecasted weather conditions (e.g. "Cloudy with 70 percent chance of showers. Low 17.")
     */
    summary: string;
    /**
     * The forecasted temperature
     */
    temperature: Temperature;
};

