/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { PressureTendency } from './PressureTendency';

/**
 * [Atmospheric pressure](https://climate.weather.gc.ca/glossary_e.html#AB_pressure)
 */
export type Pressure = {
    kpa: number;
    tendency?: PressureTendency | null;
};

