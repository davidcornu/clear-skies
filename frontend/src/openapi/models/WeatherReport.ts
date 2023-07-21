/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { CurrentConditions } from './CurrentConditions';
import type { Day } from './Day';
import type { SpecialWeatherStatement } from './SpecialWeatherStatement';

export type WeatherReport = {
    current_conditions?: CurrentConditions | null;
    special_weather_statements: Array<SpecialWeatherStatement>;
    title: string;
    updated: string;
    url: string;
    weather_forecasts: Array<Day>;
};

